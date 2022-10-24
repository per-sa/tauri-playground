<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const crypto_name = ref("");

async function calculate_comp() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg.value = await invoke("calculate_comp", { crypto_name: crypto_name.value });
}
</script>

<template>
    <div class="card">
        <input id="greet-input" v-model="crypto_name" placeholder="Enter your name..." />
        <button type="button" @click="calculate_comp()">Calculate</button>
    </div>

    <p>{{ greetMsg }}</p>
</template>
