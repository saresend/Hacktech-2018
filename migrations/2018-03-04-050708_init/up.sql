-- Your SQL goes here

CREATE TABLE CurrEvent (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL, 
    lat REAL NOT NULL,
    lng REAL NOT NULL, 
    startTime TEXT NOT NULL, 
    endTime TEXT NOT NULL
);

CREATE TABLE NewPost (
    id INTEGER PRIMARY KEY NOT NULL,
    url TEXT NOT NULL, 
    userId TEXT NOT NULL, 
    eventId INTEGER NOT NULL, 
    lat REAL NOT NULL, 
    lng REAL NOT NULL,
    FOREIGN KEY(eventId) REFERENCES CurrEvent(id)
);