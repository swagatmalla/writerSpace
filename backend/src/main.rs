mod db;
mod schema;
mod models;
mod handlers;

// Bring in required crated from Axum (web framework), Serde (for JSON), and standard library
use axum::{
    routing::{post, get}, //{delete, patch, put}
    Json, Router, // `Json` lets us work with JSON data easily
};

use serde::{Deserialize, Serialize};    // For converting between Rust structs and JSON
use std::{env, net::SocketAddr};   // To define the address/port the server listens on 
//use dotenvy::dotenv;
use reqwest::Client;
use serde_json::Value; // when you want to parse JSON without a predefined Rust struct, or you want to inspect JSON dynamically, you use serde_json::Value
use crate::db::{DbPool, establish_connection_pool};


// todo: Possibly clean up the code for all the `crate` imports
//use crate::schema::users::dsl::*;
//use crate::schema::documents::dsl::*;
//use crate::schema::projects::dsl::*;
//use crate::models::{User, Project, Document};

// The main function sets up the Axum server and defines the routes
#[tokio::main] // macro to set up the asynchronous runtime using Tokio
async fn main(){
    dotenv().ok(); // Load env vars from `.env`
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool:DbPool = establish_connection_pool(&database_url);

    // Create the router and add a POST route at `/analyze` handled by `analyze_handler`
    let app = Router::new()
            .route("/analyze", post(handlers::analyze::analyze_handler))
            .route("/users", get(handlers::users::get_users))
            .route("/createUser", post(handlers::users::create_user_handler))
            .route("/createProject", post(handlers::posts::create_project_handler))
            .with_state(pool.clone());

    // Set the address to listen on (localhost:3000)
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // A more verbose way: 
    // let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000); 
    println!("✍️ Listening on http://{}", addr);
    

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

