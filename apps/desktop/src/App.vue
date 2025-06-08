<script setup lang="ts">
import { computed, onMounted, ref, useTemplateRef } from "vue";
import BorderedContainer from "./components/BorderedContainer.vue";
import { AnimatePresence } from "motion-v";
import { useMagicKeys, whenever } from "@vueuse/core";
import { commands } from "@lux/tauri-types";
import { info } from "@tauri-apps/plugin-log";
import { listen } from "@tauri-apps/api/event";

const placeholderString = computed(() => {
    const options = [
        "Type to search files...",
        "Type to search applications...",
        "Type to execute commands...",
        "Type '1+2'..."
    ];

    return options[Math.floor(Math.random() * options.length)];
});

const query = ref("");
const visible = ref(true);

const input = useTemplateRef("input");

// todo : holy fuck this is a mess
const keys = useMagicKeys();
whenever(keys.escape!, () => {
    visible.value = false;
});

function show() {
    visible.value = true;
    setTimeout(() => {
        input.value?.focus();
    }, 10);
}

// todo : remove this
whenever(keys.shift_escape!, show);

onMounted(() => {
    input.value?.focus();
});

function exitCompleted() {
    info("[vue] toggled visibility -> false");
    commands.toggleVisibility(false);
}

listen("app_shown", () => {
    info("[vue] toggled visibility -> true");
    commands.toggleVisibility(true);
    show();
});

</script>

<template>
    <div class="px-4 w-full my-32 flex justify-center">
        <div class="w-full max-w-3xl flex flex-col gap-3">
            <AnimatePresence @exit-complete="exitCompleted">
                <BorderedContainer v-if="visible">
                    <div class="px-4 min-h-[50px] flex items-center">
                        <input
                            ref="input"
                            v-model="query"
                            class="w-full outline-none"
                            type="text"
                            :placeholder="placeholderString"
                        >
                    </div>
                </BorderedContainer>
            </AnimatePresence>

            <AnimatePresence>
                <BorderedContainer v-if="query && visible">
                    <div class="p-2.5">
                        <p>{{ query }}</p>
                    </div>
                </BorderedContainer>
            </AnimatePresence>
        </div>
    </div>
</template>

<style>
/* body {
    background: rgb(0, 0, 0);
} */
</style>
