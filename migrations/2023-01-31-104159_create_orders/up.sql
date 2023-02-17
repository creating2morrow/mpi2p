-- Your SQL goes here
CREATE TABLE orders (
  id SERIAL PRIMARY KEY,
  c_id INT,
  p_id INT,
  CONSTRAINT fk_customer
      FOREIGN KEY(c_id) 
	    REFERENCES customers(id),
  CONSTRAINT fk_product
      FOREIGN KEY(p_id) 
	    REFERENCES products(id),
  o_xmr_address VARCHAR,
  o_date INT NOT NULL,
  o_deliver_date INT,
  o_ship_date INT,
  o_hash VARCHAR,
  o_msig_prepare TEXT,
  o_msig_make TEXT,
  o_msig_kex TEXT,
  o_msig_kex_boost TEXT,
  o_status TEXT,
  o_quantity INT NOT NULL
)
