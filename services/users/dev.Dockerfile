FROM rust:1.58.0-alpine

RUN apk add --no-cache musl-dev
RUN apk add --no-cache protoc

WORKDIR /usr/src/app
COPY . ./
RUN cargo build
CMD [ "cargo", "run" ]
