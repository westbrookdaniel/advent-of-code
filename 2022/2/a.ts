const text = await Deno.readTextFile("./input.txt");

const scoreMap: Record<string, number> = { A: 1, B: 2, C: 3 };
const inputMap: Record<string, string> = { X: "A", Y: "B", Z: "C" };

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
  const yours = inputMap[g[1]];
  const theirs = g[0];
  const o = outcomeScore(yours + theirs);
  return scoreMap[yours] + o;
});

console.log(points.reduce((a, b) => a + b));
