import fs from "fs";

/**
 * Main function for Part 1
 *
 * @param {string} data - Puzzle input as a single string.
 */
async function main(data) {
  return;
}

fs.readFile("input", (err, data) => {
  if (err) throw err;
  console.time("Part 1");
  main(data.toString())
    .then((...args) => {
      console.timeEnd("Part 1");
      console.log(...args);
    })
    .catch(console.error);
});
