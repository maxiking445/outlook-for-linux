 function extractNotificationData(button) {
  const name = extractName(button);

  const title = extractTitle(button);

  const data = { name, title };
  console.table(data);

  return data;
}

function extractName(button) {
  const firstDiv = button.children[1].children[0].children[0];
  const text = firstDiv?.textContent?.trim();
  return text || "Unknown Sender";
}

function extractTitle(button) {
  const titleContainer = button.children[1].children[1].children[0].children[0];
  const text = titleContainer?.textContent?.trim();
  return text || "No Title";
}

if (typeof window !== "undefined") {
  window.extractNotificationData = extractNotificationData;
}

if (typeof module !== "undefined" && module.exports) {
  module.exports = { extractNotificationData };
}