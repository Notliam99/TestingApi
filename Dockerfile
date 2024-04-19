FROM rust:latest as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM ubuntu:latest
WORKDIR /usr/src/binary
COPY --from=builder /usr/src/app/target/release/testing_api /usr/src/binary/testing_api
EXPOSE 8080
CMD ["/usr/src/binary/testing_api", "-p", "8080"]
