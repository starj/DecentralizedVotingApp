use std::env;
use serde::{Serialize, Deserialize};
use mongodb::{Client, options::ClientOptions, Collection};
use mongodb::bson::{doc, Document};
use argon2::{self, Config};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    password: String,
    email: String,
}

impl User {
    async fn hash_password(password: &str) -> String {
        let salt = b"randomsalt"; 
        let config = Config::default();
        argon2::hash_encoded(password.as_bytes(), salt, &config).unwrap()
    }

    async fn verify_password(hashed_password: &str, provided_password: &str) -> bool {
        argon2::verify_encoded(hashed_password, provided_password.as_bytes()).unwrap_or(false)
    }
}

async fn get_database() -> Collection<User> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
    let coll_name = env::var("COLLECTION_NAME").expect("COLLECTION_NAME must be set");

    let client_options = ClientOptions::parse(&db_url).await.expect("Failed to connect to MongoDB");
    let client = Client::with_options(client_options).expect("Failed to initialize MongoDB client");
    let database = client.database(&db_name);
    let collection: Collection<User> = database.collection(&coll_name);

    collection
}

async fn register_user(username: &str, password: &str, email: &str) -> Result<(), String> {
    let collection = get_database().await;
    let hashed_password = User::hash_password(password).await;

    let new_user = User {
        username: username.to_owned(),
        password: hashed_password,
        email: email.to_owned(),
    };

    match collection.insert_one(new_user, None).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

async fn get_user_info(username: &str) -> Result<User, String> {
    let collection = get_database().await;
    match collection.find_one(doc! { "username": username }, None).await {
        Ok(Some(user)) => Ok(user),
        Ok(None) => Err("User not found".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

async fn update_user_details(username: &str, new_data: Document) -> Result<(), String> {
    let collection = get_database().await;
    match collection.update_one(doc! { "username": username }, doc! { "$set": new_data }, None).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

async fn authenticate_user(username: &str, password: &str) -> Result<bool, String> {
    let user_info = get_user_info(username).await?;
    Ok(User::verify_password(&user_info.password, password).await)
}