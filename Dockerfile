FROM ekidd/rust-musl-builder:nightly-2020-11-19 AS builder
WORKDIR /home/rust/src
COPY . .
RUN cargo build --release


FROM alpine:latest
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/rust-auth-server /app/getty
EXPOSE 8000
CMD /app/getty