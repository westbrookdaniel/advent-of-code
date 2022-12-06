const text = await Deno.readTextFile("./input.txt");

const elfCals = text.split("\n\n").map((cs) =>
  cs.split("\n").map((n) => parseInt(n))
);

const mostCals = elfCals.reduce((biggest, cur) => {
  const curCals = cur.reduce((a, b) => a + b);
  if (curCals > biggest) return curCals;
  return biggest;
}, 0);

console.log(mostCals);
