FROM rust:latest as build

WORKDIR /build
COPY . .

RUN apt update
RUN apt install -y libssl-dev build-essential cmake
RUN apt install -y protobuf-compiler

RUN cargo build --release

FROM rust:latest 

WORKDIR /app
COPY .env ./
COPY --from=build /build/target/release/api ./

CMD ["./api"]