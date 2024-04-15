FROM rust:1.76 as wasm

WORKDIR /root/wasm
RUN rustup update && \
    rustup target add wasm32-unknown-unknown && \
    cargo install wasm-pack
COPY wasm/Cargo.toml wasm/Cargo.lock ./
RUN mkdir -p src/ && \
    touch src/lib.rs && \
    wasm-pack build && \
    rm -r src/
COPY wasm/ ./
RUN touch src/lib.rs && wasm-pack build

FROM node:lts as frontend

WORKDIR /root/frontend
ARG BASE_URL /
COPY frontend/package*.json ./
COPY --from=wasm /root/wasm/pkg /root/wasm/pkg
RUN npm ci
COPY frontend/ ./
RUN npm run build

FROM thesilican/httpd

COPY --from=frontend /root/frontend/dist /public
