use self::models::User;
use diesel::{pg::Pg, prelude::*};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use ruint::aliases::U256;
use schema::users;
use std::{env, error::Error};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn establish_connection() -> PgConnection {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn run_migrations(
    connection: &mut impl MigrationHarness<Pg>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}

pub fn seed_data(
    connection: &mut PgConnection,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let seed_users = &[
        User {
            id: 1,
            name: "Alice".to_string(),
            favorite_number: U256::MAX,
        },
        User {
            id: 2,
            name: "Bob".to_string(),
            favorite_number: U256::ZERO,
        },
        User {
            id: 3,
            name: "Carol".to_string(),
            favorite_number: U256::from(3),
        },
    ];

    for user in seed_users {
        upsert_user(connection, user)?;
    }
    Ok(())
}

fn upsert_user(
    connection: &mut PgConnection,
    user: &User,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    diesel::insert_into(users::table)
        .values(user)
        .on_conflict(users::id)
        .do_update()
        .set(user)
        .execute(connection)?;
    Ok(())
}

pub fn get_users(
    connection: &mut PgConnection,
) -> Result<Vec<User>, Box<dyn Error + Send + Sync + 'static>> {
    let users = users::table
        .select(User::as_select())
        .load::<User>(connection)?;
    Ok(users)
}

pub fn get_users_with_filter(
    connection: &mut PgConnection,
) -> Result<Vec<User>, Box<dyn Error + Send + Sync + 'static>> {
    let users = users::table
        .select(User::as_select())
        .filter(users::favorite_number.lt(U256::MAX))
        .load::<User>(connection)?;
    Ok(users)
}

pub mod models;
pub mod schema;
