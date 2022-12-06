const text = await Deno.readTextFile("./input.txt");

const charList = [
  "a",
  "b",
  "c",
  "d",
  "e",
  "f",
  "g",
  "h",
  "i",
  "j",
  "k",
  "l",
  "m",
  "n",
  "o",
  "p",
  "q",
  "r",
  "s",
  "t",
  "u",
  "v",
  "w",
  "x",
  "y",
  "z",
  "A",
  "B",
  "C",
  "D",
  "E",
  "F",
  "G",
  "H",
  "I",
  "J",
  "K",
  "L",
  "M",
  "N",
  "O",
  "P",
  "Q",
  "R",
  "S",
  "T",
  "U",
  "V",
  "W",
  "X",
  "Y",
  "Z",
];

const rucksacks = text.split("\n");
const ruckSackGroups = rucksacks.reduce((acc, c) => {
  if (acc.length === 0) return [[c]];
  const latest = acc[acc.length - 1];
  if (latest.length !== 3) {
    latest.push(c);
    return acc;
  }
  return [...acc, [c]];
}, [] as string[][]);

let score = 0;

ruckSackGroups.forEach((ruck) => {
  const inter = new Set(
    [...ruck[0]].filter((x) => new Set(ruck[1]).has(x)).filter((x) =>
      new Set(ruck[2]).has(x)
    ),
  );
  const item = inter.entries().next().value[0];
  if (!item) throw "Not found";
  score += charList.indexOf(item) + 1;
});

console.log(score);
