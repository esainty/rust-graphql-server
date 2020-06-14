use juniper::{FieldResult};

#[derive(GraphQLObject)]
#[graphql(description="Top-Level Ledger Object")]
pub struct Ledger {
    id: String,
    name: String,
    description: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description="Top-Level Ledger Object")]
pub struct LedgerInput {
    name: String,
    description: String,
}
