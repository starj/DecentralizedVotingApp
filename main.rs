use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::env;
use dotenv::dotenv;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: u32,
    username: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Vote {
    id: u32,
    user_id: u32,
    candidate: String,
}

async fn vote(_: web::Json<Vote>) -> impl Responder {
    HttpResponse::Ok().json("Vote registered")
}

lazy_static::lazy_static! {
    static ref VOTES: std::sync::Mutex<Vec<Vote>> = std::sync::Mutex::new(vec![
        Vote {
            id: 1,
            user_id: 1,
            candidate: String::from("Candidate A"),
        },
        Vote {
            id: 2,
            user_id: 2,
            candidate: String::from("Candidate B"),
        },
    ]);
}

async fn get_votes() -> impl Responder {
    let votes = VOTES.lock().unwrap();
    HttpResponse::Ok().json(&*votes)
}

async fn add_user(_: web::Json<User>) -> impl Responder {
    HttpResponse::Ok().json("User added")
}

async fn get_users() -> impl Responder {
    HttpResponse::Ok().json(vec![
        User {
            id: 1,
            username: String::from("User1"),
        },
        User {
            id: 2,
            username: String::from("User2"),
        },
    ])
}

async fn tally_votes() -> impl Responder {
    let votes = VOTES.lock().unwrap();
    let mut tally = HashMap::new();
    for vote in votes.iter() {
        *tally.entry(vote.candidate.clone()).or_insert(0) += 1;
    }
    HttpResponse::Ok().json(tally)
}

async fn custom_error_404() -> impl Responder {
    HttpResponse::NotFound().json("Not Found")
}

#[actix_web::main]
async fn main() {
    dotenv().ok();

    let server_url = env::var("SERVER_URL").unwrap_or_else(|_| String::from("127.0.0.1:8080"));
    
    HttpServer::new(|| {
        App::new()
            .route("/vote", web::post().to(vote))
            .route("/votes", web::get().to(get_votes))
            .route("/user", web::post().to(add_user))
            .route("/users", web::get().to(get_users))
            .route("/tally", web::get().to(tally_votes))
            .default_service(web::route().to(custom_error_404))
    })
    .bind(&server_url)
    .expect("Failed to bind server")
    .run()
    .await
    .expect("Failed to run server");
}