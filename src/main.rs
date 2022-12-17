#[macro_use]
extern crate diesel;

mod models;
mod routes;
mod schema;
mod solana;

use rocket::routes;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::solana::{get_accounts_and_update, subscribe_to_program};
use crate::routes::{get_all_stream, index};


// Load .env and connect to sqlclient db via url.
pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    get_accounts_and_update();
    subscribe_to_program();

    let cors = rocket_cors::CorsOptions::default().to_cors()?;

    rocket::build()
        .mount("/", routes![index, get_all_stream])
        .attach(cors)
        .launch()
        .await?;

    Ok(())
}
