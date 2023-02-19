-- Your SQL goes here
CREATE TABLE products (
  pid VARCHAR PRIMARY KEY,
  v_id VARCHAR NOT NULL,
  CONSTRAINT fk_vendor
      FOREIGN KEY(v_id) 
	    REFERENCES vendors(vid),
  in_stock BOOLEAN NOT NULL DEFAULT FALSE,
  p_description TEXT NOT NULL,
  p_name VARCHAR NOT NULL,
  p_price BIGINT NOT NULL,
  qty BIGINT NOT NULL
)
