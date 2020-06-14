use juniper::{FieldResult};
use crate::db::accounting_controller;
use super::gql_schema;

struct Context {
    database: db_controller::DBController,
}

impl juniper::Context for Context {}

struct Query;

#[juniper::object(Context = Context)]
impl Query {
    fn apiVersion() -> &str {
        "1.0"
    }

    fn ledger(context: &Context, id: String) -> FieldResult<gql_schema::Ledger> {
        let connection = context.database.con
    }
}