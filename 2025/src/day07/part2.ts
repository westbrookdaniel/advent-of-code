import assert from "assert";
import fs from "fs";

const input = fs.readFileSync(import.meta.dir + "/input.txt", "utf8");
const lines = input.trim().split("\n");

const index = lines[0]!.indexOf("S");

const cache = <T extends (...args: any) => any>(fn: T): T => {
  const map: Record<string, T> = {};
  return ((...args: Parameters<T>) => {
    const key = JSON.stringify(args);
    if (map[key]) return map[key];
    const result = fn(...args);
    map[key] = result;
    return result;
  }) as any;
};

const countPaths = cache((index: number, lines: string[]): number => {
  if (lines.length === 0) return 1;

  const next = lines.slice(1);

  if (lines[0]![index] === "^") {
    const a = countPaths(index - 1, next);
    const b = countPaths(index + 1, next);
    return a + b;
  }

  return countPaths(index, next);
});

console.log(countPaths(index, lines));
