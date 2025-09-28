<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}

// 創建或更新聊天窗口
async function createChatWindow() {
  if (!name.value.trim()) {
    alert("請先輸入聊天對象的名字！");
    return;
  }

  await invoke("open_chat_window", { partnerName: name.value });
  return;

  try {
    // 嘗試獲取已存在的聊天窗口
    const existingWindow = await WebviewWindow.getByLabel('chat');
    
    if (existingWindow) {
      // 如果窗口已存在，發送事件通知更新聊天對象
      await existingWindow.emit('change-chat-partner', {
        partner: name.value
      });
      
      // 聚焦到已存在的窗口
      await existingWindow.setFocus();
      console.log('Updated existing chat window with new partner:', name.value);
    } else {
      // 如果窗口不存在，創建新的聊天窗口
      const chatWindow = new WebviewWindow('chat', {
        url: `/chat?partner=${encodeURIComponent(name.value)}`,
        title: `與 ${name.value} 聊天`,
        width: 400,
        height: 600,
        resizable: true,
        minimizable: true,
        maximizable: true,
        closable: true,
        center: true,
        decorations: false, // 使用自定義標題欄
      });

      // 監聽窗口創建事件
      chatWindow.once('tauri://created', function () {
        console.log('Chat window created successfully');
      });

      // 監聽窗口錯誤事件
      chatWindow.once('tauri://error', function (e) {
        console.error('Error creating chat window:', e);
      });
    }
  } catch (error) {
    console.error('Error in createChatWindow:', error);
  }
}
</script>

<template>
  <main class="container">
    <h1>Welcome to Tauri + Vue</h1>

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
    
    <!-- 新增聊天室按鈕 -->
    <div class="chat-section">
      <h3>聊天功能</h3>
      <p>輸入聊天對象的名字，然後點擊按鈕創建聊天窗口</p>
      <button @click="createChatWindow" class="chat-btn">
        🗨️ 新增聊天室窗口
      </button>
    </div>
  </main>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

.chat-section {
  margin-top: 40px;
  padding: 20px;
  background-color: #f8f9fa;
  border-radius: 10px;
  text-align: center;
}

.chat-section h3 {
  color: #2c3e50;
  margin-bottom: 10px;
}

.chat-section p {
  color: #666;
  margin-bottom: 20px;
  font-size: 14px;
}

.chat-btn {
  background: linear-gradient(135deg, #007bff, #0056b3);
  color: white;
  border: none;
  padding: 12px 24px;
  border-radius: 25px;
  font-size: 16px;
  cursor: pointer;
  transition: all 0.3s ease;
  box-shadow: 0 4px 15px rgba(0, 123, 255, 0.3);
}

.chat-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(0, 123, 255, 0.4);
  background: linear-gradient(135deg, #0056b3, #004085);
}

.chat-btn:active {
  transform: translateY(0);
}
</style>