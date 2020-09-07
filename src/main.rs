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
        Ok(accountsVector) => {
            println!("Displaying {} accounts", accountsVector.len());
            for account in accountsVector {
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
