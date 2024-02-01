import i18next from "i18next";

import en from "../src-tauri/locales/en.json";
import ja from "../src-tauri/locales/ja.json";

export const setLocales = (locale: string) => {
  i18next.init({
    lng: locale,
    fallbackLng: "en",
    resources: {
      en: { translation: en },
      ja: { translation: ja },
    },
  });
};
