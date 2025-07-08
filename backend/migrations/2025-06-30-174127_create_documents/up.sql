CREATE TABLE documents (
    id  SERIAL PRIMARY KEY, 
    project_id INTEGER REFERENCES projects(id) ON DELETE CASCADE, 
    title TEXT NOT NULL, 
    content TEXT, 
    media_type TEXT DEFAULT 'text', 
    file_path TEXT, 
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, 
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);