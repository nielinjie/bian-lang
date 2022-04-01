import "react-reflex/styles.css";
import "./index.css";
import "./index.jsx";


import { appWindow } from "@tauri-apps/api/window";
appWindow.listen("tauri://close-requested", ({ event, payload }) => {
  appWindow.close();
  process.exit(0);
});