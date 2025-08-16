use axum::{
    extract::{State, Path}, // Lets you extract shared application state (like the DB pool) inside a handler
    Json, // `Json` lets us work with JSON data easily
    
};

use diesel::{prelude::*}; // brings in all common Diesel traits and helper functions into scope
use chrono::Utc;
use crate::schema::projects::dsl::*;
use crate::models::Project;
use crate::models::NewProject;
use crate::db::{DbPool};

#[derive(serde::Deserialize)]
pub struct NewUserInput {
    title: String, 
    description: String
}

pub async fn create_project_handler(
    Path(user_id_url): Path<i32>,
    State(pool): State<DbPool>,
    Json(input): Json<NewUserInput>,
    //return type -> Result<Json<Project>, (StatusCode, String)>
) -> Result<Json<Project>, (axum::http::StatusCode, String)>{ // return a json object on a succesful operation, or an error tuple on a FAIL

    let new_project = NewProject {
        user_id: Some(user_id_url),
        title: input.title.clone(), 
        description: Some(input.description.clone()), 
        created_at: Some(Utc::now().naive_utc()), 
        updated_at: Some(Utc::now().naive_utc()),

    };

    let mut conn = pool.
        get().map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?; // ? to short-circuit the program incase of a failure

    let inserted_project:Project = diesel::insert_into(projects)
        .values(&new_project)
        .get_result(&mut conn)
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    return Ok(Json(inserted_project));
}
