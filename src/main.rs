 mod functions;

use actix_web::{web, App, HttpServer};
use functions::{
    actix_web_module::{health_check, index},
    discord::{discord_notify, Handler},
    load_balancer::LoadBalancer,
};
use log::{error, info};
use rand::Rng;
use reqwest::Client;
use serenity::{
    async_trait,
    http::Http,
    model::{gateway::Activity, id::ChannelId},
    prelude::*,
};
use std::sync::{Arc, Mutex};

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info"); // Set log level to info
    env_logger::init();

    let lb = LoadBalancer::new(3); // Set the maximum allowed failures

    // Replace this with your Discord channel ID
    let discord_channel = ChannelId(123456789012345678);

    info!("Starting load balancer...");

    HttpServer::new(move || {
        App::new()
            .data(lb.clone())
            .data(discord_channel)
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
}
