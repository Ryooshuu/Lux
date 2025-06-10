<script setup lang="ts">
import { motion } from "motion-v";
import { onMounted, onUnmounted, ref, useTemplateRef } from "vue";

const container = useTemplateRef("container");
const height = ref<number | "auto">("auto");

let resizeObserver: ResizeObserver;

onMounted(() => {
    if (!container.value)
        return;

    resizeObserver = new ResizeObserver((entries) => {
        const observedHeight = entries[0]!.contentRect.height;
        height.value = observedHeight;
    });

    resizeObserver.observe(container.value);
});

onUnmounted(() => {
    resizeObserver.disconnect();
});
</script>

<template>
    <motion.div
        class="overflow-hidden"
        :style="{ height }"
        :animate="{ height }"
        :transition="{ type: 'spring', bounce: 0.4, visualDuration: 0.3 }"
    >
        <div ref="container">
            <slot />
        </div>
    </motion.div>
</template>
