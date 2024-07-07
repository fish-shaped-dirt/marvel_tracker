import { createApp } from "vue";
import App from "./App.vue";
import router from "./router/index.ts";

import "./css/app.css";
import "./css/NavigationBar.css";

createApp(App).use(router).mount("#app");
