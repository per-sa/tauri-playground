<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const name = ref("");
const name2 = ref("");

async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg.value = await invoke("greet", { name: name.value });
}

async function calculate_comp() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg.value = await invoke("calculate_comp", { name: name.value, name2: name2.value });
}
</script>

<template>
    <div class="card">
        <input id="greet-input" v-model="name" placeholder="Enter your name..." />
        <input id="greet-input" v-model="name2" placeholder="Enter the person's name..." />
        <button type="button" @click="calculate_comp()">Calculate</button>
    </div>

    <p>{{ greetMsg }}</p>
</template>
