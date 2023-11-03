FROM rust:1.73-alpine
ARG SERVER_PORT=3000
ENV PORT=$SERVER_PORT

WORKDIR /app

RUN apk update && \
    apk upgrade
RUN apk add --no-cache musl-dev

RUN cargo install cargo-watch

COPY Cargo.* ./
COPY src ./src

RUN cargo build

EXPOSE ${PORT}
