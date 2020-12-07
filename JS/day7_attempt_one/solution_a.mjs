import fs from "fs";

const COLORS = [
  "aqua",
  "beige",
  "black",
  "blue",
  "bronze",
  "brown",
  "chartreuse",
  "coral",
  "crimson",
  "cyan",
  "fuchsia",
  "gold",
  "gray",
  "green",
  "indigo",
  "lavender",
  "lime",
  "magenta",
  "maroon",
  "olive",
  "orange",
  "plum",
  "purple",
  "red",
  "salmon",
  "silver",
  "tan",
  "teal",
  "tomato",
  "turquoise",
  "violet",
  "white",
  "yellow",
];

const STYLES = [
  "bright",
  "clear",
  "dark",
  "dim",
  "dotted",
  "drab",
  "dull",
  "faded",
  "light",
  "mirrored",
  "muted",
  "pale",
  "plaid",
  "posh",
  "shiny",
  "striped",
  "vibrant",
  "wavy",
];

const ENTRY_REGEX = /^(?<style>\w+) (?<color>\w+) bags contain (?<contents>(?:(?:\d+ (?:\w+) (?:(?:\w+))|no other) bags?,? ?)+).$/;

/**
 * Main function for Puzzle A
 *
 * @param {string} data - Puzzle input as a single string.
 */
async function main(data) {
  const entries = data.split(/\r?\n/).map((line) => line.match(ENTRY_REGEX));
  if (entries.includes(null))
    throw new Error(`Failed ${data.split(/\r?\n/)[1]}`);

  /** @type {{[string]: [number, string]}} */
  const contained = {};
  /** @type {{[string]: [number, string]}} */
  const contains = {};
  for (const entry of entries) {
    /** @type {string} */
    const style = entry.groups.style;
    /** @type {string} */
    const color = entry.groups.color;
    /** @type {string} */
    const contents = entry.groups.contents;
    const parsedContents = contents
      .split(", ")
      .map((content) => content.split(" "))
      .map(([number, type, color]) =>
        number === "no" ? null : [parseInt(number), type + ";" + color]
      );
    contains[style + ";" + color] = parsedContents;

    for (const value of parsedContents) {
      if (value === null) continue;
      const number = value[0];
      const id = value[1];
      if (!contained.hasOwnProperty(id)) contained[id] = [];
      contained[id].push([number, style + ";" + color]);
    }
  }

  /** @type {string[]} */
  const to_check = contained["shiny;gold"].slice();
  const checked = [];

  while (to_check.length > 0) {
    const checking = to_check.shift();
    if (checked.includes(checking)) {
      continue;
    }

    if (contained.hasOwnProperty(checking[1])) {
      to_check.push(
        ...contained[checking[1]]
      );
    }
    checked.push(checking);
  }

  return JSON.stringify(checked);
}

fs.readFile("input", (err, data) => {
  if (err) throw err;
  main(data.toString()).then(console.log).catch(console.error);
});
