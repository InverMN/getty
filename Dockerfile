FROM ekidd/rust-musl-builder:nightly-2020-11-19 AS builder
WORKDIR /home/rust/src
COPY . .
RUN cargo build --release
RUN ls target/x86_64-unknown-linux-musl/release


FROM alpine:latest
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release /app
EXPOSE 8000
CMD /app/rust-auth-server