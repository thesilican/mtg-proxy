FROM node:lts as frontend

WORKDIR /root
ARG BASE_URL /
COPY frontend/package*.json ./
RUN npm ci
COPY frontend/ ./
RUN npm run build

FROM rust:1.74 as backend

WORKDIR /root
ARG PUBLIC_DIR=public
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src/bin && \
    touch src/lib.rs && \
    echo "fn main() {}" > src/bin/backend.rs && \
    cargo build --release && \
    rm -r src/
COPY src/ src/
RUN touch src/lib.rs && cargo build --release --bin backend

FROM debian
WORKDIR /root
RUN apt-get update && apt-get -y install openssl ca-certificates
COPY --from=backend /root/target/release/backend ./backend
COPY --from=frontend /root/dist ./public

ENV PORT 8080
EXPOSE 8080
CMD ["/root/backend"]
