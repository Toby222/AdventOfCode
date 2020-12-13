import fs from "fs";
/**
 * Main function for Puzzle B
 *
 * @param {string} data - Puzzle input as a single string.
 * @returns {Promise<string>} Equation to get the final result. Needs to be solved elsewhere.
 */
async function main(data) {
  const input = data.split(/\r?\n/)[1];
  const buses = input
    .split(",")
    .map((val) => parseInt(val))
    .map((id, i) => (isNaN(id) ? null : `(t+${i}) mod ${id} == 0`))
    .filter((val) => val !== null);
  const eq = buses.join(" && ");

  // Solve elsewhere
  // WolframAlpha works
  return eq;
}

fs.readFile("input", (err, data) => {
  if (err) throw err;
  main(data.toString()).then(console.log).catch(console.error);
});
