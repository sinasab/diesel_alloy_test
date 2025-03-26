use alloy_primitives::{Address, PrimitiveSignature, B256};
use diesel::{expression::ValidGrouping, prelude::*};

#[derive(Queryable, Selectable, Insertable, AsChangeset, Debug, ValidGrouping)]
#[diesel(check_for_backend(diesel::pg::Pg), table_name = crate::schema::users)]
pub struct User {
    pub addr: Address,
    pub hash: B256,
    pub sig: PrimitiveSignature,
    pub maybe_hash: Option<B256>,
}
