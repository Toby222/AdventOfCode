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
  console.time("Part 1");
  if (err) throw err;
  main(data.toString())
    .then((...args) => {
      console.log(...args);
      console.timeEnd("Part 1");
    })
    .catch(console.error);
});
