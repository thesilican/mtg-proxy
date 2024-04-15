# mtg-proxy

Simple web app for creating Magic the Gathering proxy cards.

![Screenshot](./docs/screenshot.png)

## Running

Using docker compose:

```sh
docker compose up --build
```

Using wasm-pack & npm:

```sh
cd wasm
wasm-pack build

cd ../frontend
npm install
npm run build
```

## Technologies

- Frontend: React, Redux Toolkit, Vite
- Wasm: Rust, imageproc, lopdf, wasm-pack
- External API: [Scryfall](https://scryfall.com/docs/api)
