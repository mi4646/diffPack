import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import router from "./router";
// Import design system (replaces old main.css)
import "./assets/styles/design-system.css";
// Legacy styles - can be removed after full migration
import "./assets/styles/main.css";

const app = createApp(App);

app.use(createPinia());
app.use(router);

app.mount("#app");
