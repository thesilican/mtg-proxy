FROM rust:1.76 AS wasm

WORKDIR /root/wasm
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

FROM node:lts AS frontend

WORKDIR /root/frontend
ARG BASE_URL /
COPY frontend/package*.json ./
COPY --from=wasm /root/wasm/pkg /root/wasm/pkg
RUN npm ci
COPY frontend/ ./
RUN npm run build

FROM rust:1.76 AS backend

WORKDIR /root/backend
COPY backend/Cargo.* ./
RUN mkdir -p src/ && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -r src/
COPY backend/ ./
RUN touch src/main.rs && cargo build --release

FROM debian

WORKDIR /root
RUN apt-get update && apt-get -y install openssl ca-certificates
COPY --from=backend /root/backend/target/release/backend /root
COPY --from=frontend /root/frontend/dist /root/public

ENV PUBLIC_DIR ./public
ENV PORT 8080
EXPOSE 8080
VOLUME [ "/root/data" ]

CMD ["/root/backend"]
