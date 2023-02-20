# API docs

# [GET] get the current monero-wallet-rpc version
curl http://127.0.0.1:8000/xmr/version

# [POST] sign data (monero-wallet-rpc)
curl http://127.0.0.1:38083/json_rpc -d '{"jsonrpc":"2.0","id":"0","method":"sign","params":{"data":"c7e3f99e69363517122e07bde9de48422b119640c56eceacf2e35a45f3693db3"}}' -H 'Content-Type: application/json'

# [POST] get addresses (monero-wallet-rpc)
curl http://127.0.0.1:38083/json_rpc -d '{"jsonrpc":"2.0","id":"0","method":"get_address","params":{"account_index":0,"address_index":[0]}}' -H 'Content-Type: application/json'

# [GET] login
# customer or vendor
# xmr address
# data - random bytes to sign
# signature - generate signature with wallet private keys
curl http://127.0.0.1:8000/login/<customer|vendor>/<XMR_ADDRESS>/<SIGNATURE>

# update
# customer or vendor URI
# <id> - i32
# <data> - String
# <update_type> - Enum => 0 - active, 1 - description, 2 - name, 3 - pgp
curl -X PATCH http://127.0.0.1:8000/<customer|vendor>/update/<id>/<data>/2

# get
# create a new product
# <id> - vendor id
curl -iv http://127.0.0.1:8000/product/create/1

# get
# return all products for a vendor
# <id> - vendor id
curl -iv http://127.0.0.1:8000/products/1

# update product
# <id> - i32
# <data> - String
# <update_type> - Enum => 0 - in_stock, 1 - description, 2 - name, 3 - price 4 - qty
curl -X PATCH http://127.0.0.1:8000/product/update/<id>/<data>/2
