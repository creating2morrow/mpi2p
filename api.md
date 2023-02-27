# API docs

```bash
# [GET] get the current monero-wallet-rpc version
curl http://127.0.0.1:8000/xmr/version

# [POST] sign data (monero-wallet-rpc)
curl http://127.0.0.1:38083/json_rpc --digest -u user:pass -d '{"jsonrpc":"2.0","id":"0","method":"sign","params":{"data":"some data here"}}' -H 'Content-Type: application/json'

# [POST] get addresses (monero-wallet-rpc)
curl http://127.0.0.1:38083/json_rpc --digest -u user:pass -d '{"jsonrpc":"2.0","id":"0","method":"get_address","params":{"account_index":0,"address_index":[0]}}' -H 'Content-Type: application/json'

# [GET] login
# customer or vendor
# xmr address
# data - random bytes to sign
# signature - generate signature with wallet private keys
curl http://127.0.0.1:8000/login/<customer|vendor>/<XMR_ADDRESS>/<SIGNATURE>

# [PATCH] update
# customer or vendor URI
# <id> - i32
# <data> - String
# <update_type> - Enum => 0 - active, 1 - description, 2 - name, 3 - pgp
curl -X PATCH http://127.0.0.1:8000/<customer|vendor>/<XMR_ADDRESS>/<SIGNATURE>/update/<data>/<update_type>

# get
# create a new product
curl -iv http://127.0.0.1:8000/<XMR_ADDRESS>/<SIGNATURE>/product/create

# get
# return all products for a vendor
curl -iv http://127.0.0.1:8000/<XMR_ADDRESS>/<SIGNATURE>/products/

# update product
# <pid> - i32
# <data> - String
# <update_type> - Enum => 0 - in_stock, 1 - description, 2 - name, 3 - price 4 - qty
curl -X PATCH http://127.0.0.1:8000/<XMR_ADDRESS>/<SIGNATURE>/product/update/<pid>/<data>/<update_type>
