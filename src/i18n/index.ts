import { createI18n } from "vue-i18n";
import en from "./locales/en.json";
import ja from "./locales/ja.json";

// Get initial locale from settings or browser
function getInitialLocale(): string {
  // Default to English, will be overridden by settings in App.vue
  return "en";
}

const i18n = createI18n({
  legacy: false,
  locale: getInitialLocale(),
  fallbackLocale: "en",
  messages: {
    en,
    ja,
  },
});

export default i18n;
