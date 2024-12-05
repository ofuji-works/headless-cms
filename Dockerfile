FROM rust:1.82-slim-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
RUN adduser app && chown -R app /app
USER app
COPY --from=builder /app/target/release/app /app/headless-cms
ENV PORT 8080
EXPOSE $PORT
ENTRYPOINT ["./headless-cms"]
