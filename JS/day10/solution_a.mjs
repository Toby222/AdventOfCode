import fs from "fs";

/**
 * Main function for Puzzle A
 *
 * @param {string} data - Puzzle input as a single string.
 */
async function main(data) {
  const adapters = data
    .split(/\r?\n/)
    .map((line) => parseInt(line))
    .sort((a, b) => a - b);

  /** @type {number[]} */
  const diffs = [];

  let curNum = 0;
  while (adapters.length > 0) {
    const delta = adapters[0] - curNum;

    diffs[delta] = (diffs[delta] ?? 0) + 1;

    curNum = adapters.shift();
  }

  return diffs[1] * (diffs[3] + 1);
}

fs.readFile("input", (err, data) => {
  if (err) throw err;
  main(data.toString()).then(console.log).catch(console.error);
});
