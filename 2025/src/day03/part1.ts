import assert from "assert";
import fs from "fs";

const input = fs.readFileSync(import.meta.dir + "/input.txt", "utf8");
const banks = input.trim().split("\n");

let total = 0;

for (const bank of banks) {
  const batteries = bank.split("").map((n) => parseInt(n));

  const largestIndex = batteries.reduce((largestIndex, bat, i, arr) => {
    const largest = arr[largestIndex];
    if (largest === undefined) assert.fail("oob");

    if (i === arr.length - 1) return largestIndex;
    if (bat > largest) return i;
    return largestIndex;
  }, 0);

  const second = batteries.slice(largestIndex + 1);

  const largestSecondIndex = second.reduce((largestIndex, bat, i, arr) => {
    const largest = arr[largestIndex];
    if (largest === undefined) assert.fail("oob");

    if (bat > largest) return i;
    return largestIndex;
  }, 0);

  const f = batteries[largestIndex]?.toString() ?? "";
  const s = second[largestSecondIndex]?.toString() ?? "";

  const joltage = parseInt(f + s);

  total += joltage;
}

console.log(total);
