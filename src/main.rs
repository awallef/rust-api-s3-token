#[macro_use]
extern crate diesel;
extern crate dotenv;

mod datasource;
mod model;

use datasource::establish_connection;
use model::table::accounts;

fn main() {
    let connection = establish_connection();
    let results = accounts::all(&connection);
    match results {
        Ok(accounts_vector) => {
            println!("Displaying {} accounts", accounts_vector.len());
            for account in accounts_vector {
                println!("{}", account.id);
                println!("-----------\n");
                println!("{}", account.name);
            }
        },
        Err(err) => {
            println!("Error: {}", err);
        },
    }


}
