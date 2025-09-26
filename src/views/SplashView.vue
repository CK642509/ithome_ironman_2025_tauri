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
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  font-family: Arial, sans-serif;
}

h1 {
  margin-bottom: 2rem;
  font-size: 2.5rem;
  text-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
}

p {
  margin: 1rem 0;
  font-size: 1.2rem;
}

.loading-bar {
  width: 300px;
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
}
</style>


