FROM rust:1.26.2 as builder
WORKDIR /home/build
RUN rustup install nightly-2018-06-01
COPY . /home/build
RUN rustup run nightly cargo build

FROM rust:1.26.2
COPY --from=builder /home/build/target/debug/bitblossom-api /usr/local/bin/bitblossom-api
ENTRYPOINT ["bitblossom-api"]
