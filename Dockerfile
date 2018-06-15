FROM rust:1.26.2
WORKDIR /home/app
RUN rustup install nightly-2018-06-01 && \
    rustup default nightly-2018-06-01
COPY . /home/app
RUN cargo build --release
ENTRYPOINT ["/home/app/target/release/bitblossom-api"]
