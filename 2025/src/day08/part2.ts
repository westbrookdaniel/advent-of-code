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
    const [x, y, z] = str.split(",");
    return {
      id: i,
      x: parseInt(x!),
      y: parseInt(y!),
      z: parseInt(z!),
    };
  });

const obj: Record<string, number> = {};

console.time("distances");

for (const box of boxes) {
  for (const other of boxes) {
    if (box.id === other.id) continue;

    let key = "";
    if (box.id < other.id) key = `${other.id},${box.id}`;
    else key = `${box.id},${other.id}`;

    if (!obj[key]) obj[key] = distance(box, other);
  }
}

console.timeEnd("distances");
console.time("sort");

const distances = Object.entries(obj);

distances.sort((a, b) => {
  const [, aNum] = a;
  const [, bNum] = b;
  return aNum - bNum;
});

console.timeEnd("sort");
console.time("convert");

const joins = distances.map((dist) => {
  const [aId, bId] = dist[0].split(",");
  return [aId!, bId!] as const;
});

console.timeEnd("convert");
console.time("circuit");

const circuit = [joins[0]![0]];

// TODO this part is wrong, make sure it gets part 1 right still

for (const join of joins) {
  if (circuit.includes(join[0])) {
    if (!circuit.includes(join[1])) circuit.push(join[1]);
  } else if (circuit.includes(join[1])) {
    if (!circuit.includes(join[0])) circuit.push(join[0]);
  }
}

console.timeEnd("circuit");

const aId = circuit.pop();
const bId = circuit.pop();

const a = boxes.find((box) => box.id.toString() === aId)!;
const b = boxes.find((box) => box.id.toString() === bId)!;

console.log(a.x * b.x);
