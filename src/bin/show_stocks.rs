extern crate diesel;
extern crate trade_log_lib;

use self::diesel::prelude::*;
use self::models::*;
use self::trade_log_lib::*;

fn main() {
    use trade_log_lib::schema::stocks::dsl::*; // Allows calling stocks instead of stocks::table, etc

    // Get a connection
    let conn = establish_connection();

    // Load all stocks
    let results = stocks.load::<Stock>(&conn).expect("Error loading stocks");

    if results.is_empty() {
        println!("There are no stock symbols yet.");
    } else {
        println!("Displaying {} stock symbols", results.len());
    }

    for stock in results {
        println!("{}", stock.ticker);

        // Both name and description are Option<T>, so we need to check
        // if they are not None.
        if let Some(stock_name) = stock.name {
            // We can't simply use `name` here as we are using dsl and
            // `name` is a field on `stocks` table.
            println!("-----------------\n");
            println!("{}", stock_name);
        }
        if let Some(stock_desc) = stock.description {
            // Again, same reason for not using `description` as `name`.
            println!("-----------------\n");
            println!("{}", stock_desc);
        }
        println!("-----------------\n"); // End of printing Stock
        println!("-----------------\n");
    }
}
