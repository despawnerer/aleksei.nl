FROM rust:1.55

RUN USER=root cargo new --bin aleksei
WORKDIR /aleksei

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/aleksei*
RUN cargo build --release

COPY ./resources ./resources

EXPOSE 3030
CMD ["./target/release/aleksei"]
