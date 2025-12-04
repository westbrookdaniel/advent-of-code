import assert from "assert";
import fs from "fs";

const input = fs.readFileSync(import.meta.dir + "/input.txt", "utf8");

let lines = input
  .trim()
  .split("\n")
  .map((line) => line.split(""));
let next = structuredClone(lines);

let all = 0;

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

while (true) {
  let total = 0;

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
        next[y]![x] = "x";
      }
    }
  }

  const nextFixed = next.map((l) =>
    l.map((ch) => {
      if (ch === "x") return ".";
      return ch;
    }),
  );

  lines = structuredClone(nextFixed);
  next = structuredClone(lines);

  if (total === 0) break;
  all += total;
}

console.log(all);
