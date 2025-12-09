import assert from "assert";
import fs from "fs";

type Point = {
  id: number;
  x: number;
  y: number;
  z: number;
};

function distance(p: Point, q: Point) {
  const n1 = Math.pow(p.x - q.x, 2);
  const n2 = Math.pow(p.y - q.y, 2);
  const n3 = Math.pow(p.z - q.z, 2);
  return Math.sqrt(n1 + n2 + n3);
}

const input = fs.readFileSync(import.meta.dir + "/input.txt", "utf8");
const boxes = input
  .trim()
  .split("\n")
  .map((str, i): Point => {
    const [x, y, z] = str.split(",") as any;
    return { id: i, x, y, z };
  });

const obj: Record<string, number> = {};

for (const box of boxes) {
  for (const other of boxes) {
    if (box.id === other.id) continue;

    let key = "";
    if (box.id < other.id) key = `${other.id},${box.id}`;
    else key = `${box.id},${other.id}`;

    if (!obj[key]) obj[key] = distance(box, other);
  }
}

const distances = Object.entries(obj);

distances.sort((a, b) => {
  const [, aNum] = a;
  const [, bNum] = b;
  return aNum - bNum;
});

const PAIRS = 1_000;

const joins = distances.slice(0, PAIRS).map((dist) => {
  const [aId, bId] = dist[0].split(",");
  return [aId!, bId!];
});

for (const join of joins) {
  for (const other of joins) {
    if (join === other) continue;

    if (other.includes(join[0]!) || other.includes(join[1]!)) {
      other.push(...join);
      other.splice(0, other.length, ...new Set(other));
    }
  }
}

const final = joins
  .map((j) => j.sort())
  .reduce<string[][]>((acc, a) => {
    for (const b of acc) {
      if (a.join() === b.join()) return acc;
    }
    acc.push(a);
    return acc;
  }, [])
  .sort((a, b) => b.length - a.length)
  .slice(0, 3)
  .map((j) => j.length)
  .reduce((acc, n) => acc * n);

console.log(final);
