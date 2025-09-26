import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      redirect: "/splash"
    },
    {
      path: "/splash",
      name: "SplashPage",
      component: () => import("../views/SplashView.vue"),
    },
    {
      path: "/main",
      name: "MainPage",
      component: () => import("../views/MainView.vue"),
    },
  ],
});

export default router;