FROM rust:1.26.2 as builder
WORKDIR /home/build
RUN rustup install nightly && \
COPY . /home/build
RUN rustup run nightly cargo build

FROM debian:jessie
COPY --from=builder /home/build/target/debug/bitblossom-api /usr/local/bin/bitblossom-api
ENTRYPOINT ["bitblossom-api"]
