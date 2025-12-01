import assert from "assert";
import fs from "fs";

const input = fs.readFileSync(import.meta.dir + "/input.txt", "utf8");
const lines = input.trim().split("\n");

let pos = 50;
let total = 0;

for (const line of lines) {
  const [dir, ...str] = line.split("");
  const n = parseInt(str.join(""));

  for (let i = 0; i < n; i++) {
    if (dir === "L") pos -= 1;
    else pos += 1;

    if (pos < 0) pos = 99;
    if (pos > 99) pos = 0;

    if (pos === 0) {
      total += 1;
    }
  }
}

console.log();
console.log(total);
