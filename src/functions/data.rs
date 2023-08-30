use actix_web::{web, HttpResponse, Responder, post};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Data {
    key1: String,
    key2: String,
}

#[post("/data")]
async fn receive_data(data: web::Json<Data>) -> impl Responder {
    // Process the received data here
    let response = format!("Received data: {:?}", data);
    HttpResponse::Ok().body(response)
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(receive_data);
}