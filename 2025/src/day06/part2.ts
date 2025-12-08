import assert from "assert";
import fs from "fs";
import { zip } from "es-toolkit";

const input = fs.readFileSync(import.meta.dir + "/input.txt", "utf8");

const lines = input.trim().split("\n");

const methodsLine = lines.pop()!.split("");

const cols: number[][] = [];
const methods: string[] = [];

const currentCol: number[] = [];

let lastColBlank = true;
let i = 0;
while (true) {
  const char = methodsLine[i]!;
  const n = lines
    .map((l) => l[i] ?? "")
    .join("")
    .trim();

  if (char !== " " && char != undefined) {
    methods.push(char);
  }

  if (n === "") {
    if (lastColBlank) break;
    lastColBlank = true;

    cols.push(structuredClone(currentCol));
    currentCol.length = 0;
  } else {
    lastColBlank = false;

    currentCol.push(parseInt(n));
  }

  i++;
}

let total = 0;

for (let i = 0; i < methods.length; i++) {
  const method = methods[i]!;
  const col = cols[i]!;

  let sum = 0;

  if (method === "+") {
    sum = col.reduce((acc, n) => {
      return acc + n;
    });
  }

  if (method === "*") {
    sum = col.reduce((acc, n) => {
      return acc * n;
    });
  }

  total += sum;
}

console.log(total);
