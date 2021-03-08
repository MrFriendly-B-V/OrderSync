use actix_web::{get, web, HttpResponse, HttpRequest};
use crate::appdata::AppData;
use tera::Context;
use crate::environment::get_environment;
use rand::Rng;
use mysql::{Params, params};
use mysql::prelude::Queryable;

const WIX_REDIRECT_URI: &str = "https://www.wix.com/installer/install";

#[get("/wix/install")]
pub async fn get_install(data: web::Data<AppData>, req: HttpRequest) -> HttpResponse {
    let qstring = qstring::QString::from(req.query_string());

    //token parameter
    let token_param = qstring.get("token");
    if token_param.is_none() {
        return HttpResponse::BadRequest().json("Missing required parameter 'token'");
    }

    //Generate a random 64 char state
    let state_gen: String = rand::thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(64).map(char::from).collect();

    //Insert the state into the database
    let mut conn = data.database.pool.get_conn().unwrap();
    conn.exec::<usize, &str, Params>("INSERT INTO states (state) VALUES (:state)", params!{
        "state" => state_gen.clone()
    }).expect("Database error");

    //Build the redirect uri
    let env = get_environment();
    let redirect_uri = format!("{wix_redirect_uri}?token={token}&appId={app_id}&redirectUrl={redirect_url}&state={state}",
                               wix_redirect_uri = WIX_REDIRECT_URI,
                               token = token_param.unwrap().clone(),
                               app_id = env.wix_app_id,
                               redirect_url = format!("{}/wix/grant", env.api_host),
                               state = state_gen
    );
    
    let mut ctx = Context::new();
    ctx.insert("redirect_uri", &redirect_uri);
    let rendered = data.tera.render("wix/install.html", &ctx).unwrap();

    HttpResponse::Ok().body(rendered)
}