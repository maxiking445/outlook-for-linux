const origCreateObjectURL = URL.createObjectURL;

let userTriggeredDownload = false;

//Listen for blob URLs being created to intercept downloads
URL.createObjectURL = function (blob) {
  const url = origCreateObjectURL(blob);

  const isPreview = !!document.querySelector("canvas");

  if (!isPreview || userTriggeredDownload) {
    (async () => {
      const arrayBuffer = await blob.arrayBuffer();
      const uint8Array = new Uint8Array(arrayBuffer);
      const base64 = btoa(String.fromCharCode(...uint8Array));

      if (blob.type !== "application/javascript") {
        window.__TAURI__.core
          .invoke("download", {
            payload: {
              type: blob.type,
              size: blob.size,
              name: blob.name || "download",
              data: base64,
            },
          })
          .catch(console.error);
      }
    })();
  }

  return url;
};

// Listen for user clicks on download buttons to pass through download
document.addEventListener("click", (e) => {
  const downloadButton = e.target.closest(
    'button[role="menuitem"][data-is-focusable="true"]',
  );
  if (downloadButton) {
    userTriggeredDownload = true;
    setTimeout(() => (userTriggeredDownload = false), 100);
  }
});
