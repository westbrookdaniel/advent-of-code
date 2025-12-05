import { parentPort, workerData } from "worker_threads";

const { start, end, ranges } = workerData;

function totalInRange(min: number, max: number): number {
  let total = 0;

  for (let i = min; i <= max; i++) {
    let freshRange: null | number[] = null;

    for (const range of ranges) {
      if (i >= range[0]! && i <= range[1]!) freshRange = range;
      if (freshRange) break;
    }

    if (freshRange) {
      let diff = freshRange[1]! - Math.max(freshRange[0]!, i);
      total += diff + 1;
      i = freshRange[1]!;
    }
  }

  return total;
}

const result = totalInRange(start, end);
parentPort!.postMessage(result);
