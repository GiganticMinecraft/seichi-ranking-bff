# syntax=docker/dockerfile:1.4
FROM lukemathwalker/cargo-chef:latest-rust-1.68.2 AS chef
WORKDIR /app

FROM chef AS planner
COPY --link . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS build-env 
COPY --from=planner --link /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY --link . .
RUN cargo build --release

FROM gcr.io/distroless/cc
LABEL org.opencontainers.image.source=https://github.com/GiganticMinecraft/seichi-ranking-bff
COPY --from=build-env --link /app/target/release/seichi-ranking-bff /
CMD ["./seichi-ranking-bff"]
