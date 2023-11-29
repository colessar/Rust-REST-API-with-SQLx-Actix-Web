CREATE TABLE todolist_entries (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255),
    complete BOOLEAN DEFAULT FALSE
);