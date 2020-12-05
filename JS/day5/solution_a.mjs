import fs from "fs";

/**
 * Main function for Puzzle A
 *
 * @param {string} data - Puzzle input as a single string.
 * @returns {number} The highest Seat ID in the puzzle input.
 */
async function main(data) {
  const entries = data
    .split(/\r?\n/)
    .map((line) => line.replace(/[FL]/g, "0").replace(/[BR]/g, "1"))
    .map((line) => line.match(/^(?<y>[01]{7})(?<x>[01]{3})$/))
    .map((entry) => parseInt(entry[0], 2));

  return Math.max(...entries);
}

fs.readFile("input", (err, data) => {
  if (err) throw err;
  main(data.toString()).then(console.log).catch(console.error);
});
