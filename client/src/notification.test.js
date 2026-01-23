import { test } from "node:test";
import assert from "node:assert/strict";
import { JSDOM } from "jsdom";
import { extractNotificationData } from "./notification-extractor.js";

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

  console.log("Button for test:", button);
  const result = extractNotificationData(button);

  console.log(result);
  assert.equal(result.name, "Surname Lastname");
  assert.equal(result.title, "MAIL_TITLE");
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
