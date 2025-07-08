
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
//use diesel::pg::Pg;
use diesel::{Insertable, Queryable, Selectable, pg::Pg};
use crate::schema::{users, projects, documents};

// macro to tell the compiler to automatically derive these traits for this struct
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Selectable)]
// Debug to print the struct
// Clone to easily pass the data around
// Serialize to convert the struct into JSON
// Deserialize to parse JSON (or other formats) into the struct
// Queryable to map results from a query result
// Insertable to tell how to insert his struct into the DB
#[diesel(table_name = users)]
#[diesel(check_for_backend(Pg))]
pub struct User{
    pub id: i32,
    pub username: String, 
    pub email: String, 
    pub hashed_password: String, 
    pub created_at: Option<NaiveDateTime>, 
    pub updated_at: Option<NaiveDateTime> 
}


#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Selectable)]
#[diesel(table_name = projects)]
#[diesel(check_for_backend(Pg))]
pub struct Project{
    pub id:i32, 
    pub user_id:Option<i32>,
    pub title:String, 
    pub description:Option<String>, 
    pub created_at:Option<NaiveDateTime>, 
    pub updated_at:Option<NaiveDateTime>
}


#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Selectable)]
#[diesel(table_name = documents)]
#[diesel(check_for_backend(Pg))]
pub struct Document{
    pub id: i32,
    pub project_id: Option<i32>, 
    pub title: String, 
    pub content: Option<String>, 
    pub media_type: Option<String>, 
    pub file_path:Option<String>, 
    pub created_at:Option<NaiveDateTime>, 
    pub updated_at: Option<NaiveDateTime>   
}

