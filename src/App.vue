<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Love from "./components/Love.vue";
</script>

<template>
  <div class="container">
    <h1>Welcome, {{ user }} </h1>

    <div class="row">

      <img src="./assets/cryptocurrency.png" class="logo vite" alt="Tauri logo" />

    </div>

    <p>Check your love compatibility with your signifcant Oother.</p>

    <Love />

  </div>
</template>

<style scoped>
.logo.vite {
  filter: drop-shadow(0 0 2em #ff7474);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>

<script lang="ts">
export default {
  data() {
    return {
      user: "clmb",
    };
  },
};
</script>