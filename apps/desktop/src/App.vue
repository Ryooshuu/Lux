<script setup lang="ts">
import { computed, ref } from "vue";
import BorderedContainer from "./components/BorderedContainer.vue";
import { AnimatePresence } from "motion-v";

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
</script>

<template>
    <div class="px-4 w-full my-32 font-[Inter] flex justify-center">
        <div class="w-full max-w-3xl flex flex-col gap-3">
            <BorderedContainer>
                <div class="p-2.5">
                    <input
                        v-model="query"
                        class="w-full outline-none text-white placeholder:text-[rgba(255,255,255,0.25)]"
                        type="text"
                        :placeholder="placeholderString"
                    >
                </div>
            </BorderedContainer>

            <AnimatePresence>
                <BorderedContainer v-if="query">
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
    background: rgb(100, 0, 0);
} */
</style>
