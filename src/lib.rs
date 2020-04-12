#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use self::models::{NewStock, Stock};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_stock<'a>(
    conn: &PgConnection,
    ticker: &'a str,
    name: Option<&'a str>,
    description: Option<&'a str>,
) -> Stock {
    use schema::stocks;

    // If the ticker is greater than 4 characters,
    // let's just panic for now.
    if ticker.len() > 4 {
        panic!("Ticker must be 4 characters or less!");
    }

    // No need to match on Some or None for Option<T> values
    // as Diesel will take care of this for us.
    let new_stock = NewStock {
        ticker,
        name,
        description,
    };

    // Diesel will supply DEFAULT values for and None
    // if the field is marked NULLABLE
    diesel::insert_into(stocks::table)
        .values(&new_stock)
        .get_result(conn)
        .expect("Error saving new stock")
}
