use alloy_primitives::{Address, PrimitiveSignature, B256};
use diesel::{expression::ValidGrouping, prelude::*};
use std::fmt::{Display, Formatter};

#[derive(Queryable, Selectable, Insertable, AsChangeset, Debug, ValidGrouping)]
#[diesel(check_for_backend(diesel::pg::Pg), table_name = crate::schema::users)]
pub struct User {
    pub addr: Address,
    pub hash: B256,
    pub sig: PrimitiveSignature,
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "User {{ addr: {}, sig: {}, hash: {} }}",
            self.addr, self.sig, self.hash
        )
    }
}
