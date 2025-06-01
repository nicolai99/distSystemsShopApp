FROM rust:1.85.1-slim-bullseye

# View app name in Cargo.toml
ARG APP_NAME=shop

WORKDIR /app

COPY Cargo.toml ./
RUN mkdir src \
    && echo "// dummy file" > src/lib.rs \
    && cargo build --release

COPY src src
RUN cargo build --locked --release
RUN cp ./target/release/$APP_NAME /bin/server

CMD ["/bin/server"]
WORKDIR /app

COPY . ./