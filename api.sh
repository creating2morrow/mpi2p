# API docs

# [GET] get the current monero-wallet-rpc version
curl http://127.0.0.1:8000/xmr/version

# [GET] login
# TODO: sign random data - current data to sign is "LOGIN"
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
