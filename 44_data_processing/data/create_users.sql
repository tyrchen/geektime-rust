CREATE TABLE IF NOT EXISTS users
(
    id              INTEGER PRIMARY KEY NOT NULL,
    email           VARCHAR UNIQUE      NOT NULL,
    hashed_password VARCHAR             NOT NULL
);
