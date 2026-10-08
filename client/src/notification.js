(() => {
  if (window.__outlookNotificationObserver) return;
  const delivered = new WeakMap();
  const pending = new WeakSet();

  async function send(title, body) {
    const invoke = window.__TAURI__?.core?.invoke;
    if (!invoke) throw new Error("Tauri core API is unavailable");
    await invoke("send_notification", { title, body });
  }

  function scan(root = document) {
    const panes = [...root.querySelectorAll('[data-app-section="NotificationPane"]')];
    if (root.matches?.('[data-app-section="NotificationPane"]')) panes.push(root);
    const enclosingPane = root.closest?.('[data-app-section="NotificationPane"]');
    if (enclosingPane && !panes.includes(enclosingPane)) panes.push(enclosingPane);
    panes.forEach(pane => {
      pane.querySelectorAll('button, [role="button"]').forEach(card => {
        const data = window.extractNotificationData(card);
        if (!data?.valid) return;
        const fingerprint = JSON.stringify([data.name, data.title, data.preview]);
        if (delivered.get(card) === fingerprint || pending.has(card)) return;
        pending.add(card);
        send("New email from " + data.name, [data.title, data.preview].filter(Boolean).join("\n"))
          .then(() => {
            delivered.set(card, fingerprint);
          })
          .catch(error => {
            console.error("Outlook notification delivery failed:", error);
          })
          .finally(() => pending.delete(card));
      });
    });
  }

  const observer = new MutationObserver(records => {
    // Examine added nodes immediately, including toasts removed in the same
    // render batch. Unrelated Outlook mutations must not postpone detection.
    for (const record of records) {
      for (const node of record.addedNodes) {
        if (node.nodeType === Node.ELEMENT_NODE) {
          scan(node);
        }
      }
    }
    scan();
  });
  // Observe the document itself: Outlook can replace the root while booting.
  observer.observe(document, { childList: true, subtree: true, characterData: true, attributes: true, attributeFilter: ["role", "data-app-section", "aria-live"] });
  window.__outlookNotificationObserver = observer;
  // Retry failed IPC and discover cards already present when injected.
  setInterval(scan, 3000);
  scan();
  window.sendNotification = (name, title) => send("New email from " + name, title);
})();
