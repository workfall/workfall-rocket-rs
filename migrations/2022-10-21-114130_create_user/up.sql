-- Your SQL goes here

CREATE TABLE users (
  id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  first_name VARCHAR NOT NULL,
  middle_name VARCHAR,
  last_name VARCHAR NOT NULL,
  email VARCHAR NOT NULL UNIQUE CONSTRAINT proper_email CHECK (email ~* '^[A-Za-z0-9._+%-]+@[A-Za-z0-9.-]+[.][A-Za-z]+$'),
  password VARCHAR NOT NULL
)