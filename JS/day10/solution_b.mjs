import fs from "fs";

/**
 * Main function for Puzzle B
 *
 * @param {string} data - Puzzle input as a single string.
 */
async function main(data) {
  const joltages = data
    .split(/\r?\n/)
    .map((line) => parseInt(line))
    .sort((a, b) => a - b)
    .reverse();
  const maxVal = joltages[0] + 3;
  joltages.push(0);
  joltages.unshift(maxVal);

  const poss = {};
  const len = [];
  len[maxVal] = 1;

  joltages.forEach((joltage) => {
    poss[joltage] = joltages.filter(
      (val) => val > joltage && val <= joltage + 3
    );
    if (joltage !== maxVal) {
      len[joltage] = 0;
      for (let i = 0; i < poss[joltage].length; i++) {
        len[joltage] += len[poss[joltage][i]];
      }
    }
  });

  return len[0];
}

fs.readFile("input", (err, data) => {
  if (err) throw err;
  main(data.toString()).then(console.log).catch(console.error);
});
