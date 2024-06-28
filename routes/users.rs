use async_trait::async_trait;
use mongodb::{bson::{doc}, Client, Collection};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use argon2::{self, Config};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    username: String,
    password: String,
    email: String,
}

#[async_trait]
trait Cache<T> {
    async fn get(&self, key: &str) -> Option<T>;
    async fn set(&self, key: &str, value: T);
}

struct UserCache {
    users: Arc<Mutex<HashMap<String, User>>>,
}

#[async_trait]
impl Cache<User> for UserCache {
    async fn get(&self, key: &str) -> Option<User> {
        let users = self.users.lock().await;
        users.get(key).cloned()
    }

    async fn set(&self, key: &str, value: User) {
        let mut users = self.users.lock().await;
        users.insert(key.to_string(), value);
    }
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
    // Your existing implementation remains unchanged
}

async fn register_user(username: &str, password: &examplen(String), email: &str, global_cache: Arc<UserCache>) -> Result<(), String> {
    // Before proceeding to register, check if the user already exists in the cache to prevent unnecessary DB calls.
    if global_cache.get(username).await.is_some() {
        return Err("User already exists".to_string());
    }
    
    // Assuming you have a mechanism to insert into your MongoDB collection here, which sets the user.
    // Let's say the user is successfully inserted:
    let hashed_password = User::hash_password(password).await;
    let user = User {
        username: username.to_string(),
        password: hashed_password,
        email: email.to_string(),
    };
    
    // Update cache after successful registration
    global_cache.set(username, user.clone()).await;
    
    Ok(())
}

async fn get_user_info(username: &str, global_cache: Arc<UserManager>) -> Result<User, String> {
    // Check cache first
    if let Some(user) = global_cache.get(username).await {
        return Ok(user);
    }

    // Original data fetching logic
    let collection = get_database().await;
    match collection.find_one(doc! { "username": username }, None).await {
        Ok(Some(user)) => {
            // Update cache
            global_cache.set(username, user.clone()).await;
            Ok(user)
        },
        Ok(None) => Err("User not found".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tokio::main]
async fn main() {
    let global_cache = Arc::new(UserCache {
        users: Arc::new(Mutex::new(HashMap::new())),
    });

    // Example registration and retrieval from cache to demonstrate approach
}