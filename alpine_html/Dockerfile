FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN ./bin/init-tailwind-linux && ./bin/tailwindcss -i styles/tailwind.css -o assets/main.css
RUN cargo build --release --bin alpine_html

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/alpine_html /usr/local/bin
COPY --from=builder /app/assets ./assets
ENTRYPOINT ["/usr/local/bin/alpine_html"]
