use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime, Utc};
use diesel::{Insertable, Queryable};
use dotenv::dotenv;
use std::env;

table! {
    users (id) {
        id -> Integer,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
        registration_date -> Timestamp,
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub registration_in_date: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub email: &'a str,
    pub registration_date: NaiveDateTime,
}

pub fn establish_connection() -> diesel::PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the .env file");
    diesel::PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_location))
}

pub fn create_user<'a>(conn: &PgConnection, username: &'a str, password: &'a str, email: &'a str) -> User {
    use schema::users;

    let new_user = NewUser {
        username,
        password,
        email,
        registration_date: Utc::now().naive_utc(),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user")
}