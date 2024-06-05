use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::env;
use dotenv::dotenv;

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    username: String,
}

#[derive(Serialize, Deserialize)]
struct Vote {
    id: u32,
    user_id: u32,
    candidate: String,
}

async fn vote(_: web::Json<Vote>) -> impl Responder {
    HttpResponse::Ok().json("Vote registered")
}

async fn get_votes() -> impl Responder {
    HttpResponse::Ok().json(vec![
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
    ])
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let server_url = env::var("SERVER_URL").expect("SERVER_URL not found.");

    HttpServer::new(|| {
        App::new()
            .route("/vote", web::post().to(vote))
            .route("/votes", web::get().to(get_votes))
            .route("/user", web::post().to(add_user))
            .route("/users", web::get().to(get_users))
    })
    .bind(server_url)?
    .run()
    .await
}