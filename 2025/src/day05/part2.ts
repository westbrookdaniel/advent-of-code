import assert from "assert";
import fs from "fs";

const input = fs.readFileSync(import.meta.dir + "/input.txt", "utf8");
const [fresh] = input
  .trim()
  .split("\n\n")
  .map((part) => part.split("\n"));

let lastRangesLength = 0;
let ranges = fresh!.map((r) => r.split("-").map((n) => parseInt(n)));

while (lastRangesLength !== ranges.length) {
  const mergedRanges: number[][] = [];

  for (const range of ranges) {
    const isOverlappingWithIndex = mergedRanges.findIndex((existing) => {
      return existing[0]! <= range[1]! && existing[1]! >= range[0]!;
    });

    if (isOverlappingWithIndex === -1) {
      mergedRanges.push(range);
    } else {
      const overlapping = mergedRanges[isOverlappingWithIndex]!;
      mergedRanges[isOverlappingWithIndex] = [
        Math.min(range[0]!, overlapping[0]!),
        Math.max(range[1]!, overlapping[1]!),
      ];
    }
  }

  lastRangesLength = ranges.length;
  ranges = mergedRanges;
}

let total = 0;
for (const range of ranges) {
  total += range[1]! - range[0]! + 1;
}

console.log(total);
