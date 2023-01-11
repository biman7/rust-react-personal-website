import "dotenv/config";
import path from "path";
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vitejs.dev/config/
export default defineConfig(({ command }) => ({
  plugins: [react()],
  base: command === "serve" ? "" : "/dist/",
  publicDir: "fake_dir_so_nothing_gets_copied",
  build: {
    manifest: true,
    outDir: path.resolve(__dirname, "public/dist"),
    rollupOptions: {
      input: [
        path.resolve(__dirname, "src/main.jsx"),
        path.resolve(__dirname, "src/main.css"),
      ],
    },
  },
}));
