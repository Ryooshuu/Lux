import { createApp } from "vue";
import { commands } from "@lux/tauri-types";
import { warn, debug, trace, info, error } from "@tauri-apps/plugin-log";

import App from "./App.vue";
import "./styles/index.css";
import "unfonts.css";

// #region console forwarding
function forwardConsole(
    fnName: "log" | "warn" | "error" | "info" | "debug",
    logger: (message: string) => Promise<void>
) {
    const original = console[fnName];
    console[fnName] = (message) => {
        original(message);
        logger(message);
    };
}

forwardConsole("log", trace);
forwardConsole("warn", warn);
forwardConsole("error", error);
forwardConsole("info", info);
forwardConsole("debug", debug);
// #endregion

// #region transparency fix

let dir = 1;
async function fixthisbullshittransparencyissue() {
    dir *= -1;
    await commands.fixTransparency(dir);
}

setInterval(() => {
    fixthisbullshittransparencyissue();
}, 1);

// #endregion

createApp(App)
    .mount("#app");

document.addEventListener("keydown", async (e) => {
    if (e.key === "Escape") {
        await commands.exitApp();
    }
});
