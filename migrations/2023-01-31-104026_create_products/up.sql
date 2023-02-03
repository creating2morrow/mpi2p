-- Your SQL goes here
CREATE TABLE products (
  id SERIAL PRIMARY KEY,
  CONSTRAINT fk_vendor
      FOREIGN KEY(id) 
	    REFERENCES vendors(id),
  p_name VARCHAR NOT NULL,
  p_pgp VARCHAR NOT NULL,
  p_price INT NOT NULL,
  qty INT NOT NULL
)
