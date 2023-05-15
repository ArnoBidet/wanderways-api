FROM rust:1.68 as builder
WORKDIR /usr/src/wanderways-api
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /usr/src/wanderways-api/target/release .
COPY --from=builder /usr/src/wanderways-api/Rocket.toml .

CMD ["/app/wanderways-api"]