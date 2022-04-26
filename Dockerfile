FROM clux/muslrust AS builder
WORKDIR /volume
COPY . .
RUN cargo test
RUN cargo build --release

FROM alpine
COPY --from=builder /volume/target/x86_64-unknown-linux-musl/release/url-wrapper .
ENTRYPOINT [ "/url-wrapper" ]
