-- Your SQL goes here
CREATE TABLE orders (
  orid VARCHAR PRIMARY KEY,
  c_id VARCHAR NOT NULL,
  p_id VARCHAR NOT NULL,
  CONSTRAINT fk_customer
      FOREIGN KEY(c_id) 
	    REFERENCES customers(cid),
  CONSTRAINT fk_product
      FOREIGN KEY(p_id) 
	    REFERENCES products(pid),
  o_xmr_address VARCHAR,
  o_date BIGINT NOT NULL,
  o_deliver_date BIGINT NOT NULL,
  o_ship_date BIGINT NOT NULL,
  o_hash VARCHAR,
  o_msig_prepare TEXT,
  o_msig_make TEXT,
  o_msig_kex TEXT,
  o_msig_kex_boost TEXT,
  o_status TEXT,
  o_quantity BIGINT NOT NULL
)
