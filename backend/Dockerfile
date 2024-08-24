FROM rust:1.80-slim-bullseye as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim as release
RUN apt-get update && apt-get install -y libssl1.1 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/DevTrackr /usr/local/bin/DevTrackr
CMD ["DevTrackr"]