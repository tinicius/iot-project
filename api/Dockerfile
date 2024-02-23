FROM rust:latest as build

WORKDIR /build
COPY . .

RUN apt update \
    && apt install -y libssl-dev build-essential cmake \
    && apt install -y protobuf-compiler

RUN cargo build --release

FROM rust:slim-bookworm

WORKDIR /app
COPY .env ./
COPY --from=build /build/target/release/api ./

CMD ["./api"]