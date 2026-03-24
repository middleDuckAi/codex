import assert from "node:assert/strict";
import test from "node:test";

import { setCodexProcessTitle } from "../bin/process_title.js";

test("setCodexProcessTitle sets the launcher title to codex", () => {
  const originalTitle = process.title;

  try {
    process.title = "node";
    setCodexProcessTitle();
    assert.equal(process.title, "codex");
  } finally {
    process.title = originalTitle;
  }
});
