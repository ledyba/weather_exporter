FROM rust:alpine

WORKDIR /src
COPY . .

RUN cargo build --release

FROM alpine:3.12

WORKDIR /
COPY --from=build /src/target/release/weather_exporter weather_exporter

RUN ["chmod", "a+x", "/weather_exporter"]

EXPOSE 8080
CMD ["/weather_exporter", "--listen", ":8080"]
