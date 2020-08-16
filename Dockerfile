FROM ekidd/rust-musl-builder as builder

WORKDIR /home/rust/src
COPY . .

RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:3.12

WORKDIR /

COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/weather_exporter weather_exporter

RUN apk add --no-cache ca-certificates && update-ca-certificates

ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR=/etc/ssl/certs

RUN ["chmod", "a+x", "/weather_exporter"]

EXPOSE 8080
ENTRYPOINT ["/weather_exporter"]
