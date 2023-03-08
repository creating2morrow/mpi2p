# mpi2p

[![Rust](https://github.com/creating2morrow/mpi2p/actions/workflows/rust.yml/badge.svg)](https://github.com/creating2morrow/mpi2p/actions/workflows/rust.yml)

i2p marketplace api standards bootstrap

* This is project is primarily for research and development use
* Contributors welcome!

## I2P (invisble internet)

* download from [geti2p.org](https://geti2p.org/en/)
* setup tunnel at http://127.0.0.1:7657/i2ptunnelmgr

## Rustlang

* https://www.rust-lang.org/learn
* rust-analyzer vscodium plugin (optional)

## Rocket 

* Web-framework information available at [rocket.rs](https://rocket.rs/)
* see `Cargo.toml`

## PostgreSQL

* install from [Podman](https://registry.hub.docker.com/_/postgres/) official image
* `podman pull postgres`
* `podman run --rm -P -p 127.0.0.1:5432:5432 -e POSTGRES_DB=postgres -e POSTGRES_PASSWORD=postgres --name pg postgres:latest`
* set db with the r flag, e.g. `cargo run -- -p <PGDB_URL>`
*  default is - `postgres://postgres:postgres@127.0.0.1:5432/postgres`

## Diesel ORM

* install diesel cli from [diesel.rs](https://diesel.rs/guides/configuring-diesel-cli.html)
* `diesel migration run --database-url <db_url_string>`

## Monero

* get info [getmonero.org](https://getmonero.org)
* multisignature wallets to secure and release funds per order
* start monero-wallet-rpc e.g `./monero-x86_64-linux-gnu-v0.18.1.2/monero-wallet-rpc --rpc-bind-port 38083 --stagenet --wallet-file Monero/test --rpc-login user:pass --prompt-for-password  --daemon-address  http://stagenet.xmr-tw.org:38081`

## Development

* setup dependencies as required
* `lib.rs` - add new modules here
* related code should have a module (customer, vendor, etc.)
* `models.rs` - if updating db generate new `schema.rs`
    * `diesel migration run`
    * `rm src/schema.rs && diesel print-schema --database-url <db_url_string> > src/schema.rs` 
* `main.rs` - controllers / app startup are here
* `RUST_LOG=debug cargo run -- --monero-rpc-username user --monero-rpc-cred pass`
* hack away

## Production

Maybe one day

## Podman Container

* in progress

## API Examples

* see [api.md](./api.md)
* curl or vscodium http rest client plugin

## Authentication / Authorization Lifecycle

User signs rotating random data per request and the `Authorization` header
is screened accordingly. The login API is used to get the random data.

`Authorization: <ADDRESS>:<SIGNATURE>`

{in progress}: encrypt the plain-text auth into a JWT.
* make the expiration of the tokens configurable at run-time

### Authentication

* `/login/<customer|vendor>/<address>/<signature>`
* on initial request just send address and signature but it doesn't matter any string
* get auth response and sign the `data` in it with xmr wallet [sign]() feature
* call the login API a second time to create customer or vendor

### Authorization

* protected APIs currently require address/signature path parameters
* <i>Future use</i>: set `Authorization: Bearer <JWT(claim:ADDRESS:SIGNATURE)>` on subsequent requests
* address must match the one on file during login
* if auth expiration then new data must be signed (default : 60 minutes)
* call the login API again to refresh and sign new data
* the login API will create customer or vendor if they dont exist
* to check for existence use the `/customer|vendor/<ADDRESS>` API

## High-Level Order Lifecycle

* vendor adds a new product with description and price
* customer orders product
* app initiates multisig account creation for order
* customer and vendor exchange keys with app
* customer funds multisig wallet
* vendor ships
* customer recieves product and signs multisig transaction
* vendors signs and submits multisig transaction
* disputes handled on case-by-case basis, app can sign disputes only
* {in-progress} - detailed flow

## Status

* [x] Customer APIs
* [x] Vendor APIs
* [x] Product APIs
* [x] Order APIs / Monero Multisignature integration - 3/23
* [x] Automate I2P tunnel creation, etc. - 3/23
* [ ] From_Request Async Trait Authorization flow - 4/23
* [ ] Unit tests - 5/23
* [ ] Functional tests - 5/23
* [ ] Pentest - 5/23
* [ ] Documentation - 5/23
* [ ] GUI - 6/23
* [ ] Beta release v0.1.0-beta 7/23
