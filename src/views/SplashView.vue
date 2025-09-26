<template>
  <div class="splash-container">
    <h1>歡迎使用 Tauri 應用程式</h1>
    <p>Loading...</p>
    <div class="loading-bar">
      <div class="loading-progress" :style="{ width: progress + '%' }"></div>
    </div>
    <p>{{ progress }}%</p>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useRouter } from "vue-router";
import { getCurrentWindow, Window } from "@tauri-apps/api/window";

const router = useRouter();
const progress = ref(0);

onMounted(async () => {
  // 模擬載入進度
  const interval = setInterval(() => {
    progress.value += 1;
    if (progress.value >= 100) {
      clearInterval(interval);
    }
  }, 100); // 10 秒內從 0% 到 100%

  // 10 秒後關閉 splash screen 並顯示 main window
  setTimeout(async () => {
    try {
      // 嘗試獲取並顯示 main window
      const mainWindow = new Window("main");
      await mainWindow.show();
      await mainWindow.setFocus();
      
      // 等待一下確保 main window 完全顯示後再關閉 splash
      setTimeout(async () => {
        try {
          const currentWindow = getCurrentWindow();
          await currentWindow.close();
        } catch (closeError) {
          console.error("關閉 splash screen 時發生錯誤:", closeError);
        }
      }, 100);
      
    } catch (error) {
      console.error("切換 window 時發生錯誤:", error);
      // 如果失敗，使用路由跳轉作為備用方案
      router.push("/main");
    }
  }, 10000); // 10 秒
});
</script>

<style scoped>
.splash-container {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  height: 100vh;
  width: 100%;
  background: rgba(102, 126, 234, 0.85);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  color: white;
  font-family: Arial, sans-serif;
  /* border-radius: 10px; */
  border: 1px solid rgba(255, 255, 255, 0.2);
  box-shadow: 0 8px 32px 0 rgba(31, 38, 135, 0.37);
  padding: 20px;
  box-sizing: border-box;
  overflow: hidden;
}

h1 {
  margin-bottom: 1.5rem;
  font-size: 1.8rem;
  text-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
  text-align: center;
}

p {
  margin: 0.8rem 0;
  font-size: 1rem;
  text-align: center;
}

.loading-bar {
  width: 250px;
  height: 6px;
  background-color: rgba(255, 255, 255, 0.3);
  border-radius: 3px;
  overflow: hidden;
  margin: 1rem 0;
}

.loading-progress {
  height: 100%;
  background-color: #4CAF50;
  transition: width 0.3s ease;
  border-radius: 3px;
  box-shadow: 0 0 10px rgba(76, 175, 80, 0.5);
}
</style>

<style>
/* 全局樣式，移除捲軸和預設邊距 */
html, body {
  margin: 0;
  padding: 0;
  overflow: hidden;
  height: 100%;
  width: 100%;
}

#app {
  height: 100vh;
  overflow: hidden;
}
</style>


