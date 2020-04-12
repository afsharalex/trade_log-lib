extern crate diesel;
extern crate trade_log_lib;

use self::trade_log_lib::*;
use std::io::{stdin, Read};
use std::ops::Deref;

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";

fn main() {
    let conn = establish_connection();

    // Get Ticker
    println!("Please enter Ticker for stock symbol (max 4 characters)");
    let mut ticker = String::new();
    stdin().read_line(&mut ticker).unwrap();
    let ticker = &ticker[..4]; // Ticker max is 4 characters

    // Get name
    println!("\nPlease enter Name for stock symbol (max 255 characters)");
    println!("Press {} when finished\n", EOF);
    let mut name = String::new();
    stdin().read_to_string(&mut name).unwrap();

    // Get Description
    println!("\nPlease enter Description for stock symbol");
    println!("Press {} when finished\n", EOF);
    let mut description = String::new();
    stdin().read_to_string(&mut description).unwrap();

    let stock = create_stock(
        &conn,
        ticker,
        Option::from(name.deref()),        // Can not pass String, need str
        Option::from(description.deref()), // Also, need to wrap into an Option<T>
    );

    println!("Stock symbol saved: ");
    println!("------------------\n\n");
    println!("{}", stock.ticker);
    if let Some(name) = stock.name {
        println!("\n------------------\n");
        println!("{}", name);
    }
    if let Some(description) = stock.description {
        println!("\n------------------\n");
        println!("{}", description);
    }
    println!("------------------\n\n");
}
