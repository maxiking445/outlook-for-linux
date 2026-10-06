(function () {
  if (window.__outlookUrlChangeHandler) return;

  const handleClick = (event) => {
    const anchor = event.target.closest?.("a[href]");
    if (!anchor) return;

    const linkUrl = anchor.href;
    try {
      if (!/^https?:$/i.test(new URL(linkUrl, window.location.href).protocol)) return;
    } catch {
      return;
    }

    const invoke = window.__TAURI__?.core?.invoke;
    if (!invoke) return;

    event.preventDefault();
    event.stopPropagation();
    invoke("open_in_browser", { url: linkUrl }).catch((err) =>
      console.error("Failed to open URL:", err)
    );
  };

  document.addEventListener(
    "click",
    handleClick,
    true
  );
  window.__outlookUrlChangeHandler = handleClick;
})();
