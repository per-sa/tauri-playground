<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const newlines = ref("");
const name = ref("");

async function get_crypto() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg.value = await invoke("get_crypto", { name: name.value });

    // separates for new lines
    let newlines = greetMsg.value.split(",").join("\n");
}
</script>

<template>
    <div class="card">
        <input id="greet-input" v-model="name" placeholder="Enter crypto name..." />
        <button type="button" @click="get_crypto()">Calculate</button>
    </div>

    <p id="response">{{ greetMsg }}</p>
</template>

<style>
#response {
    font-size: 1.5em;
    font-weight: bold;
    color: #ff7474;
}
</style>