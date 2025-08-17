use serde::{Deserialize, Serialize};    // For converting between Rust structs and JSON
use serde_json::json;
use axum::{
    extract:: {Path, State},
    Json, // `Json` lets us work with JSON data easily
    http::StatusCode
};
//use axum_macros::{debug_handler};
use crate::db::{DbPool};
use std::{env};   // To define the address/port the server listens on 
use reqwest::Client;
use serde_json::Value; // when you want to parse JSON without a predefined Rust struct, or you want to inspect JSON dynamically, you use serde_json::Value
use crate::models::Document;
use crate::schema::documents::dsl::documents;
use diesel::prelude::*;

// Define what we expect the client to send in the request body (JSON)
#[derive(Deserialize, Serialize)] //JSON to struct
pub struct AnalyzeRequest{
    instruction: String, // User-defined analysis
}

// Define what we'll send back in the response body (also JSON)
#[derive(Serialize)]
pub struct AnalyzeResponse{
    response: String, // LLM-generated or dummy response
}


//Pattern: Extractor<Type> = “Get this thing from the request and parse it into Type.”
#[axum::debug_handler]
pub async fn analyze_handler(
    Path(doc_id):Path<i32>, 
    State(pool):State<DbPool>,
    Json(instruction_choice):Json<AnalyzeRequest>
    ) -> Result<Json<AnalyzeResponse>, (StatusCode, String)>{
    let python_url = env::var("PYTHON_MICROSERVICE_URL")
        .expect("PYTHON_MICROSERVICE_URL not set");

    
    let mut conn = pool.get().map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB pool error".to_string()))?;
    let document = documents
        .find(doc_id) // shorthand for WHERE id = doc_id
        .first::<Document>(&mut conn) // executes the query on the db connection, maps the first result to the RUST Document Struct
        .map_err(|_| (StatusCode::NOT_FOUND, "Document not found".to_string()))?;

    // get the text content
    // if the Option is Some(value), give the value
    // if it's None, give the defaulr for that type
    let text_content = document.content.unwrap_or_default();

    let client = Client::new();
    // json! is a macro invocation
    let payload = json!({
        "input_text": text_content, 
        "instruction": instruction_choice.instruction.clone()
    });
    let res = client
        .post(&python_url)
        .json(&payload)
        .send() // sends the http request asynchronously, returning a Future that resolves to a Response
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "LLM request failed".to_string()))?;  

    
    // value from serde_json is an enum representing any valid JSON value
    // the enum Value might contain: 
    // Null, Bool, Number, String, Array, Object(Map<String, Value)
    let response_json:Value = res.json()
                            .await
                            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse LLM response".to_string()))?;
                                                                                                    // if the return from map_err is Ok(value), it unwraps it, so the assignment works
                                                                                                    // if not, it returns early from the program with a return type (StatusCode, String) which matches the return type Reuslt<Json<AnalyzeResponse>, (StatusCode, String))>
    
    let response_text = response_json["response"]
        .as_str()
        .unwrap_or("Missing response")
        .to_string();

    Ok(Json(AnalyzeResponse{response:response_text}))
    }