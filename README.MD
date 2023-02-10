# mpi2p

i2p marketplace api standards bootstrap

## Rocket 

* Web-framework available at [rocket.rs](https://rocket.rs/)
* place Rocket `core` directory in the root

## PostgreSQL

* install from [Docker](https://registry.hub.docker.com/_/postgres/) official image
* run `docker run --rm -P -p 127.0.0.1:5432:5432 -e POSTGRES_DB=postgres -e POSTGRES_PASSWORD=postgres --name pg postgres:latest`
* create an environment `.env` file with `DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/postgres`

## Diesel ORM

* install diesel cli from [diesel.rs](https://diesel.rs/guides/configuring-diesel-cli.html)
* run `diesel migration generate && diesel print-schema > src/schema.rs`
