import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";

// 本番ビルドのみ右クリック・コピー禁止
if (!import.meta.env.DEV) {
  document.addEventListener("contextmenu", (e) => e.preventDefault());
  document.addEventListener("copy", (e) => e.preventDefault());
}

createApp(App).mount("#app");
