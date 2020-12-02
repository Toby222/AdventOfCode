import * as fs from "fs";

const ENTRY_REGEX = /^(\d+?)-(\d+?) (\w?): (\w+)$/;

/**
 * Main function for Puzzle A
 *
 * @param {string} data Puzzle input as a single string.
 * @returns {number} Amount of valid passwords in puzzle input.
 */
async function main(data) {
  const lines = data.split("\n").map((line) => line.trim());
  const pass_data = lines.map((line) => line.match(ENTRY_REGEX));

  let valid_passwords = [];
  for (const entry of pass_data) {
    if (!entry) throw new Error(`Failed to parse some data. ${entry}`);
    const min = parseInt(entry[1]);
    const max = parseInt(entry[2]);
    const char = entry[3];
    const pass = entry[4];

    if (
      pass.split(char).length - 1 >= min &&
      pass.split(char).length - 1 <= max
    ) {
      valid_passwords.push(entry);
    }
  }

  return valid_passwords.length;
}

fs.readFile("input", (err, data) => {
  if (err) throw err;
  main(data.toString()).then(console.log).catch(console.error);
});
