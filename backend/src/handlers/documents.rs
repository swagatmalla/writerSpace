use axum::{
    extract::State, // Lets you extract shared application state (like the DB pool) inside a handler
    extract::Path, // to extract things from the URL 
    Json// `Json` lets us work with JSON data easily
};

use diesel::{prelude::*}; // brings in all common Diesel traits and helper functions into scope
use chrono::Utc;
use crate::schema::documents::dsl::*;
use crate::models::{Document, NewDocument};
use crate::db::{DbPool};

#[derive(serde::Deserialize)]
pub struct NewUserInput { 
    pub title: String, 
    pub content: Option<String>, 
    pub media_type:Option<String>, 
    pub file_path:Option<String>, 
}

#[derive(serde::Deserialize, diesel::AsChangeset)]
#[diesel(table_name = crate::schema::documents)]
pub struct DocUpdate {
    pub title: Option<String>, 
    pub content: Option<String>
}

pub async fn update_document(
    Path(doc_id): Path<i32>, 
    State(pool):State<DbPool>, 
    Json(doc_update): Json<DocUpdate>
)->Result<Json<Document>, (axum::http::StatusCode, String)>{
    let mut conn = pool
        .get()
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let result  = diesel::update(documents.filter(id.eq(doc_id)))
            .set(&doc_update)
            .get_result(&mut conn)
            .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    return Ok(Json(result));
}

pub async fn get_document(
    Path(document_id_url): Path<i32>, 
    State(pool): State<DbPool>, 
)->Result<Json<Document>, (axum::http::StatusCode, String)>{
    let mut conn = pool
        .get()
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let result = documents
        .filter(id.eq(document_id_url))
        .select(Document::as_select())
        .first::<Document>(&mut conn)
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    return Ok(Json(result));


}

pub async fn create_document_handler(
    Path(project_id_url): Path<i32>,
    State(pool): State<DbPool>,
    Json(input): Json<NewUserInput>,
    //return type -> Result<Json<Project>, (StatusCode, String)>
) -> Result<Json<Document>, (axum::http::StatusCode, String)>{ // return a json object on a succesful operation, or an error tuple on a FAIL

    let new_document= NewDocument {
        project_id: Some(project_id_url), 
        title: input.title.clone(),  
        content: input.content.clone(),
        media_type: input.media_type.clone(), 
        file_path: input.file_path.clone(),
        created_at: Some(Utc::now().naive_utc()), 
        updated_at: Some(Utc::now().naive_utc()),

    };

    let mut conn = pool.
        get().map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?; // ? to short-circuit the program incase of a failure

    let inserted_document:Document = diesel::insert_into(documents)
        .values(&new_document)
        .get_result(&mut conn)
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    return Ok(Json(inserted_document));
}
