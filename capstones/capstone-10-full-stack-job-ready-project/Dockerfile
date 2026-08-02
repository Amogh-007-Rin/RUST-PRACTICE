FROM rust:1.83-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY capstones/capstone-10-full-stack-job-ready-project/solution/Cargo.toml Cargo.toml
COPY capstones/capstone-10-full-stack-job-ready-project/solution/src/ src/

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/capstone-10-solution /usr/local/bin/url-shortener

ENV DATABASE_URL="sqlite:/data/url_shortener.db?mode=rwc"
ENV BASE_URL="http://localhost:3000"

RUN mkdir -p /data

EXPOSE 3000

CMD ["url-shortener"]
