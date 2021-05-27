FROM rust

RUN apt-get update \
  && apt-get install -yy wait-for-it \
  && rm -rf /var/lib/apt/lists/*

RUN cargo install sqlx-cli --no-default-features --features postgres && \
  cargo install cargo-watch && \
  cargo install systemfd
WORKDIR /app
