import fs from "fs";

const CONTENT_REGEX = /^(\d+) (\w+ \w+) bags?.?$/;

/**
 * Main function for Puzzle A
 *
 * @param {string} data - Puzzle input as a single string.
 * @returns {number} Number of all bag types that eventually contain a shiny golden bag.
 */
async function main(data) {
  const lines = data.split(/\r?\n/);

  /** @type {Object.<string, [number, string][]>} */
  const contentsOf = {};
  /** @type {Object.<string, [number, string][]>} */
  const containedBy = {};
  for (const line of lines) {
    const [bag, content] = line.split(" bags contain ");

    contentsOf[bag] = [];
    if (content === "no other bags.") continue;

    contentsOf[bag] = content.split(", ").map((entry) => {
      const [, count, containedBag] = entry.match(CONTENT_REGEX);
      return [parseInt(count), containedBag];
    });
    for (const [count, containedBag] of contentsOf[bag]) {
      if (!containedBy.hasOwnProperty(containedBag))
        containedBy[containedBag] = [];
      containedBy[containedBag].push([count, bag]);
    }
  }

  const to_check = containedBy["shiny gold"].slice();
  /** @type {string[]} */
  const checked = [];

  while (to_check.length > 0) {
    const [, bag] = to_check.shift();
    if (checked.includes(bag) || bag === undefined) continue;
    if (containedBy.hasOwnProperty(bag))
      to_check.push(...containedBy[bag].slice());
    checked.push(bag);
  }

  return checked;
}

fs.readFile("input", (err, data) => {
  if (err) throw err;
  main(data.toString()).then(console.log).catch(console.error);
});
