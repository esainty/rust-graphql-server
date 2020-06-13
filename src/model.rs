use super::schema::ledger;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Ledger {
    pub id: String,
    pub name: String,
    pub description: String 
}

#[derive(Insertable)]
#[table_name = "ledger"]
pub struct NewLedger<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub description: &'a str
}