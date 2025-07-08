use serde::{Deserialize, Serialize};    // For converting between Rust structs and JSON
use axum::{
    Json // `Json` lets us work with JSON data easily
};
use std::{env};   // To define the address/port the server listens on 
use dotenvy::dotenv;
use reqwest::Client;
use serde_json::Value; // when you want to parse JSON without a predefined Rust struct, or you want to inspect JSON dynamically, you use serde_json::Value


// Define what we expect the client to send in the request body (JSON)
#[derive(Deserialize, Serialize)] //JSON to struct
pub struct AnalyzeRequest{
    input_text: String, // The user's writing
    instruction: String, // User-defined analysis
}

// Define what we'll send back in the response body (also JSON)
#[derive(Serialize)]
pub struct AnalyzeResponse{
    response: String, // LLM-generated or dummy response
}
                // Json() is  type and a tuple struct constructor from the Axum framework 
pub async fn analyze_handler(Json(payload): Json<AnalyzeRequest>) -> Json<AnalyzeResponse>{
    let python_url = env::var("PYTHON_MICROSERVICE_URL")
        .expect("PYTHON_MICROSERVICE_URL not set");

    let client = Client::new();
    let res = client
        .post(&python_url)
        .json(&payload)
        .send() // sends the http request asynchronously, returning a Future that resolves to a Response
        .await
        .expect("Failed to contact Python microservice"); 

    let response_json:Value = res.json().await.expect("Invalid JSON from Python");
    let response_text = response_json["response"]
        .as_str()
        .unwrap_or("Missing response")
        .to_string();

    Json(AnalyzeResponse{response:response_text})
    }