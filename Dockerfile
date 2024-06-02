FROM rust:alpine3.20 as builder
WORKDIR /usr/src/sceawian

RUN apk add --no-cache musl-dev

RUN cargo init
COPY ./Cargo.toml ./Cargo.lock ./
RUN cargo build --release

RUN rm ./target/release/deps/sceawian*
RUN rm src/*.rs
COPY ./src ./src
RUN cargo build --release

FROM alpine:3.20 as runner
WORKDIR /usr/share/sceawian

RUN apk add --no-cache openssh-client git

COPY --from=builder /usr/src/sceawian/target/release/sceawian ./sceawian
CMD ./sceawian
