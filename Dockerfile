FROM rust:bookworm AS build
WORKDIR /usr/src/storage
COPY . .
RUN cargo build --release

FROM ubuntu:latest
COPY --from=build /usr/src/storage/target/release/chatbox-timeclock /usr/local/bin/chatbox-timeclock
CMD ["/usr/local/bin/chatbox-timeclock"]
