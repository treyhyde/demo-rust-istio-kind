FROM rustlang/rust:nightly-buster as builder

ADD src /app/src
ADD Cargo.toml /app

WORKDIR /app

RUN rustup default nightly
RUN cargo build --release

FROM ubuntu:rolling

COPY --from=builder /app/target/release/hello_person hello_person

RUN apt update && apt install tini -y tini

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080

ENTRYPOINT ["tini"]
CMD ["./hello_person"]
