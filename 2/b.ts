const text = await Deno.readTextFile("./input.txt");

const scoreMap: Record<string, number> = { A: 1, B: 2, C: 3 };

function getInput(outcomeTheirs: string) {
  if (outcomeTheirs === "XA") return "C";
  if (outcomeTheirs === "XB") return "A";
  if (outcomeTheirs === "XC") return "B";

  if (outcomeTheirs === "YA") return "A";
  if (outcomeTheirs === "YB") return "B";
  if (outcomeTheirs === "YC") return "C";

  if (outcomeTheirs === "ZA") return "B";
  if (outcomeTheirs === "ZB") return "C";
  if (outcomeTheirs === "ZC") return "A";

  return "";
}

function outcomeScore(yoursTheirs: string) {
  if (yoursTheirs === "BA") return 6;
  if (yoursTheirs === "BC") return 0;
  if (yoursTheirs === "BB") return 3;

  if (yoursTheirs === "CA") return 0;
  if (yoursTheirs === "CC") return 3;
  if (yoursTheirs === "CB") return 6;

  if (yoursTheirs === "AA") return 3;
  if (yoursTheirs === "AC") return 6;
  if (yoursTheirs === "AB") return 0;

  return 0;
}

const games = text.split("\n").map((g) => g.split(" "));

const points = games.map((g) => {
  const theirs = g[0];
  const yours = getInput(g[1] + theirs);
  const o = outcomeScore(yours + theirs);
  return scoreMap[yours] + o;
});

console.log(points.reduce((a, b) => a + b));
