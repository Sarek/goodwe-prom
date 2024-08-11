FROM rust:alpine AS builder

RUN apk add musl-dev
WORKDIR /src
COPY . /src

RUN rustup update nightly && rustup default nightly
RUN cargo build --bins --release

FROM scratch

COPY --from=builder /src/target/release/goodwe-prom /

EXPOSE 8080/tcp

CMD ["/goodwe-prom"]
