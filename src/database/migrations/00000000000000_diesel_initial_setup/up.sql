CREATE TABLE IF NOT EXISTS Users (
        id SERIAL PRIMARY KEY,
        username VARCHAR NOT NULL UNIQUE,
        passkey VARCHAR NOT NULL,
        email VARCHAR UNIQUE
    );

CREATE TABLE IF NOT EXISTS MFA (
        id integer REFERENCES Users (id) ON DELETE CASCADE NOT NULL,
        otp integer PRIMARY KEY 
    );

CREATE TABLE IF NOT EXISTS Session (
        user_id integer REFERENCES Users (id) ON DELETE CASCADE NOT NULL,
        session_token BYTEA PRIMARY KEY
    )
