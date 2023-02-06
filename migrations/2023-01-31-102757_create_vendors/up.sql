-- Your SQL goes here
CREATE TABLE vendors (
  id SERIAL PRIMARY KEY,
  v_xmr_address VARCHAR NOT NULL,
  v_display_name VARCHAR NOT NULL,
  v_description TEXT NOT NULL,
  active BOOLEAN NOT NULL DEFAULT FALSE
)
