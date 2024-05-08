FROM rust:1.78.0 AS BUILDER

WORKDIR /app
RUN apt update && apt install -y lld clang
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

FROM debian:bookworm-slim AS RUNTIME

WORKDIR /app

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*


COPY --from=BUILDER /app/target/release/zero2prod zero2prod
COPY configuration configuration

ENV APP_ENV production

ENTRYPOINT [ "./zero2prod" ]