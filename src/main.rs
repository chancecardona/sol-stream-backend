#[macro_use]
extern crate diesel;

mod models;
mod models::Stream;
mod routes::{get_all_stream, index};
mod schema;
mod solana::{get_all_program_accounts, subscribe_to_program};
use rocket::{get, routes};

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

// Load .env and connect to sqlclient db via url.
pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cors = rocket_cors::CorsOptions::default().to_cors()?;

    rocket::build()
        .mount("/", routes![index,route_with_pubkey])
        .attach(cors)
        .launch()
        .await?;

    Ok(())
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<pubkey>")]
fn route_with_pubkey(pubkey: &str)-> String{
    format!("Hello {}",pubkey)
}
