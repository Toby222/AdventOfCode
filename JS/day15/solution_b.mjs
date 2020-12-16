import fs from "fs";
import { stdout } from "process";

/**
 * Main function for Puzzle B
 *
 * @param {string} data - Puzzle input as a single string.
 */
async function main(data) {
  const input = data.split(",").map((num) => parseInt(num));
  /** @type {Map<number, number>} */
  const lastSaid = new Map();
  let prevSpoken = NaN;
  let spoken = NaN;

  for (let i = 0; i < input.length - 1; i++) {
    lastSaid.set(input[i], i);
  }

  spoken = input[input.length - 1];

  for (let i = input.length; i < 30000000; i++) {
    prevSpoken = spoken;
    spoken = 0;
    if (lastSaid.has(prevSpoken))
      spoken = i - lastSaid.get(prevSpoken) - 1;

    lastSaid.set(prevSpoken, i - 1);
  }

  return spoken;
}

fs.readFile("input", (err, data) => {
  if (err) throw err;
  main(data.toString()).then(console.log).catch(console.error);
});
