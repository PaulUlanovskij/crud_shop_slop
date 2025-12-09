FROM clux/muslrust:1.88.0-stable AS builder

WORKDIR /app
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release --target x86_64-unknown-linux-musl


FROM rust:1.88.0-alpine AS runtime

WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/slopshop slopshop
COPY configuration configuration
COPY index.html index.html
COPY style.css style.css
COPY index.js index.js
ENV APP_ENVIROMENT production
ENTRYPOINT ["./slopshop"]
