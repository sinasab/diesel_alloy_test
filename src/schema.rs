diesel::table! {
    users (addr) {
        addr -> Bytea,
        hash -> Bytea,
        sig -> Bytea,
    }
}
