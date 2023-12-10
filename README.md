# mtg-proxy

Simple web app for creating Magic the Gathering proxy cards.

![Screenshot](./docs/screenshot.png)

## Running

Using docker compose:

```sh
docker compose up --build
```

Using npm & cargo:

```sh
cd frontend
npm i
npm run build

cd ..
cargo run --release
```

## Technologies

- Frontend: React, Redux Toolkit, Vite
- Backend: Rust, axum, imageproc, lopdf
- External API: [Scryfall](https://scryfall.com/docs/api)
