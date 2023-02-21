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
  o_xmr_address VARCHAR NOT NULL,
  o_date BIGINT NOT NULL,
  o_deliver_date BIGINT NOT NULL,
  o_ship_date BIGINT NOT NULL,
  o_hash VARCHAR NOT NULL,
  o_msig_prepare TEXT NOT NULL,
  o_msig_make TEXT NOT NULL,
  o_msig_kex TEXT NOT NULL,
  o_msig_kex_boost TEXT NOT NULL,
  o_status TEXT NOT NULL,
  o_quantity BIGINT NOT NULL
)
