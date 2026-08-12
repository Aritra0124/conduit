FROM rust:1.97-alpine

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

CMD ["conduit"]

