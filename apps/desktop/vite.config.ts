import { defineConfig, mergeConfig } from "vite";
import plugins from "./vite.plugins";
import tauri from "./vite.tauri";

export default defineConfig(mergeConfig(plugins, tauri));
