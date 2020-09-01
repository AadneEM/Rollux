ARG BASE_IMAGE=ekidd/rust-musl-builder:latest

FROM ${BASE_IMAGE} AS builder

ADD --chown=rust:rust . ./

RUN cargo build --release

FROM alpine:latest

RUN apk --no-cache add ca-certificates

COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/rollux /usr/local/bin/

ENV DISCORD_TOKEN="invalid"

CMD /usr/local/bin/rollux
