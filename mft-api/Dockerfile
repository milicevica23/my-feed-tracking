FROM rust:1.71


WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .

CMD ["mft-api"]