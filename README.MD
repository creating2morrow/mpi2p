# mpi2p

[![rust-server](https://github.com/creating2morrow/mpi2p/actions/workflows/rust_server.yml/badge.svg)](https://github.com/creating2morrow/mpi2p/actions/workflows/rust_server.yml)
[![rust-egui](https://github.com/creating2morrow/mpi2p/actions/workflows/rust_gui.yml/badge.svg?branch=egui&event=push)](https://github.com/creating2morrow/mpi2p/actions/workflows/rust_gui.yml)

i2p marketplace api standards bootstrap

* This is project is primarily for research and development use
* Contributors welcome!

## I2P (invisble internet)

* automated tunnel control
* i2p-zero

## Rustlang

* https://www.rust-lang.org/learn
* rust-analyzer vscodium plugin (optional)

## Rocket 

* Web-framework information available at [rocket.rs](https://rocket.rs/)
* see `Cargo.toml`

## lmdb

* need to update lmdb-rs
* wip

## Monero

* get info [getmonero.org](https://getmonero.org)
* multisignature wallets to secure and release funds per order

## Development

* setup dependencies as required
* `lib.rs` - add new modules here
* related code should have a module (customer, vendor, etc.)
* `models.rs` - db structs
* `main.rs` - controllers / app startup are here
* `RUST_LOG=debug cargo run -- --monero-rpc-username user --monero-rpc-cred pass`
* hack away

## Production

Maybe one day

## Podman Container

* in progress

## API Examples

* see [curl.md](./curl.md)
* curl or vscodium http rest client plugin

## Authentication / Authorization Lifecycle

User signs rotating random data per request and the `Authorization` header
is screened accordingly. The login API is used to get the random data.

`token: <JWT>`

### Authentication

* `/login/<customer|vendor>/<address>/<signature>`
* on initial request send address and signature but it doesn't matter data signed
* get auth response and sign the `data` in it with xmr wallet [sign](https://www.getmonero.org/resources/developer-guides/wallet-rpc.html#sign) feature
* call the login API a second time to create customer or vendor

### Authorization

* `token: <JWT(claim:ADDRESS,EXPIRATION)>` on subsequent requests
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

## GUI

* experimental [egui](https://docs.rs/egui/latest/egui/)
* push to egui branch

## Status

* [x] Customer APIs
* [x] Vendor APIs
* [x] Product APIs
* [x] Order APIs / Monero Multisignature integration
* [x] Automate I2P tunnel creation, etc.
* [x] From_Request Async Trait Authorization flow
* [x] LMDB rewrite - 3/23
* [ ] Tests / Docs - 4/23
* [ ] GUI - 5/23
* [ ] Beta release v0.1.0-beta 6/23
