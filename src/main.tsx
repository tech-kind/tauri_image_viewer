import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { setLocales } from "./setLocales";

const locale =
  (window.navigator.languages && window.navigator.languages[0]) ||
  window.navigator.language;

setLocales(locale);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
