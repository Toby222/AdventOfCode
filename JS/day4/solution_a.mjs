import fs from "fs";

const REQUIRED_FIELDS = [
  "byr", // (Birth Year)
  "iyr", // (Issue Year)
  "eyr", // (Expiration Year)
  "hgt", // (Height)
  "hcl", // (Hair Color)
  "ecl", // (Eye Color)
  "pid", // (Passport ID)
  // "cid", // (Country ID) // Optional
];

/**
 * Main function for Puzzle A
 *
 * @param {string} data - Puzzle input as a single string.
 */
async function main(data) {
  const entries = data.split(/(?:\r?\n){2}/).map((raw) => raw.split(/\s+/g));
  /** @type {{[string]: number|string}[]} */
  let passports = [];

  for (const raw of entries) {
    let passport = {};
    raw
      .map((rawKV) => rawKV.split(":"))
      .forEach(([key, value]) => (passport[key] = value));
    passports.push(passport);
  }

  /** @type {{[string]: number|string}[]} */
  const valid_passports = [];
  for (const passport of passports) {
    if (
      REQUIRED_FIELDS.map((required) =>
        Object.keys(passport).includes(required)
      ).reduce((prev, cur) => prev && cur, true)
    ) {
      valid_passports.push(passport);
    }
  }
  return valid_passports.length;
}

fs.readFile("input", (err, data) => {
  if (err) throw err;
  main(data.toString()).then(console.log).catch(console.error);
});
