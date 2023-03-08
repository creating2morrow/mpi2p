-- Your SQL goes here
CREATE TABLE disputes (
  did VARCHAR PRIMARY KEY,
  created BIGINT NOT NULL,
  orid VARCHAR NOT NULL,
  tx_set VARCHAR NOT NULL
)
