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

let score = 0;

rucksacks.forEach((ruck) => {
  const comp1 = ruck.slice(0, ruck.length / 2);
  const comp2 = ruck.slice(ruck.length / 2, ruck.length);
  const commonChar = comp1.split("").find((char) => comp2.indexOf(char) !== -1);
  if (!commonChar) throw "Failed";
  score += charList.indexOf(commonChar) + 1;
});

console.log(score);
