-- Your SQL goes here
CREATE TABLE authorizations (
  aid VARCHAR PRIMARY KEY,
  created BIGINT NOT NULL,
  rnd VARCHAR NOT NULL,
  xmr_address VARCHAR NOT NULL
)
