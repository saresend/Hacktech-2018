-- Your SQL goes here

CREATE TABLE CurrEvent (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL, 
    lat REAL NOT NULL,
    lng REAL NOT NULL, 
    start_time TEXT NOT NULL, 
    end_time TEXT NOT NULL
);

CREATE TABLE NewPost (
    id SERIAL PRIMARY KEY,
    url TEXT NOT NULL, 
    user_id INTEGER NOT NULL, 
    lat REAL NOT NULL, 
    lng REAL NOT NULL
);