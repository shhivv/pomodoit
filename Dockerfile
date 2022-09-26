FROM rust:1.64

WORKDIR /bot

COPY Cargo.toml Cargo.lock .env ./
ADD src ./src

RUN cargo build --release

CMD ["./target/release/pomodoit"]