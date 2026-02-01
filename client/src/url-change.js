(function () {
  document.addEventListener(
    "click",
    (event) => {
      const anchor = event.target.closest("a[href]");
      if (!anchor) return;
      const linkUrl = anchor.href;
      window.__TAURI__?.core
        ?.invoke("open_in_browser", { url: linkUrl })
        .then(() => console.log("URL successfully opened:", linkUrl))
        .catch((err) => console.error("Failed to open URL:", err));
    },
    true
  );
})();
