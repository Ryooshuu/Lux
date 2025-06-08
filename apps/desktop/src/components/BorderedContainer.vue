<script setup lang="ts">
import { motion } from "motion-v";
import FluidContainer from "./FluidContainer.vue";

// todo : props
//        respect user theme
</script>

<template>
    <motion.div
        class="bordered-bg overflow-hidden"
        :initial="{ opacity: 0, height: 0, scale: 0.99 }"
        :exit="{ opacity: 0, height: 0, scale: 0.99 }"
        :animate="{ opacity: 1, height: 'auto', scale: 1 }"
        :transition="{ type: 'spring', visualDuration: 0.3, bounce: 0.4 }"
    >
        <FluidContainer>
            <slot />
        </FluidContainer>
    </motion.div>
</template>

<style>
.bordered-bg {
    /* background: hsla(0, 0%, 5%, 80%);
    border: solid 2px hsl(0, 0%, 10%);
    border-radius: 10px; */

    background: hsla(0, 0%, 95%, 80%);
    border: solid 2px hsl(0, 0%, 100%);
    border-radius: 10px;
}
</style>
