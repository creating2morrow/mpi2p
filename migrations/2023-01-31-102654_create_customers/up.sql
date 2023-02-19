-- Your SQL goes here
CREATE TABLE customers (
  cid VARCHAR PRIMARY KEY,
  c_xmr_address VARCHAR NOT NULL,
  c_name VARCHAR NOT NULL,
  c_pgp VARCHAR NOT NULL
)