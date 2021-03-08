use mysql::prelude::Queryable;
use mysql::{Params, Row, params};
use serde::{Serialize, Deserialize};

use crate::database::Database;
use crate::environment::get_environment;

const WIX_REFRESH_URI: &str = "https://www.wix.com/oauth/access";

#[derive(Serialize)]
struct RefreshRequest {
    grant_type:     String,
    client_id:      String,
    client_secret:  String,
    refresh_token:  String
}

#[derive(Deserialize)]
struct RefreshResponse {
    refresh_token:  String,
    access_token:   String
}

/**
Get a Wix access_token for a Website

## Params
    **database** Instance of a Database object
    **instance_id** The instance ID to get the access_token for

## Returns
    **Ok**: The requested access token
    **Err*: A summary of what went wrong
*/
pub fn get_access_token(database: Database, instance_id: String) -> Result<String, String> {
    //Get the refresh token from the database
    let mut conn = database.pool.get_conn().unwrap();
    let result = conn.exec::<Row, &str, Params>("SELECT refresh_token FROM wix_grants WHERE instance_id = :instance_id", params! {
        "instance_id" => instance_id.clone()
    });

    if result.is_err() {
        return Err(result.err.unwrap().to_string);
    }

    let rows = result.unwrap();
    let mut refresh_token: Option<String>;
    for i in 0..rows.len() -1 {
        let row_refresh_token = rows.get(i).unwrap().get::<String, &str>("refresh_token");

        if row_refresh_token.is_none() && i != (rows.len() - 1) {
            continue;
        } else {
            refresh_token = row_refresh_token;
        }
    };

    if refresh_token.is_none() {
        return Err(format!("No refresh_token for instance {}", instance_id));
    }

    //Build the request payload
    let env = get_environment();
    let request_payload = RefreshRequest {
        refresh_token:  refresh_token.unwrap(),
        grant_type:     "refresh_token".to_string(),
        client_id:      env.wix_app_id,
        client_secret:  env.wix_app_secret
    };

    //Exchange the refresh_token for an access_token
    let result = reqwest::blocking::Client::new().post(WIX_REFRESH_URI).body(serde_json::to_string(&request_payload).unwrap()).send();
    if result.is_err() {
        return Err(result.err().unwrap().to_string());
    }

    let request_response: RefreshResponse = result.unwrap().json().unwrap();

    //Insert the new data into the database
    let _ = conn.exec::<usize, &str, Params>("UPDATE wix_grants SET access_token = :access_token, refresh_token = :refresh_token WHERE instance_id = :instance_id", params! {
        "access_token" = request_response.access_token.clone(),
        "refresh_token" = request_response.refresh_token,
        "instance_id" = instance_id
    }).expect("Database error occurred.");

    return Ok(request_response.access_token);
}