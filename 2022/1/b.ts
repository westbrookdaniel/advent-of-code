const text = await Deno.readTextFile("./input.txt");

const elfCals = text.split("\n\n").map((cs) =>
  cs.split("\n").map((n) => parseInt(n))
);

const top: number[] = [];

function getMostCalsIndex() {
  const mostCalsIndex = elfCals.reduce((biggestIndex, cur, i) => {
    if (top.indexOf(i) !== -1) return biggestIndex;

    const curCals = cur.reduce((a, b) => a + b);
    const biggest = elfCals[biggestIndex]?.reduce((a, b) => a + b);
    if (biggest && biggest > curCals) return biggestIndex;
    return i;
  }, -1);
  top.push(mostCalsIndex);
}

getMostCalsIndex();
getMostCalsIndex();
getMostCalsIndex();

console.log(
  top.map((i) => elfCals[i].reduce((a, b) => a + b)).reduce((a, b) => a + b),
);
