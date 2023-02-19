-- Your SQL goes here
CREATE TABLE vendors (
  vid VARCHAR PRIMARY KEY,
  v_xmr_address VARCHAR NOT NULL,
  v_name VARCHAR NOT NULL,
  v_description TEXT NOT NULL,
  v_pgp TEXT NOT NULL,
  active BOOLEAN NOT NULL DEFAULT FALSE
)
