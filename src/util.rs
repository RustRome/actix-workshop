use diesel::{
    r2d2::{ConnectionManager, Pool},
    sqlite::SqliteConnection,
};
use dotenv::dotenv;
use std::env;

embed_migrations!();
#[cfg(not(test))]
#[allow(unused_imports)]
pub fn pool() -> Pool<ConnectionManager<SqliteConnection>> {
    create_pool("DATABASE_URL")
}

fn create_pool(env: &str) -> Pool<ConnectionManager<SqliteConnection>> {
    dotenv().ok();
    let database_url = env::var(env).expect("DATABASE_URL must be set");

    let pool = Pool::builder()
        .build(ConnectionManager::<SqliteConnection>::new(database_url))
        .expect("Failed to create the pool");

    let conn = pool.get().expect("Failed to get a connection");

    embedded_migrations::run(&conn).expect("Failed to run migrations");

    drop(conn);

    pool
}

#[allow(unused_imports)]
#[cfg(test)]
pub fn pool() -> Pool<ConnectionManager<SqliteConnection>> {
    use std::fs;

    dotenv().ok();

    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");

    match fs::remove_file(database_url) {
        Ok(_) => {}
        _ => {}
    }

    create_pool("TEST_DATABASE_URL")
}
