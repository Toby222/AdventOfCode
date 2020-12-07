import fs from "fs";

const CONTENT_REGEX = /^(\d+) (\w+ \w+) bags?.?$/;

/**
 * Main function for Puzzle B
 *
 * @param {string} data - Puzzle input as a single string.
 * @returns {number} Amount of bags inside a shiny golden bag.
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

  /**
   * @param {string} bag Bag to count contents of.
   * @returns {number} Total amount of bags inside bag, recursively.
   */
  function countBags(bag) {
    if (!contentsOf.hasOwnProperty(bag)) {
      return 0;
    }

    const contents = contentsOf[bag];
    let result = contents.reduce((prev, cur) => prev + cur[0], 0);
    result += contents.reduce(
      (prev, cur) => prev + cur[0] * countBags(cur[1]),
      0
    );
    return result;
  }
  return countBags("shiny gold");
}

fs.readFile("input", (err, data) => {
  if (err) throw err;
  main(data.toString()).then(console.log).catch(console.error);
});
