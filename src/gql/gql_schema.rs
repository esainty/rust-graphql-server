use juniper::{FieldResult};

#[derive(GraphQLObject)]
#[graphql(description="Top-Level Ledger Object")]
pub struct Ledger {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description="Top-Level Ledger Object")]
pub struct LedgerInput {
    pub name: String,
    pub description: String,
}
