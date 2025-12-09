import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";

// 右クリック・コピー無効化
document.addEventListener("contextmenu", (e) => e.preventDefault());
document.addEventListener("copy", (e) => e.preventDefault());

createApp(App).mount("#app");
