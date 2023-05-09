FROM rust

WORKDIR /usr/app

RUN cargo install diesel_cli

COPY ./ ./

RUN cargo build --release \
    && mv target/release/galactic_gambit_leaderboard . \
    && cargo clean

EXPOSE 3000

ENTRYPOINT ["./entrypoint.sh"]