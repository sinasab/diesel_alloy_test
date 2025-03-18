use crate::{models::*, schema::*};
use alloy_primitives::{Address, PrimitiveSignature, B256};
use alloy_signer::SignerSync;
use alloy_signer_local::PrivateKeySigner;
use diesel::{PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use std::{error::Error, str::FromStr};

// Generate a random signer and hash.
// A user record consists of:
// - the address of that signer
// - the random hash
// - signature over the hash by the signer
fn get_random_user() -> User {
    let random_signer = PrivateKeySigner::random();
    let random_hash = B256::random();
    let sig = random_signer.sign_hash_sync(&random_hash).unwrap();
    User {
        addr: Address::from_str(&random_signer.address().to_string()).unwrap(),
        sig: PrimitiveSignature::from_str(&sig.to_string()).unwrap(),
        hash: random_hash,
    }
}

pub fn validate_user(user: &User) -> Result<(), String> {
    let recovered_address = user.sig.recover_address_from_prehash(&user.hash).unwrap();
    if recovered_address != user.addr {
        return Err("Invalid signature".to_string());
    }
    Ok(())
}

pub fn seed_data(
    connection: &mut PgConnection,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    for _ in 0..10 {
        let user = get_random_user();
        upsert_user(connection, &user)?;
    }
    Ok(())
}

fn upsert_user(connection: &mut PgConnection, user: &User) -> Result<(), String> {
    diesel::insert_into(users::table)
        .values(user)
        .on_conflict(users::addr)
        .do_update()
        .set(user)
        .execute(connection)
        .map_err(|_| "Failed to upsert user")?;
    Ok(())
}

pub fn get_users(connection: &mut PgConnection) -> Result<Vec<User>, String> {
    let users = users::table
        .select(User::as_select())
        .get_results(connection)
        .map_err(|_| "Failed to get users")?;
    Ok(users)
}
