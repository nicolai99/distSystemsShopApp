# distSystemsShopApp

## Installation
docker-compose build\
docker-compose up

## Last Updates
- Docker-Compose erweitert
- Rust-Backend auf Postgres umstellt (Cargo.toml und Abfragen in main.rs)
- Defaul url in Rocket.toml
- Postgres:
    - sqlx migrate add init
    - migrations/init.sql bearbeitet (CREATE TABLE...)