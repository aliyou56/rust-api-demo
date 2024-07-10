FROM messense/rust-musl-cross:x86_64-musl AS builder
WORKDIR /rust-api-demo
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

# new stage with a minimal image
FROM scratch
COPY --from=builder /demo/target/x86_64-unknown-linux-musl/release/rust-api-demo /rust-api-demo
ENTRYPOINT ["/rust-api-demo"]
EXPOSE 3000
