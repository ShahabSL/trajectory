import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

const tauriPlatform = process.env.TAURI_ENV_PLATFORM;

export default defineConfig({
  plugins: [react()],
  base: "./",
  clearScreen: false,
  server: {
    host: "127.0.0.1",
    port: 1420,
    strictPort: true,
  },
  build: {
    target: tauriPlatform === "windows" ? "chrome105" : "safari13",
  },
});
