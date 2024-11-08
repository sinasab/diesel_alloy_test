use diesel_uint_test::*;

fn main() {
    println!("establishing db connection");
    let connection = &mut establish_connection();

    println!("running migrations");
    run_migrations(connection).unwrap();

    println!("seeding data");
    seed_data(connection).unwrap();

    println!("querying db for all users");
    let results = get_users(connection).unwrap();
    for user in results {
        println!("{}", user);
    }

    println!("querying db for users with filter");
    let results = get_users_with_filter(connection).unwrap();
    for user in results {
        println!("{}", user);
    }

    println!("done!");
}
