ARG RUST_VERSION=1.72.0
ARG APP_NAME=pragma-monitoring
FROM lukemathwalker/cargo-chef:latest-rust-${RUST_VERSION}-slim-bullseye AS cargo-chef
WORKDIR /app

FROM cargo-chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM cargo-chef AS builder
COPY --from=planner /app/recipe.json recipe.json

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
    libpq-dev \
    pkg-config \
    libssl-dev \ 
    ca-certificates \
    wget \
    && rm -rf /var/lib/apt/lists/*

RUN cargo chef cook --profile release --recipe-path recipe.json 
COPY . .
RUN cargo build --locked --release
RUN cp ./target/release/$APP_NAME /bin/server

FROM debian:bullseye-slim AS final
RUN apt-get update && DEBIEN_FRONTEND=noninteractive apt-get install -y \ 
    libpq-dev \
    libssl1.1 \
    procps \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /bin/server /bin/
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

RUN adduser \
    --no-log-init \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid 10001 \
    appuser
USER appuser

EXPOSE 8080

ENV RUST_LOG=info

CMD ["/bin/server"]