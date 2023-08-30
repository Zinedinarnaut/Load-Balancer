use actix_web::{get, web, Responder, HttpResponse};
use super::load_balancer::LoadBalancer;
use super::discord::{discord_notify, Handler};
use super::health_check;
use std::sync::{Arc, Mutex};
use rand::Rng;
use reqwest::Client;
use serenity::{
    async_trait,
    http::Http,
    model::{gateway::Activity, id::ChannelId},
    prelude::*,
};
use log::{error, info};

async fn health_check(server: &str) -> bool {
    match reqwest::get(server).await {
        Ok(response) => response.status().is_success(),
        Err(_) => false,
    }
}

#[get("/")]
async fn index(
    lb: web::Data<LoadBalancer>,
    discord_channel: web::Data<ChannelId>,
) -> impl Responder {
    let algorithm = LoadBalancingAlgorithm::RoundRobin;

    if let Some(server) = lb.get_server(algorithm) {
        if health_check(&server).await {
            info!("Request routed to: {}", server);
            return HttpResponse::Ok().body(format!("Request routed to: {}", server));
        } else {
            lb.update_failure_count(&server, true);
            error!("Failed health check for server: {}", server);
            if lb.is_server_unhealthy(&server) {
                let message = format!("Server {} is down!", server);
                discord_notify(*discord_channel, message).await;
            }
        }
    }

    HttpResponse::InternalServerError().body("No healthy servers available")
}
