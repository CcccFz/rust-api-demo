FROM alpine:latest
ADD target/x86_64-unknown-linux-musl/release/rust-api-demo /app/
WORKDIR /app/
ENTRYPOINT ["./rust-api-demo"]
