<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Store } from "@tauri-apps/plugin-store";

const greetMsg = ref("");
const name = ref("");
const greetName = ref("");
let store: Store;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
  greetName.value = name.value;
  
  // // 也可以直接在前端儲存姓名到 store
  // if (store) {
  //   await store.set("name", name.value);
  //   await store.save();
  // }
}

// 在元件掛載時載入 store 中的姓名
onMounted(async () => {
  // 載入 store (也可以不給參數，預設會是 store.json)
  store = await Store.load("user-data.json");
  
  // 檢查是否有儲存的姓名
  const storedName = await store.get<string>("name");
  if (storedName) {
    greetName.value = storedName;
  }
});
</script>

<template>
  <main class="container">
    <h1>Welcome to Tauri + Vue</h1>
    <h2 v-if="greetName !== ''">Welcome back~ {{ greetName }}~</h2>

    <div class="row">
      <a href="https://vite.dev" target="_blank">
        <img src="/vite.svg" class="logo vite" alt="Vite logo" />
      </a>
      <a href="https://tauri.app" target="_blank">
        <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
      </a>
      <a href="https://vuejs.org/" target="_blank">
        <img src="../assets/vue.svg" class="logo vue" alt="Vue logo" />
      </a>
    </div>
    <p>Click on the Tauri, Vite, and Vue logos to learn more.</p>

    <form class="row" @submit.prevent="greet">
      <input id="greet-input" v-model="name" placeholder="Enter a name..." />
      <button type="submit">Greet</button>
    </form>
    <p>{{ greetMsg }}</p>
  </main>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>