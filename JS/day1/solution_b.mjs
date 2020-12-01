import fs from "fs";

/**
 * Main function for Puzzle B
 *
 * @param {string} data - Puzzle input as a single string.
 */
async function main(data) {
  const rows = data.split("\n").map((line) => parseInt(line));

  for (const a of rows) {
    for (const b of rows) {
      for (const c of rows) {
        if (a + b + c === 2020) {
          return a * b * c;
        }
      }
    }
  }

  throw new Error("No solution found!");
}

fs.readFile("input", (err, data) => {
  if (err) throw err;
  main(data.toString()).then(console.log).catch(console.error);
});
