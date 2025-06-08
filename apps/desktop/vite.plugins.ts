import { defineConfig } from "vite";
import tailwindcss from "@tailwindcss/vite";
import vue from "@vitejs/plugin-vue";
import autoImport from "unplugin-auto-import/vite";
import unfonts from "unplugin-fonts/vite";
import tsconfigPaths from "vite-tsconfig-paths";
import vueDevTools from "vite-plugin-vue-devtools";

export default defineConfig({
    plugins: [
        tsconfigPaths(),
        vueDevTools(),

        tailwindcss(),
        vue(),
        unfonts({
            google: {
                families: ["Noto Sans"]
            }
        }),
        autoImport({
            dts: true,
            vueTemplate: true,
            imports: [
                "vue"
            ],
            eslintrc: {
                enabled: true
            }
        })
    ]
});
