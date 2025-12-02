import assert from "assert";
import fs from "fs";

const input = fs.readFileSync(import.meta.dir + "/input.txt", "utf8");
const ranges = input.trim().split(",");

const invalidIds = [];

for (const range of ranges) {
  const [start, end] = range.split("-").map((n) => parseInt(n));

  if (!start) assert.fail("Not valid start");
  if (!end) assert.fail("Not valid end");

  for (let id = start; id <= end; id++) {
    const idStr = id.toString();

    const halfLength = idStr.length / 2;
    if (halfLength !== Math.floor(halfLength)) continue;

    const first = idStr.slice(0, halfLength);
    const second = idStr.slice(halfLength);
    if (first !== second) continue;

    invalidIds.push(id);
  }
}

const total = invalidIds.reduce((a, b) => a + b, 0);

console.log(total);
