import assert from "assert";
import fs from "fs";

const input = fs.readFileSync(import.meta.dir + "/input.txt", "utf8");
const [fresh, data] = input
  .trim()
  .split("\n\n")
  .map((part) => part.split("\n"));

const ranges = fresh!.map((r) => r.split("-").map((n) => parseInt(n)));
const ings = data!.map((n) => parseInt(n));

let total = 0;

for (const ing of ings) {
  let fresh = false;

  for (const range of ranges) {
    if (ing >= range[0]! && ing <= range[1]!) fresh = true;
    if (fresh) break;
  }

  if (fresh) total += 1;
}

console.log(total);
