FROM rust:1.82-slim-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY capstones/capstone-07-task-management-api/solution/Cargo.toml capstones/capstone-07-task-management-api/solution/Cargo.toml
COPY capstones/capstone-07-task-management-api/solution/src/ capstones/capstone-07-task-management-api/solution/src/

# Create dummy sources for workspace members so cargo can resolve the workspace
RUN for member in $(grep -oP '"[^"]*"' Cargo.toml | tr -d '"' | grep -v capstone-07-task-management-api-solution); do \
    mkdir -p "$(dirname "$member")/src" && echo 'fn main() {}' > "$member/src/main.rs" \
    || echo "fn main() {}" > "$member/src/lib.rs"; \
done

RUN cargo build --release -p capstone-07-task-management-api-solution

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/capstone-07-task-management-api-solution /usr/local/bin/task-api

EXPOSE 3000

ENV DATABASE_URL="sqlite:data.db?mode=rwc"
ENV JWT_SECRET="change-me-in-production"
ENV PORT=3000

VOLUME ["/data"]
WORKDIR /data

CMD ["task-api"]
