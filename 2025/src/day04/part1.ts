import assert from "assert";
import fs from "fs";

const input = fs.readFileSync(import.meta.dir + "/input.txt", "utf8");
const lines = input.trim().split("\n");

let total = 0;

const dirs = [
  { x: -1, y: -1 },
  { x: 0, y: -1 },
  { x: 1, y: -1 },
  { x: -1, y: 0 },
  { x: 1, y: 0 },
  { x: -1, y: 1 },
  { x: 0, y: 1 },
  { x: 1, y: 1 },
];

for (let y = 0; y < lines.length; y++) {
  const line = lines[y];
  assert(line);

  for (let x = 0; x < lines.length; x++) {
    const char = line[x];
    assert(char);

    if (char !== "@") {
      continue;
    }

    let surrounding = 0;

    for (const dir of dirs) {
      const dirX = x + dir.x;
      const dirY = y + dir.y;

      const dirLine = lines[dirY];
      if (!dirLine) continue;

      const dirChar = dirLine[dirX];
      if (!dirChar) continue;

      if (dirChar === "@") surrounding += 1;
    }

    if (surrounding < 4) {
      total += 1;
    }
  }
}

console.log(total);
