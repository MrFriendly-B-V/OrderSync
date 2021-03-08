use actix_web::{web, HttpResponse, post};

use crate::appdata::AppData;
use crate::types::wix::{OrderCreatedJwtData, OrderData};

#[post("/wix/webhooks/order_created")]
pub async fn post_order_created(data: web::Data<AppData>, bytes: web::Bytes) -> HttpResponse {
    let body = String::from_utf8(bytes.to_vec());
    if body.is_err() {
        return HttpResponse::InternalServerError().body(body.err().unwrap().to_string());
    }

    let body_unwrapped = body.unwrap();

    //Its a JWT, so split on . and get the second element, which is the payload
    let jwt_parts: Vec<&str> = body_unwrapped.split(".").collect();
    let jwt_payload_base64 = jwt_parts.get(1).unwrap();

    //Convert from Base64 to UTF8 and deserialize
    let jwt_data: OrderCreatedJwtData = serde_json::from_str(
        &String::from_utf8(base64::decode(jwt_payload_base64.as_bytes()).unwrap()).unwrap()
    ).unwrap();

    //We're interested in the data part
    //For some reason Wix has the data as json, as a string. So we need to un-escape \" to just " so we can deserialize it
    let data_fixed = jwt_data.data.replace("\\", "");

    //We can now deserialize the payload data into an OrderData object
    let order_data: OrderData = serde_json::from_str(&data_fixed).unwrap();

    //TODO fetch order from Wix and insert into the database
    //Requires response to my ticket about missing the instanceId parameter
    //Besides that Wix' documentation doesn't seem fully accurate, not sure what to do about that.

    HttpResponse::Ok().finish()
}