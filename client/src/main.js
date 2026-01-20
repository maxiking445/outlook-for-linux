import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from "@tauri-apps/plugin-notification";
import { invoke } from "@tauri-apps/api/core";

await invoke("send_notification", {
  title: "Outlook",
  body: "Neue Nachricht!",
});


