-- Your SQL goes here
CREATE TABLE orders (
  orid VARCHAR PRIMARY KEY,
  c_id VARCHAR NOT NULL,
  p_id VARCHAR NOT NULL,
  v_id VARCHAR NOT NULL,
  CONSTRAINT fk_customer
      FOREIGN KEY(c_id) 
	    REFERENCES customers(cid),
  CONSTRAINT fk_product
      FOREIGN KEY(p_id) 
	    REFERENCES products(pid),
  CONSTRAINT fk_vendor
      FOREIGN KEY(v_id) 
	    REFERENCES vendors(vid),
  o_xmr_address VARCHAR NOT NULL,
  o_cust_msig_info VARCHAR NOT NULL,
  o_cust_msig_txset VARCHAR NOT NULL,
  o_cust_kex_1 VARCHAR NOT NULL,
  o_cust_kex_2 VARCHAR NOT NULL,
  o_cust_kex_3 VARCHAR NOT NULL,
  o_date BIGINT NOT NULL,
  o_deliver_date BIGINT NOT NULL,
  o_ship_date BIGINT NOT NULL,
  o_hash VARCHAR NOT NULL,
  o_msig_prepare TEXT NOT NULL,
  o_msig_make TEXT NOT NULL,
  o_msig_kex_1 TEXT NOT NULL,
  o_msig_kex_2 TEXT NOT NULL,
  o_msig_kex_3 TEXT NOT NULL,
  o_subaddress VARCHAR NOT NULL,
  o_status TEXT NOT NULL,
  o_quantity BIGINT NOT NULL,
  o_vend_kex_1 VARCHAR NOT NULL,
  o_vend_kex_2 VARCHAR NOT NULL,
  o_vend_kex_3 VARCHAR NOT NULL,
  o_vend_msig_info VARCHAR NOT NULL,
  o_vend_msig_txset VARCHAR NOT NULL
)
