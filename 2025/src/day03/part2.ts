import assert from "assert";
import fs from "fs";

const input = fs.readFileSync(import.meta.dir + "/input.txt", "utf8");
const banks = input.trim().split("\n");

let total = 0;

function findLargestWithOffset(n: number) {
  return (largestIndex: number, bat: number, i: number, arr: number[]) => {
    const largest = arr[largestIndex];
    if (largest === undefined) assert.fail("oob");

    if (i > arr.length - n) return largestIndex;
    if (bat > largest) return i;
    return largestIndex;
  };
}

for (const bank of banks) {
  const numbers: number[] = [];

  let batteries = bank.split("").map((n) => parseInt(n));

  for (let i = 12; i > 0; i--) {
    const largestIndex = batteries.reduce(findLargestWithOffset(i), 0);

    numbers.push(batteries[largestIndex]!);
    batteries = batteries.slice(largestIndex + 1);
  }

  const joltage = parseInt(numbers.map((i) => i.toString()).join(""));

  total += joltage;
}

console.log(total);
