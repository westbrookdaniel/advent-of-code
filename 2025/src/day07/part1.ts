import assert from "assert";
import fs from "fs";

const input = fs.readFileSync(import.meta.dir + "/input.txt", "utf8");

const lines = input.trim().split("\n");

let beams: number[] = [];

let splitCount = 0;

for (const line of lines) {
  if (!beams.length) {
    beams.push(line.indexOf("S"));
    continue;
  }

  const next = beams.reduce<number[]>((acc, beam) => {
    let chars = line.split("");
    if (chars[beam] === "^") {
      splitCount += 1;
      acc.push(beam - 1, beam + 1);
    } else {
      acc.push(beam);
    }
    return acc;
  }, []);

  beams = [...new Set(next)];
}

console.log(splitCount);
