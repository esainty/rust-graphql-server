use dotenv::dotenv;
use std::env;
use diesel::prelude::Connection;
use diesel::sqlite::*;
use super::db_model::*;
use super::db_schema::*;

pub struct DBController {
    connection: SqliteConnection,
}

impl DBController {
    fn get_db_controller() -> DBController {
        DBController {
            connection: establish_connection(),
        }
    }

    fn establish_connection() -> SqliteConnection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
        SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
    }

    fn create_ledger<'a>(connection: &SqliteConnection, name: &'a str, description: &'a str) -> usize {
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
}
