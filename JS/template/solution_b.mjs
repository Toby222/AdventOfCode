import fs from "fs";

/**
 * Main function for Part 2
 *
 * @param {string} data - Puzzle input as a single string.
 */
async function main(data) {
  return;
}

fs.readFile("input", (err, data) => {
  console.time("Part 2")
  if (err) throw err;
  main(data.toString()).then(console.log).catch(console.error);
  console.timeEnd("Part 2")
});
