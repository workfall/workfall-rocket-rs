-- Your SQL goes here

CREATE TABLE users (
  id VARCHAR PRIMARY KEY NOT NULL,
  first_name VARCHAR NOT NULL,
  middle_name VARCHAR,
  last_name VARCHAR NOT NULL,
  email VARCHAR NOT NULL UNIQUE CONSTRAINT proper_email CHECK (email ~* '^[A-Za-z0-9._+%-]+@[A-Za-z0-9.-]+[.][A-Za-z]+$'),
  role_id VARCHAR REFERENCES roles (id) NOT NULL,
  password VARCHAR NOT NULL
)
