<script setup lang="ts">
import { ref, onMounted } from "vue";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";

const chatMessages = ref<Array<{ sender: string; message: string; timestamp: string }>>([]);
const newMessage = ref("");
const chatPartner = ref("");

// 從 URL 參數獲取聊天對象名稱
onMounted(() => {
  const urlParams = new URLSearchParams(window.location.search);
  chatPartner.value = urlParams.get('partner') || '未知用戶';
  
  // 添加一條歡迎消息
  chatMessages.value.push({
    sender: "系統",
    message: `歡迎與 ${chatPartner.value} 開始聊天！`,
    timestamp: new Date().toLocaleTimeString()
  });
});

// 發送消息
function sendMessage() {
  if (newMessage.value.trim()) {
    chatMessages.value.push({
      sender: "我",
      message: newMessage.value,
      timestamp: new Date().toLocaleTimeString()
    });
    
    // 模擬對方回覆（demo 用途）
    setTimeout(() => {
      chatMessages.value.push({
        sender: chatPartner.value,
        message: `收到您的消息："${newMessage.value}"`,
        timestamp: new Date().toLocaleTimeString()
      });
    }, 1000);
    
    newMessage.value = "";
  }
}

// 關閉窗口
async function closeWindow() {
  const currentWindow = WebviewWindow.getCurrent();
  await currentWindow.close();
}
</script>

<template>
  <div class="chat-window">
    <!-- 標題欄 -->
    <div class="chat-header" data-tauri-drag-region>
      <h3>與 {{ chatPartner }} 聊天</h3>
      <button @click="closeWindow" class="close-btn">×</button>
    </div>
    
    <!-- 聊天區域 -->
    <div class="chat-messages">
      <div 
        v-for="(msg, index) in chatMessages" 
        :key="index" 
        :class="['message', msg.sender === '我' ? 'my-message' : 'other-message']"
      >
        <div class="message-header">
          <span class="sender">{{ msg.sender }}</span>
          <span class="timestamp">{{ msg.timestamp }}</span>
        </div>
        <div class="message-body">{{ msg.message }}</div>
      </div>
    </div>
    
    <!-- 輸入區域 -->
    <div class="chat-input">
      <form @submit.prevent="sendMessage" class="input-form">
        <input 
          v-model="newMessage" 
          placeholder="輸入消息..." 
          class="message-input"
        />
        <button type="submit" class="send-btn">發送</button>
      </form>
    </div>
  </div>
</template>

<style scoped>
.chat-window {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background-color: #f5f5f5;
}

.chat-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 20px;
  background-color: #2c3e50;
  color: white;
  user-select: none;
}

.chat-header h3 {
  margin: 0;
  font-size: 16px;
}

.close-btn {
  background: none;
  border: none;
  color: white;
  font-size: 20px;
  cursor: pointer;
  padding: 0;
  width: 30px;
  height: 30px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.close-btn:hover {
  background-color: rgba(255, 255, 255, 0.2);
}

.chat-messages {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.message {
  max-width: 70%;
  word-wrap: break-word;
}

.my-message {
  align-self: flex-end;
}

.other-message {
  align-self: flex-start;
}

.message-header {
  display: flex;
  justify-content: space-between;
  margin-bottom: 5px;
  font-size: 12px;
  color: #666;
}

.sender {
  font-weight: bold;
}

.timestamp {
  color: #999;
}

.message-body {
  padding: 10px 15px;
  border-radius: 15px;
  background-color: white;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.my-message .message-body {
  background-color: #007bff;
  color: white;
}

.chat-input {
  padding: 20px;
  background-color: white;
  border-top: 1px solid #ddd;
}

.input-form {
  display: flex;
  gap: 10px;
}

.message-input {
  flex: 1;
  padding: 10px 15px;
  border: 1px solid #ddd;
  border-radius: 25px;
  outline: none;
  font-size: 14px;
}

.message-input:focus {
  border-color: #007bff;
}

.send-btn {
  padding: 10px 20px;
  background-color: #007bff;
  color: white;
  border: none;
  border-radius: 25px;
  cursor: pointer;
  font-size: 14px;
}

.send-btn:hover {
  background-color: #0056b3;
}

/* 滾動條樣式 */
.chat-messages::-webkit-scrollbar {
  width: 6px;
}

.chat-messages::-webkit-scrollbar-track {
  background: #f1f1f1;
}

.chat-messages::-webkit-scrollbar-thumb {
  background: #c1c1c1;
  border-radius: 3px;
}

.chat-messages::-webkit-scrollbar-thumb:hover {
  background: #a8a8a8;
}
</style>