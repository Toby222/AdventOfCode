import fs from "fs";

/**
 * Main function for Puzzle A
 *
 * @param {string} data - Puzzle input as a single string.
 */
async function main(data) {
  const rows = data.split("\n").map((line) => parseInt(line));

  for (const a of rows) {
    for (const b of rows) {
      if (a + b === 2020) {
        return a * b;
      }
    }
  }

  throw new Error("No solution found!");
}

fs.readFile("input", (err, data) => {
  if (err) throw err;
  main(data.toString()).then(console.log).catch(console.error);
});
