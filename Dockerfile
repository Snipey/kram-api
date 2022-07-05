FROM ekidd/rust-musl-builder AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM alpine

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/kram-api-new /app/kram
EXPOSE 8080
CMD ["/bin/sh", "-c", "/app/kram"]
# CMD ["/bin/sh"]