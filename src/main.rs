#[macro_use]
extern crate diesel;
#[macro_use]
extern crate juniper;

use juniper::{Variables, EmptyMutation};

pub mod db;
pub mod gql;

fn main() {
    use db::db_controller::*;
    use gql::gql_resolver;

    let context = gql_resolver::Context {
        controller: DBController::get_db_controller()
    };

    let result = juniper::execute(
        r#"
        query
        { ledger(id: "abcdef") {
            name
            description
            }
        }"#,
        None,
        &gql_resolver::Schema::new(gql_resolver::Query, gql_resolver::Mutation),
        &Variables::new(),
        &context
    );

    let result = context.controller.get_ledger("abcdef");

    println!("Program finished!");

    // println!("Result = {}", result)


    // let connection = establish_connection();
    // create_ledger(&connection, "New Ledger", "A beautiful ledger, for beautiful you");
}