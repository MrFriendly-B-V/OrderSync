use actix_web::{get, web, HttpResponse, HttpRequest};
use crate::appdata::AppData;
use mysql::prelude::Queryable;
use mysql::{Row, Params, params};
use serde::{Serialize, Deserialize};
use crate::environment::get_environment;
use tera::Context;

const WIX_TOKEN_URI: &str = "https://www.wix.com/oauth/access";
const WIX_FINISH_FLOW_URI: &str = "https://www.wix.com/_api/site-apps/v1/site-apps/token-received";

#[derive(Serialize)]
struct ExchangeAccessToken {
    grant_type:     String,
    client_id:      String,
    client_secret:  String,
    code:           String
}

#[derive(Deserialize)]
struct ExchangeResponse {
    refresh_token:  String,
    access_token:   String
}

#[get("/wix/grant")]
pub async fn get_grant(data: web::Data<AppData>, req: HttpRequest) -> HttpResponse {
    let qstring = qstring::QString::from(req.query_string());

    let code_param = qstring.get("code");
    if code_param.is_none() {
        return HttpResponse::BadRequest().json("Missing required parameter 'code'");
    }

    let state_param = qstring.get("state");
    if code_param.is_none() {
        return HttpResponse::BadRequest().json("Missing required parameter 'state'");
    }

    let instance_id_param = qstring.get("instanceId");
    if instance_id_param.is_none() {
        return HttpResponse::BadRequest().json("Missing required parameter 'instanceId'");
    }

    //Verify the 'state' param
    let mut conn = data.database.pool.get_conn().unwrap();
    let sql_result = conn.exec::<Row, &str, Params>("SELECT state FROM states WHERE state = :state", params! {
        "state" => state_param.unwrap().clone()
    });

    if sql_result.is_err() {
        return HttpResponse::InternalServerError().json("Something went wrong while processing your request. Please try again later.");
    }

    let rows = sql_result.unwrap();
    let mut state_exists = false;
    for row in rows {
        let row_state = row.get::<String, &str>("state");

        if row_state.is_none() {
            continue;
        }

        if row_state.unwrap() == state_param.unwrap().clone() {
           state_exists = true;
            break;
        }
    }

    //Check the state_exists var
    if !state_exists {
        return HttpResponse::Unauthorized().json("Parameter 'state' is invalid");
    }

    //We've now validated that the state parameter is valid, we can assume that this is a genuine authentic request
    //We now need to send a POST request to Wix to get a refresh and access token
    let env = get_environment();
    let exchange_access_token = ExchangeAccessToken {
        grant_type:     "authorization_code".to_string(),
        client_id:      env.wix_app_id,
        client_secret:  env.wix_app_secret,
        code:           code_param.unwrap().to_string()
    };

    let access_token_response = reqwest::blocking::Client::new().post(WIX_TOKEN_URI).json(&serde_json::to_string(&exchange_access_token).unwrap()).send();

    if access_token_response.is_err() {
        return HttpResponse::InternalServerError().body(access_token_response.err().unwrap().to_string());
    }

    let unwrapped_access_token_response = access_token_response.unwrap();
    let exchange_response: ExchangeResponse = unwrapped_access_token_response.json().unwrap();

    //Insert the data into the database
    let _ = conn.exec::<usize, &str, Params>("INSERT INTO wix_grants (instance_id, access_token, refresh_token) VALUES (:instance_id, :access_token, :refresh_token)", params!{
            "instance_id" => instance_id_param.unwrap(),
            "access_token" => exchange_response.access_token.clone(),
            "refresh_token" => exchange_response.refresh_token
    }).expect("Database error");

    //Now send a POST request to wix indicating that our flow is done, and redirect the user to the dashboard
    let _ = reqwest::Client::new().post(WIX_FINISH_FLOW_URI).header("Authorization", exchange_response.access_token).send();

    //Redirect the user to our dashboard
    let dashboard_uri = format!("{frontend_host}/static/dashboard.html", frontend_host = env.frontend_host);

    let mut ctx = Context::new();
    ctx.insert("redirect_uri", &dashboard_uri);
    let rendered = data.tera.render("wix/grant.html", &ctx).unwrap();

    HttpResponse::Ok().body(rendered)
}