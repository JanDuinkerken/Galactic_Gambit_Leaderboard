# Galactic Gambit Leaderboard

This is a backend to control the entries on the leaderboard for our game Galactic Gambit

## Dependencies

- Rust
- Cargo
- Docker
- Docker Compose

## Getting started

Make sure to edit the `docker-compose.yml` file with the username, password and db name that you want to have.

To connect to the database a `.env` file with the location of the database is needed. Use the file `.env.example` as a template.

## How to run

To run the server you can run the following command from the root of the project:

```
docker compose up
```
