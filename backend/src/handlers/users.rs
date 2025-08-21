use axum::{
    extract::State, // Lets you extract shared application state (like the DB pool) inside a handler
    Json// `Json` lets us work with JSON data easily
};

use diesel::{prelude::*}; // brings in all common Diesel traits and helper functions into scope
use chrono::Utc;
use crate::schema::users::dsl::*;
use crate::schema::projects::dsl::*;
use crate::models::{User, NewUser, Project};
use crate::db::{DbPool};

#[derive(serde::Deserialize)]
pub struct NewUserInput {
    username: String, 
    email: String, 
    password: String,
}

pub async fn create_user_handler(
    State(pool): State<DbPool>,
    Json(input): Json<NewUserInput>,
) -> Result<Json<User>, (axum::http::StatusCode, String)>{ // return a json object on a succesful operation, or an error tuple on a FAIL
    let new_user= NewUser{
        username: input.username.clone(),
        email: input.email.clone(),
        hashed_password: format!("hashed({})", input.password), // fake hashing for test
        created_at: Some(Utc::now().naive_utc()), 
        updated_at: Some(Utc::now().naive_utc()),

    };

    let mut conn = pool.
        get().map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?; // ? to short-circuit the program incase of a failure

    let inserted_user: User = diesel::insert_into(users)
        .values(&new_user)
        .get_result(&mut conn)
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    return Ok(Json(inserted_user));
}

pub async fn get_users(State(pool): State<DbPool>) -> Result<Json<Vec<User>>, (axum::http::StatusCode, String)>{
    let mut conn = pool.get().map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let results = users
    .select(User::as_select())
    .load(&mut conn)
    .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    return Ok(Json(results));
}

//-> Result<Json<User>, (axum::http::StatusCode, String)>
pub async fn get_projects(State(pool): State<DbPool>) -> Result<Json<Vec<Project>>, (axum::http::StatusCode, String)>{
    let mut conn = pool
            .get().map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let results = projects.
        select(Project::as_select())
        .load(&mut conn)
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;


    return Ok(Json(results));
    

}

