FROM rust:1.26.2 as builder
WORKDIR /home/app
ARG DEBUG=0
ARG TAG=master
RUN rustup install nightly-2018-06-24 && \
    rustup default nightly-2018-06-24
RUN git clone -b ${TAG} https://github.com/AtsukiTak/bluumm-api /home/app
RUN [ ${DEBUG} -eq 0 ] && cargo build --release || cargo build
RUN [ ${DEBUG} -eq 0 ] && cp target/release/bluumm-api /usr/local/bin/ || cp target/debug/bluumm-api /usr/local/bin/

FROM rust:1.26.2
COPY --from=builder /usr/local/bin/bluumm-api /usr/local/bin/bluumm-api
ENTRYPOINT ["/usr/local/bin/bluumm-api"]
