import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "MainPage",
      component: () => import("../views/MainView.vue"),
    },
    {
      path: "/chat",
      name: "ChatWindow",
      component: () => import("../views/ChatWindow.vue"),
    },
  ],
});

export default router;