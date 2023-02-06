-- Your SQL goes here
CREATE TABLE orders (
  id SERIAL PRIMARY KEY,
  CONSTRAINT fk_customer
      FOREIGN KEY(id) 
	    REFERENCES customers(id),
  CONSTRAINT fk_product
      FOREIGN KEY(id) 
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
  o_status TEXT
)
