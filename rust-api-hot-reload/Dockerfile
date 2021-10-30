# 開発環境
FROM rust:1.43.1 as develop-stage
WORKDIR /app
RUN cargo install cargo-watch
COPY . .

# ビルド環境
FROM develop-stage as build-stage
RUN cargo build --release

# 本番環境
FROM rust:1.43.1-slim-stretch
COPY --from=build-stage /app/target/release/api .
EXPOSE 8088
CMD ["/usr/local/bin/api"]
