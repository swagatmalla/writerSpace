mod db;
mod schema;
mod models;
mod handlers;

// Bring in required crate from Axum (web framework), Serde (for JSON), and standard library
use axum::{
    routing::{post, get, put}, 
    Router, 
};

// to allow Cross Origin Resource Sharing 
use tower_http::cors::{Any, CorsLayer};

use std::{env, net::SocketAddr};   // To define the address/port the server listens on 
use dotenvy::dotenv;
//use reqwest::Client;
//use serde_json::Value; // when you want to parse JSON without a predefined Rust struct, or you want to inspect JSON dynamically, you use serde_json::Value
use crate::db::{DbPool, establish_connection_pool};

// The main function sets up the Axum server and defines the routes
#[tokio::main] // macro to set up the asynchronous runtime using Tokio
async fn main(){
    dotenv().ok(); // Load env vars from `.env`
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool:DbPool = establish_connection_pool(&database_url);
    //Create the CORS middleware
    let cors = CorsLayer::new()
        .allow_origin(Any) //allow all origins (for dev)
        .allow_methods(Any)
        .allow_headers(Any);

    // Create the router and add a POST route at `/analyze` handled by `analyze_handler`
    let app = Router::new()
            .route("/users", get(handlers::users::get_users))
            .route("/users", post(handlers::users::create_user_handler))
            .route("/users/:user_id/projects", post(handlers::projects::create_project_handler))
            .route("/users/:user_id/projects", get(handlers::users::get_projects))
            .route("/projects/:project_id/documents", post(handlers::documents::create_document_handler))
            .route("/projects/:project_id/documents", get(handlers::projects::get_project_documents))
            .route("/documents/:doc_id/analyze", post(handlers::analyze::analyze_handler))
            .route("/documents/:doc_id", get(handlers::documents::get_document))
            .route("/documents/:doc_id", put(handlers::documents::update_document))
            .layer(cors)
            .with_state(pool.clone());

    // Set the address to listen on (localhost:3000)
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // A more verbose way: 
    // let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000); 
    println!("✍️ - Listening on http://{}", addr);
    

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

