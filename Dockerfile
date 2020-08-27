FROM debian

RUN apt update
RUN apt install libssl1.1 openssl ca-certificates

ADD target/release/rollux /bin/.

RUN mkdir /app/

WORKDIR /app/

ENV DISCORD_TOKEN="invalid"

CMD '/bin/rollux'