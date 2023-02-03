-- Your SQL goes here
CREATE TABLE vendors (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  description TEXT NOT NULL,
  active BOOLEAN NOT NULL DEFAULT FALSE
)
