import { test } from "node:test";
import assert from "node:assert/strict";
import { JSDOM } from "jsdom";
import { readFileSync } from "node:fs";
import { runInNewContext } from "node:vm";

// The application injects a classic script with window.eval, not an ES module.
const scriptWindow = {};
runInNewContext(readFileSync(new URL("./notification-extractor.js", import.meta.url), "utf8"), {
  window: scriptWindow,
  console,
});
const { extractNotificationData } = scriptWindow;

// HTML String for notification test
const testHTML = `
<button type="button">
  <div>
    <span aria-label="Username Avatar">
      <span>ML</span>
    </span>
  </div>
  <div>
    <div>
      <div>Surname Lastname</div>
      <div type="button">
        <span><i></i></span>
      </div>
    </div>
    <div>
      <div>
        <span>MAIL_TITLE</span>
      </div>
    </div>
    <div>MAIL_CONTENT</div>
  </div>
</button>`;


test("Notification Extractor - Standard Case", () => {
  const dom = new JSDOM(testHTML);
  const document = dom.window.document; 
  const button = document.querySelector("button");

  const result = extractNotificationData(button);

  assert.equal(result.name, "Surname Lastname");
  assert.equal(result.title, "MAIL_TITLE");
  assert.equal(result.preview, "MAIL_CONTENT");
});

test("Injected observer detects multiple cards and survives repeated injection", async () => {
  const dom = new JSDOM('<div data-app-section="NotificationPane"></div>', { runScripts: "outside-only" });
  const calls = [];
  dom.window.__TAURI__ = { core: { invoke: async (command, data) => {
    if (command === "send_notification") calls.push({ command, data });
  } } };
  const extractor = readFileSync(new URL("./notification-extractor.js", import.meta.url), "utf8");
  const observer = readFileSync(new URL("./notification.js", import.meta.url), "utf8");
  try {
    dom.window.eval(extractor);
    dom.window.eval(observer);
    dom.window.eval(observer);
    const pane = dom.window.document.querySelector("div");
    pane.innerHTML = testHTML + testHTML.replace("MAIL_TITLE", "SECOND_MAIL");
    await new Promise(resolve => setTimeout(resolve, 250));
    assert.equal(calls.length, 2);
    assert.equal(calls[0].command, "send_notification");
    assert.equal(calls[0].data.title, "Neue Mail von Surname Lastname");
    assert.equal(calls[0].data.body, "MAIL_TITLE\nMAIL_CONTENT");
    pane.appendChild(dom.window.document.createElement("span"));
    await new Promise(resolve => setTimeout(resolve, 250));
    assert.equal(calls.length, 2);
    // A later, separate mail with the same subject must not be lost.
    pane.insertAdjacentHTML("beforeend", testHTML);
    await new Promise(resolve => setTimeout(resolve, 250));
    assert.equal(calls.length, 3);
    // A complete toast pane can disappear before the next timer tick.
    const shortToast = dom.window.document.createElement("div");
    shortToast.setAttribute("data-app-section", "NotificationPane");
    shortToast.innerHTML = testHTML.replace("MAIL_TITLE", "SHORT_TOAST");
    dom.window.document.body.appendChild(shortToast);
    shortToast.remove();
    await new Promise(resolve => setTimeout(resolve, 30));
    assert.equal(calls.length, 4);
    assert.equal(calls[3].data.body, "SHORT_TOAST\nMAIL_CONTENT");
  } finally {
    dom.window.close();
  }
});

test("Partial cards and action buttons do not throw or produce mail", () => {
  const dom = new JSDOM('<button><span>Dismiss</span></button>');
  assert.equal(extractNotificationData(dom.window.document.querySelector("button")).valid, false);
  dom.window.close();
});

test("Notification Extractor - Missing Name", () => {
  const dom = new JSDOM(testHTML.replace("Surname Lastname", ""));
  const button = dom.window.document.querySelector("button");

  const result = extractNotificationData(button);

  assert.equal(result.name, "Unknown Sender");
  assert.equal(result.title, "MAIL_TITLE");
});

test("Notification Extractor - Missing Title", () => {
  const dom = new JSDOM(testHTML.replace("MAIL_TITLE", ""));
  const button = dom.window.document.querySelector("button");

  const result = extractNotificationData(button);

  assert.equal(result.name, "Surname Lastname");
  assert.equal(result.title, "No Title");
});

test("Notification Extractor - Empty Content", () => {
  const emptyHTML = testHTML
    .replace("Surname Lastname", "")
    .replace("MAIL_TITLE", "");
  const dom = new JSDOM(emptyHTML);
  const button = dom.window.document.querySelector("button");

  const result = extractNotificationData(button);

  assert.equal(result.name, "Unknown Sender");
  assert.equal(result.title, "No Title");
});
