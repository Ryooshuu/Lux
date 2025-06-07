import { defineConfig } from "vite";

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
    clearScreen: false,
    server: {
        port: 1566,
        strictPort: true,
        host: host || false,
        hmr: host
            ? {
                protocol: "ws",
                host: host,
                port: 1421
            }
            : undefined,
        watch: {
            ignored: ["**/src-tauri/**"]
        }
    },
    build: {
        minify: true
    }
});
