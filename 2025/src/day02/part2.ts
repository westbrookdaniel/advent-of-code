import assert from "assert";
import fs from "fs";

function chunk<T>(array: T[], size: number): T[][] {
  const result = [];
  for (let i = 0; i < array.length; i += size) {
    result.push(array.slice(i, i + size));
  }
  return result;
}

const input = fs.readFileSync(import.meta.dir + "/input.txt", "utf8");
const ranges = input.trim().split(",");

const invalidIds: number[] = [];

for (const range of ranges) {
  const [start, end] = range.split("-").map((n) => parseInt(n));

  if (!start) assert.fail("Not valid start");
  if (!end) assert.fail("Not valid end");

  for (let id = start; id <= end; id++) {
    const idStr = id.toString();

    const possibleGroups = idStr.length / 2;

    for (let n = 1; n <= possibleGroups; n++) {
      const parts = idStr.length / n;
      if (parts !== Math.floor(parts)) continue;

      const partLen = idStr.length / parts;

      const arr = chunk(idStr.split(""), partLen);

      if (arr.some((a) => a.join("") !== arr[0]!.join(""))) continue;

      if (invalidIds.includes(id)) continue;

      invalidIds.push(id);
    }
  }
}

const total = invalidIds.reduce((a, b) => a + b, 0);

console.log(total);
