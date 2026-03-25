import { createRouter, createWebHistory } from "vue-router";
import Home from "@/views/Home.vue";

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: "/",
            name: "home",
            component: Home,
        },
        {
            path: "/local",
            name: "local",
            component: () => import("@/views/LocalAnalysis.vue"),
        },
        {
            path: "/settings",
            name: "settings",
            component: () => import("@/views/Settings.vue"),
        },
    ],
});

export default router;
