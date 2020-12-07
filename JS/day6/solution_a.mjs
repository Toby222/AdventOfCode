import fs from "fs";

/**
 * Main function for Puzzle A
 *
 * @param {string} data - Puzzle input as a single string.
 * @returns {number} Sum of amount of questions answered positively by anyone per group.
 */
async function main(data) {
  const groups = data.split(/\r?\n\r?\n/);
  // .map(group => group.split(/\r\n/));

  /** @type {number[]} */
  const answer_count = [];

  for (const group of groups) {
    let answers = 0;
    for (const char of "abcdefghijklmnopqrstuvwxyz") {
      if (group.includes(char)) {
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
