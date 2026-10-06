function extractNotificationData(card) {
  // Mail toast: avatar column, then sender, subject and preview.
  // Partial cards and nested action buttons must not throw.
  const content = card?.children?.[1];
  const name = content?.children?.[0]?.children?.[0]?.textContent?.trim() || "";
  const title = content?.children?.[1]?.textContent?.trim() || "";
  const preview = content?.children?.[2]?.textContent?.trim() || "";
  return {
    name: name || "Unknown Sender",
    title: title || "No Title",
    preview,
    valid: Boolean(content && content.children.length >= 2 && (name || title || preview)),
  };
}

if (typeof window !== "undefined") window.extractNotificationData = extractNotificationData;
