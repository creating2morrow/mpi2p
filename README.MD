# mpi2p

i2p marketplace api standards bootstrap

* This is project is for research and development use
* Contributors welcome!

## I2P (invisble internet)

* download from [geti2p.org](https://geti2p.org/en/)
* setup tunnel at http://127.0.0.1:7657/i2ptunnelmgr

## Rocket 

* Web-framework information available at [rocket.rs](https://rocket.rs/)
* see `Cargo.toml`

## PostgreSQL

* install from [Podman](https://registry.hub.docker.com/_/postgres/) official image
* `podman pull postgres`
* `podman run --rm -P -p 127.0.0.1:5432:5432 -e POSTGRES_DB=postgres -e POSTGRES_PASSWORD=postgres --name pg postgres:latest`
* set db with the r flag, e.g. `cargo run -- -r <PGDB_URL>`
*  default is - `postgres://postgres:postgres@127.0.0.1:5432/postgres`

## Diesel ORM

* install diesel cli from [diesel.rs](https://diesel.rs/guides/configuring-diesel-cli.html)
* run `diesel migration generate && diesel print-schema > src/schema.rs && diesel migration run`

## Monero

* get info [getmonero.org](https://getmonero.org)
* wallet signing for login (see api.sh) for sample CURL requests
* multisignature wallets to secure and release funds per order
* start monero-wallet-rpc e.g `./monero-x86_64-linux-gnu-v0.18.1.2/monero-wallet-rpc --rpc-bind-port 38083 --stagenet --wallet-file Monero/test --disable-rpc-login --prompt-for-password  --daemon-address  http://stagenet.xmr-tw.org:38081`

## Development

* setup dependencies as required
* `cargo run`
* hack away

## API Examples

* see `api.sh`
* {in progress}: swagger open api

### Authorization

User signs rotating random data per request and the `Authorization` header
is screened accordingly. The login API is used to get the random data.

`Authorization: <ADDRESS>:<SIGNATURE>`

{in progress}: encrypt the plain-text auth into a JWT.
* make the expiration of the tokens configurable at run-time

## Authentication / Authorization Lifecycle

### Authentication

* /login/<customer|vendor>/<address>/<signature>
* on initial request just send address and signature but it doesn't matter any string
* get auth response and sign the `data` in it with xmr wallet [sign]() feature
* call the login API a second time to create customer or vendor

### Authorization

* protected APIs currently require address/signature path parameters
* <i>Future use</i>: set `Authorization: <ADDRESS>:<SIGNATURE>` on subsequent requests
* address must match the one on file during login
* if auth expiration then new data must be signed (default : 60 minutes)
* call the login API again to refresh and sign new data
* the login API will create customer or vendor if they dont exist
* to check for existence use auth header and xmr address in the /customer|vendor/<ADDRESS> API

## High Level Order Lifecycle

* vendor adds a new product with description and price
* customer orders product
* app initiates multisig account creation for order
* customer and vendor exchange keys with app
* customer funds multisig wallet
* vendor ships
* customer recieves product and signs multisig transaction
* vendors signs and submits multisig transaction
* disputes handled on case-by-case basis, app can sign disputes only

## Status

* [x] Initial commit
* [x] Customer APIs
* [x] Vendor APIs
* [x] Product APIs
* [ ] Order APIs / Monero Multisignature integration - 3/23
* [ ] From_Request Async Trait Authorization flow - 4/23
* [ ] Unit tests - 5/23
* [ ] Functional tests - 5/23
* [ ] Pentest - 5/23
* [ ] Documentation - 5/23
* [ ] GUI - 6/23
* [ ] Beta release v0.1.0-beta 6/23
