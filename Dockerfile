FROM rust:1.78.0

WORKDIR /app
RUN apt update && apt install -y lld clang
COPY . .
ENV SQLX_OFFLINE true
ENV APP_ENV production
RUN cargo build --release
ENTRYPOINT [ "./target/release/zero2prod" ]