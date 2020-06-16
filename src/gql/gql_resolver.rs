use juniper::{FieldResult};
use crate::db::accounting_controller;
use crate::db::db_controller;
use super::gql_schema;

pub struct Context {
    pub controller: db_controller::DBController,
}

impl juniper::Context for Context {}

pub struct Query;

#[juniper::object(Context = Context)]
impl Query {
    fn apiVersion() -> &str {
        "1.0"
    }

    pub fn ledger(context: &Context, id: String) -> FieldResult<gql_schema::Ledger> {
        Ok(context.controller.get_ledger(&id))
    }
}

pub struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {
    // fn create_ledger(context: &Context, name: &str, description: &str) -> &str {
    //     context.controller.create_ledger("Fat Ledger", "A Meaty Ledger, for Meaty You.");
    //     "Success"
    // }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation>;