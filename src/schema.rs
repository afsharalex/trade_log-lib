table! {
    stocks (id) {
        id -> Int4,
        ticker -> Varchar,
        name -> Nullable<Varchar>,
        description -> Nullable<Text>,
    }
}
