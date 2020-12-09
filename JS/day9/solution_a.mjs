import fs from "fs";

/**
 * Main function for Puzzle A
 *
 * @param {string} data - Puzzle input as a single string.
 * @returns {[index: number, value: number]} Index and value of the first number that can't be made by adding any of the previous 25.
 */
async function main(data) {
  const numbers = data.split(/\r?\n/).map((line) => parseInt(line));

  number_loop: for (let i = 25; i < numbers.length; i++) {
    for (let a = i - 25; a < i; a++) {
      for (let b = a + 1; b < i; b++) {
        if (numbers[i] === numbers[a] + numbers[b]) continue number_loop;
      }
    }
    return [i, numbers[i]];
  }

  return [NaN, NaN];
}

fs.readFile("input", (err, data) => {
  if (err) throw err;
  main(data.toString()).then(console.log).catch(console.error);
});
