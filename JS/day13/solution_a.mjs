import fs from "fs";

/**
 * Main function for Puzzle A
 *
 * @param {string} data - Puzzle input as a single string.
 * @returns {Promise<number>} The ID of the earliest bus to take, multiplied by the minutes until then.
 */
async function main(data) {
  const lines = data.split(/\r?\n/);
  const earliest = parseInt(lines[0]);
  const buses = lines[1]
    .split(",")
    .map((val) => parseInt(val))
    .filter((num) => !isNaN(num));

  const departsAfter = buses
    .map((id) => [id, id - (earliest % id)])
    .sort((a, b) => a[1] - b[1]);

  return departsAfter[0][0] * departsAfter[0][1];
}

fs.readFile("input", (err, data) => {
  if (err) throw err;
  main(data.toString()).then(console.log).catch(console.error);
});
