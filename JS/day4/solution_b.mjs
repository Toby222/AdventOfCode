import fs from "fs";

/** @type {Record<string, RegExp>}} */
const REQUIRED_FIELDS = {
  byr: /^\d{4}$/, // (Birth Year)
  iyr: /^\d{4}$/, // (Issue Year)
  eyr: /^\d{4}$/, // (Expiration Year)
  hgt: /^\d{2}(?:in|\dcm)$/, // (Height)
  hcl: /^#[0-9a-f]{6}$/, // (Hair Color)
  ecl: /^(?:amb|blu|brn|gry|grn|hzl|oth)$/, // (Eye Color)
  pid: /^\d{9}$/, // (Passport ID)
  // IGNORED // cid: /^.*$/, // (Country ID)
};

/**
 * Main function for Puzzle A
 *
 * @param {string} data - Puzzle input as a single string.
 */
async function main(data) {
  const entries = data.split(/(?:\r?\n){2}/).map((raw) => raw.split(/\s+/g));
  /** @type {Record<string, string|number>[]} */
  let passports = [];

  for (const raw of entries) {
    let passport = {};
    raw
      .map((rawKV) => rawKV.split(":"))
      .forEach(([key, value]) => {
        passport[key] = value;
      });
    if ([7, 8].includes(Object.keys(passport).length)) passports.push(passport);
  }

  let once = true;
  /** @type {Record<string, string|number>[]} */
  const valid_passports = [];
  for (const passport of passports) {
    let valid = true;

    for (const required in REQUIRED_FIELDS) {
      if (!valid) break;
      if (
        !passport.hasOwnProperty(required) ||
        !REQUIRED_FIELDS[required].test(passport[required])
      ) {
        valid = false;
        console.log(required)
        break;
      }
      switch (required) {
        case "byr":
          const byr = parseInt(passport[required]);
          if (1920 > byr || byr > 2002) valid = false;
          break;
        case "iyr":
          const iyr = parseInt(passport[required]);
          if (2010 > iyr || iyr > 2020) valid = false;
          break;
        case "eyr":
          const eyr = parseInt(passport[required]);
          if (2020 > eyr || eyr > 2030) valid = false;
          break;
        case "hgt":
          const hgt = parseInt(passport[required]);
          const unit = passport[required].substr(hgt.toString().length);
          switch (unit) {
            case "cm":
              if (hgt > 193 || hgt < 150) valid = false;
              break;
            case "in":
              if (hgt > 76 || hgt < 59) valid = false;
              break;
          }
          break;
      }
    }

    if (valid) {
      valid_passports.push(passport);
    }
  }
  return valid_passports.length;
}

fs.readFile("input", (err, data) => {
  if (err) throw err;
  main(data.toString()).then(console.log).catch(console.error);
});
