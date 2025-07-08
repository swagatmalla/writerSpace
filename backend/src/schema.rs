// @generated automatically by Diesel CLI.

diesel::table! {
    documents (id) {
        id -> Int4,
        project_id -> Nullable<Int4>,
        title -> Text,
        content -> Nullable<Text>,
        media_type -> Nullable<Text>,
        file_path -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    projects (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        title -> Text,
        description -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        hashed_password -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(documents -> projects (project_id));
diesel::joinable!(projects -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    documents,
    projects,
    users,
);
