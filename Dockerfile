FROM rust:1.59.0 as build-env
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM gcr.io/distroless/cc
LABEL org.opencontainers.image.source=https://github.com/GiganticMinecraft/SeichiRankingBFF
COPY --from=build-env /app/target/release/seichi-api /
CMD ["./seichi-api"]
