import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "LandingPage",
      component: () => import("../App.vue"),
    },
    {
      path: "/",
      name: "SplashPage",
      component: () => import("../views/SplashView.vue"),
    },
  ],
});

export default router;