mod functions;

use actix_web::{web, App, HttpServer};
use functions::{
    actix_web_module::{health_check, index},
    discord::{discord_notify, Handler},
    load_balancer::LoadBalancer, // Adjust the module path to match your actual module structure
	  data::configure as data_configure ; // Import the configure function
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
use sqlx::PgPool; // Import SQLx for PostgreSQL

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info"); // Set log level to info
    env_logger::init();

    let lb = LoadBalancer::new(3); // Set the maximum allowed failures

    // Replace this with your Discord channel ID
    let discord_channel = ChannelId(123456789012345678);

    // Set up your Prisma client and SQLx connection pool
    let prisma = prisma::PrismaClient::new();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::new(&database_url).await.expect("Failed to create pool");

    info!("Starting load balancer...");

    HttpServer::new(move || {
        App::new()
			      .configure(data_configure)
            .data(lb.clone())
            .data(discord_channel)
            .data(prisma.clone()) // Pass the Prisma client to the Actix app
            .data(pool.clone())   // Pass the SQLx pool to the Actix app
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
}
