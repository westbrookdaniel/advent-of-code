import assert from "assert";
import fs from "fs";
import { zip } from "es-toolkit";

const input = fs.readFileSync(import.meta.dir + "/input.txt", "utf8");

const grid = input
  .trim()
  .split("\n")
  .map((line) => {
    const strs: string[] = [];
    const parts = line.trim().split(/\s+/);
    for (const part of parts) {
      if (part.length > 0) strs.push(part);
    }
    return strs;
  });

const cols = zip(...grid);

let total = 0;

for (const col of cols) {
  const method = col.pop();
  const numbers = col.map((n) => parseInt(n));

  let sum = 0;

  if (method === "+") {
    sum = numbers.reduce((acc, n) => {
      return acc + n;
    });
  }

  if (method === "*") {
    sum = numbers.reduce((acc, n) => {
      return acc * n;
    });
  }

  total += sum;
}

console.log(total);
