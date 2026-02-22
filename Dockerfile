FROM rust:bookworm AS build
WORKDIR /usr/src/storage
COPY . .
RUN cargo build --release

FROM ubuntu:latest
COPY --from=build /usr/src/storage/target/release/chatbox-timeclock /usr/local/bin/chatbox-timeclock
WORKDIR /chatbox-timeclock
RUN apt-get update && apt-get install -y pkg-config openssl libssl-dev && apt-get install -y ca-certificates && update-ca-certificates && rm -rf /var/lib/apt/lists/*
CMD ["/usr/local/bin/chatbox-timeclock"]
