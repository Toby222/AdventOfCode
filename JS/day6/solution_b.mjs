import fs from "fs";

/**
 * Main function for Puzzle B
 *
 * @param {string} data - Puzzle input as a single string.
 * @returns {number} Sum of amount of questions answered positively by everyone per group.
 */
async function main(data) {
  const groups = data.split(/\r?\n\r?\n/).map((group) => group.split(/\r?\n/));

  /** @type {number[]} */
  const answer_count = [];

  for (const group of groups) {
    let answers = 0;
    for (const char of "abcdefghijklmnopqrstuvwxyz") {
      if (group.reduce((prev, cur) => prev && cur.includes(char), true)) {
        answers++;
      }
    }
    answer_count.push(answers);
  }

  return answer_count.reduce((prev, cur) => prev + cur, 0);
}

fs.readFile("input", (err, data) => {
  if (err) throw err;
  main(data.toString()).then(console.log).catch(console.error);
});
