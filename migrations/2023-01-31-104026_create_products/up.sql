-- Your SQL goes here
CREATE TABLE products (
  id SERIAL PRIMARY KEY,
  v_id INT NOT NULL,
  CONSTRAINT fk_vendor
      FOREIGN KEY(v_id) 
	    REFERENCES vendors(id),
  in_stock BOOLEAN NOT NULL DEFAULT FALSE,
  p_description TEXT NOT NULL,
  p_name VARCHAR NOT NULL,
  p_price INT NOT NULL,
  qty INT NOT NULL
)
