FROM rust:1.83 AS wasm

WORKDIR /app/wasm
RUN rustup update && \
    rustup target add wasm32-unknown-unknown && \
    cargo install wasm-pack
COPY wasm/Cargo.* ./
RUN mkdir -p src/ && \
    touch src/lib.rs && \
    wasm-pack build && \
    rm -r src/
COPY wasm/ ./
RUN touch src/lib.rs && wasm-pack build

FROM node:22.16 AS frontend

WORKDIR /app/frontend
ARG BASE_URL /
COPY frontend/package*.json ./
COPY --from=wasm /app/wasm/pkg /app/wasm/pkg
RUN npm ci
COPY frontend/ ./
RUN npm run build

FROM rust:1.83 AS backend

WORKDIR /app/backend
COPY backend/Cargo.* ./
RUN mkdir -p src/ && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -r src/
COPY backend/ ./
RUN touch src/main.rs && cargo build --release

FROM debian:12

WORKDIR /app
RUN apt-get update && apt-get -y install openssl ca-certificates
COPY --from=backend /app/backend/target/release/backend /app
COPY --from=frontend /app/frontend/dist /app/public

ENV PUBLIC_DIR=./public
ENV PORT=8080
ENV DATABASE_FILE=/app/data/database.db
EXPOSE 8080
VOLUME [ "/app/data" ]

CMD ["/app/backend"]
