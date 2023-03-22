CREATE TABLE users (
    id SERIAL8 PRIMARY KEY,
    name VARCHAR(255) NOT NULL ,
    surname VARCHAR(255)  NOT NULL ,
    password_hash VARCHAR(64)  NOT NULL
);
