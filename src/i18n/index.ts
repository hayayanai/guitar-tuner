import { createI18n } from "vue-i18n";
import type { MessageSchema } from "./schema";
import en from "./locales/en.json";
import ja from "./locales/ja.json";

const i18n = createI18n<[MessageSchema], "en" | "ja">({
  legacy: false,
  locale: "en",
  fallbackLocale: "en",
  messages: {
    en,
    ja,
  },
});

export default i18n;
export type { MessageSchema };

// Type for useI18n composition API
export type I18n = typeof i18n;
