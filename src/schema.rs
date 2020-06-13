use diesel::Table;

table! {
    ledger (id) {
        id -> Text,
        name -> Text,
        description -> Text,
    }
}