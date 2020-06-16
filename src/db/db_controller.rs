use dotenv::dotenv;
use std::env;
use diesel::prelude::Connection;
use diesel::sqlite::*;
use super::db_model::*;
use super::db_schema::*;
use crate::gql::gql_schema::Ledger as GQLLedger;

pub struct DBController {
    connection: SqliteConnection,
}

impl DBController {
    pub fn get_db_controller() -> DBController {
        DBController {
            connection: establish_connection(),
        }
    }

    pub fn get_ledger<'a>(&self, id: &str) -> GQLLedger {
        use diesel::prelude::*;
        use super::db_schema::ledger::dsl::*;


        let mut ledgers = ledger.filter(id.eq(id)).load::<Ledger>(&self.connection).expect("Error accessing database");
        if ledgers.len() < 1 {
            panic!("Database did not contain requested ledger");
        } else if ledgers.len() > 1 {
            panic!("Database contains more than one ledger with the given id")
        } else {
            let new_ledger = ledgers.remove(0);
            GQLLedger {
                id: new_ledger.id,
                name: new_ledger.name,
                description: new_ledger.description
            }
        }

    }

    pub fn create_ledger<'a>(&self, name: &'a str, description: &'a str) -> usize {
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
            .execute(&self.connection)
            .expect("Error saving new ledger")
    }
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
