<script setup lang="ts">
import { motion } from "motion-v";
import FluidContainer from "./FluidContainer.vue";

// todo : props
//        respect user theme
</script>

<template>
    <motion.div
        class="bg-[rgba(26,26,26,0.8)] border-2 border-[rgb(26,26,26)] overflow-hidden rounded-[10px]"
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
