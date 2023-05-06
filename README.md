# Galactic Gambit Leaderboard
This is a backend to control the entries on the leaderboard for our game Galactic Gambit

## Dependencies
- Rust
- Cargo
- Docker
- Docker Compose

## Getting started
Make sure to edit the docker-compose.yml file with the username, password and db name that you want to have.

To connect to the database a .env with the location of the database is needed, you can create it like this:
```DATABASE_URL=postgres://username:password@localhost:port/database > .env```

## How to run
To run the database you can run the following command from the root of the project:
```cargo run```
