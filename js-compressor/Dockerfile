
FROM node:lts-bookworm-slim

WORKDIR /app

COPY package*.json ./


RUN npm install

COPY . .

ENTRYPOINT ["node", "cli.js"]

