pub mod model;
pub mod schema;

// use rusqlite::{Connection, Result};
// use rusqlite::NO_PARAMS;

use model::*;
use diesel::prelude::Connection as OtherConnection;
use diesel::RunQueryDsl;
use diesel::sqlite::*;

use dotenv::dotenv;
use std::env;

use model::NewLedger;

#[macro_use]
extern crate diesel;
extern crate dotenv;

fn main() {
    use self::schema::ledger::dsl::*;

    let connection = establish_connection();
    let size = create_ledger(&connection, "New Ledger", "A beautiful ledger, for beautiful you");

    // let connection = connect("dev.db");
    // let table = "Ledger";
    // let columns = vec!("id", "name", "description");
    // let values = vec!("091823asd", "GL2", "Something or other");
    // add_to_table(connection, table, columns, values);
}

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn create_ledger<'a>(connection: &SqliteConnection, name: &'a str, description: &'a str) -> usize {
    use schema::ledger;
    use diesel::prelude::*;
    use sha2::{Sha256, Digest};
    use std::time::SystemTime;

    let new_ledger = NewLedger {
        id: {
            // Generate hash from input components + system time
            let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_micros().to_string();
            let hash = &Sha256::new()
                .chain(name)
                .chain(description)
                .chain(time)
                .finalize();
            // Format as hexadecimal
            &format!("{:x}", hash)
        },
        name,
        description,
    };

    diesel::insert_into(ledger::table)
        .values(&new_ledger)
        .execute(connection)
        .expect("Error saving new ledger")
}

// fn connect(path: &str) -> Connection {
//     Connection::open(path).unwrap()
// }
//
// fn add_to_table(connection: Connection, table: &str, columns: Vec<&str>, values: Vec<&str>) {
//     // let mut query = format!("INSERT INTO {}", table);
//     // for column in columns {
//     //     query = format!("{} ")
//     // }
//     // for value in values {
//     //     query = format!("{} {}", query, value);
//     // }
//
//     let query = r#"INSERT INTO Ledger (id, name, description) VALUES ("kajhsd", "GL", "Some Ledger")"#;
//     connection.execute(&query, NO_PARAMS).unwrap();
// }
