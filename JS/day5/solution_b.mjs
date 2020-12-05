import fs from "fs";

/**
 * Main function for Puzzle A
 *
 * @param {string} data - Puzzle input as a single string.
 * @returns {number[]} List of Seat IDs missing from the puzzle input, ignoring the missing front and back seats.
 */
async function main(data) {
  const entries = data
    .split(/\r?\n/)
    .map((line) => line.replace(/[FL]/g, "0").replace(/[BR]/g, "1"))
    .map((line) => line.match(/^(?<y>[01]{7})(?<x>[01]{3})$/))
    .map((entry) => parseInt(entry[0], 2));

  /** @type {number[]} */
  let missing_seats = [];
  for (let id = Math.min(...entries); id < Math.max(...entries); id++) {
    if (!entries.includes(id)) {
      missing_seats.push(id);
    }
  }

  return missing_seats;
}

fs.readFile("input", (err, data) => {
  if (err) throw err;
  main(data.toString()).then(console.log).catch(console.error);
});
