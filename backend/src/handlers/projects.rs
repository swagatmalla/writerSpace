use axum::{
    extract::State, // Lets you extract shared application state (like the DB pool) inside a handler
    Json// `Json` lets us work with JSON data easily
};

//use serde::{Deserialize, Serialize};    // For converting between Rust structs and JSON
use diesel::{prelude::*}; // brings in all common Diesel traits and helper functions into scope
use chrono::Utc;
use crate::schema::users::dsl::*;
use crate::models::Project;
use crate::db::{DbPool};

#[derive(serde::Deserialize)]
pub struct NewUserInput {
    id: i32, 
    user_id: String,
    title: String, 
    description: String
}

pub async fn create_project_handler(
    State(pool): State<DbPool>,
    Json(input): Json<NewUserInput>,
) -> Result<Json<Project>, (axum::http::StatusCode, String)>{ // return a json object on a succesful operation, or an error tuple on a FAIL
    let new_user= Project{
        //id: 0, // will be ignored if auto-incremented
        user_id: Some(input.user_id.parse()),
        title: input.title.clone(), 
        description: Some(input.description.clone()), 
        created_at: Some(Utc::now().naive_utc()), 
        updated_at: Some(Utc::now().naive_utc()),

    };

    let mut conn = pool.
        get().map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?; // ? to short-circuit the program incase of a failure

    diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut conn)
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    return Ok(Json(new_user));
}
