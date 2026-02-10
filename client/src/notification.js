console.log("Notification Observer loaded");
const seenNotifications = new Set();

// Every 5 seconds
setInterval(() => {
  const pane = document.querySelector('[data-app-section="NotificationPane"]');

  if (pane) {
    const firstChild = pane.firstElementChild;

    if (firstChild && firstChild.firstElementChild) {
      var nodeCLone =
        firstChild.firstElementChild.firstElementChild.cloneNode(true);

      const data = window.extractNotificationData(nodeCLone);
      const notificationId =
        `${data.name}_${data.title.substring(0, 50)}`.substring(0, 100);

      if (seenNotifications.has(notificationId)) {
        console.log(
          "Duplicate notification detected, skipping:",
          notificationId,
        );
        return;
      }
      seenNotifications.add(notificationId);

      const notifier = new TauriNotifier();
      notifier.send(data.name, data.title);
    }
  }

  console.log(`🔌 Tauri available: ${!!window.__TAURI__}`);
  console.log("─".repeat(50));
}, 1000);

console.log("Observer started!");

class TauriNotifier {
  constructor() {
    this.invoke = window.__TAURI__?.core?.invoke;
    if (!this.invoke) {
      return;
    }
  }

  async send(title, body) {
    if (!this.invoke) return false;
    try {
      await this.invoke("send_notification", { title, body });
      console.log(`Notification send: ${title}`);
      return true;
    } catch (error) {
      console.error("Invoke failed:", error);
      return false;
    }
  }
}

// devconsole: sendNotification('test', 'test')
window.sendNotification = function (name, title) {
  const notifier = new TauriNotifier();
  notifier.send("New Mail From: " + name, title);
};
