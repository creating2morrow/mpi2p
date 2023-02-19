-- Your SQL goes here
CREATE TABLE authorizations (
  id SERIAL PRIMARY KEY,
  xmr_address VARCHAR NOT NULL,
  rnd VARCHAR NOT NULL,
  created INT NOT NULL
)
