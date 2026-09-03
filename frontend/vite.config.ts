import process from "node:process";
import { vanillaExtractPlugin } from "@vanilla-extract/vite-plugin";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";
import topLevelAwait from "vite-plugin-top-level-await";
import wasm from "vite-plugin-wasm";

// https://vitejs.dev/config/
export default defineConfig({
	plugins: [react(), vanillaExtractPlugin(), wasm()],
	worker: {
		format: "iife",
		plugins: () => [topLevelAwait(), wasm()],
	},
	server: {
		proxy: {
			"/api": {
				target: "http://localhost:8080",
				changeOrigin: true,
			},
		},
	},
	base: process.env.BASE_URL ?? "/",
});
