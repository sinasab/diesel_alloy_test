use std::fmt::{Display, Formatter};

use diesel::{expression::ValidGrouping, prelude::*};
use ruint::aliases::U256;

#[derive(Queryable, Selectable, Insertable, AsChangeset, Debug, ValidGrouping)]
#[diesel(check_for_backend(diesel::pg::Pg), table_name = crate::schema::users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub favorite_number: U256,
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "User {{ id: {}, name: {}, favorite_number: {} }}",
            self.id, self.name, self.favorite_number
        )
    }
}
