import { createRouter, createWebHistory } from "vue-router";
import LocalAnalysis from "@/views/LocalAnalysis.vue";

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: "/",
            name: "local",
            component: LocalAnalysis,
        },
        {
            path: "/remote",
            name: "remote",
            component: () => import("@/views/RemoteAnalysis.vue"),
        },
        {
            path: "/settings",
            name: "settings",
            component: () => import("@/views/Settings.vue"),
        },
    ],
});

export default router;
