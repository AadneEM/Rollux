FROM node:latest

RUN mkdir /app/

ADD . /app/.

RUN cd /app/ && npm install

RUN mkdir /data

WORKDIR /data/

CMD ["node", "/app/bot.js"]
