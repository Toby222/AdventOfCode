import fs from "fs";

/**
 * Main function for Puzzle A
 *
 * @param {number[]} numbers - Parsed puzzle input.
 * @returns {[index: number, value: number]} Index and value of the first number that can't be made by adding any of the previous 25.
 */
async function partA(numbers) {
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

/**
 * Main function for Puzzle B
 *
 * @param {string} data - Puzzle input as a single string.
 * @returns {number} The sum of the smallest and largest number in a contiguous range that adds up to the result of part A.
 */
async function main(data) {
  const numbers = data.split(/\r?\n/).map((line) => parseInt(line));

  const [, partAResult] = await partA(numbers);

  let minIndex = 0;
  let maxIndex = 1;

  while (
    numbers
      .slice(minIndex, maxIndex + 1)
      .reduce((prev, cur) => prev + cur, 0) !== partAResult
  ) {
    maxIndex++;
    if (maxIndex == numbers.length) {
      minIndex++;
      maxIndex = minIndex + 1;
    }
  }
  const minNumber = Math.min(...numbers.slice(minIndex, maxIndex + 1));
  const maxNumber = Math.max(...numbers.slice(minIndex, maxIndex + 1));

  return minNumber + maxNumber;
}

fs.readFile("input", (err, data) => {
  if (err) throw err;
  main(data.toString()).then(console.log).catch(console.error);
});
