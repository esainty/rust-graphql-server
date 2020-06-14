#[macro_use]
extern crate diesel;
#[macro_use]
extern crate juniper;

pub mod db;
pub mod gql;

fn main() {
    use db::db_controller::*;

    let connection = establish_connection();
    create_ledger(&connection, "New Ledger", "A beautiful ledger, for beautiful you");
}