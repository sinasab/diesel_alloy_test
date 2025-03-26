diesel::table! {
    users (addr) {
        addr -> Bytea,
        hash -> Bytea,
        sig -> Bytea,
        maybe_hash -> Nullable<Bytea>,
    }
}
