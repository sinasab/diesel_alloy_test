diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        favorite_number -> Bytea,
    }
}
