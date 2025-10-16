# Stage 1: Build
FROM rust:1.81 AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# Stage 2: Run
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/tcgapi-service-mtg .
EXPOSE 8000
CMD ["./tcgapi-service-mtg"]

