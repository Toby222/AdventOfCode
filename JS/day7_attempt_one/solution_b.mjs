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

const ENTRY_REGEX = new RegExp(
  `^(?<style>${STYLES.join("|")}) (?<color>${COLORS.join(
    "|"
  )}) bags contain (?:((?:\\d+ (?:${STYLES.join("|")}) (?:${COLORS.join(
    "|"
  )}))|no other) bags,?)+$`
);
const ENTRY_REGEX_PREGENERATED = /^(?<style>bright|clear|dark|dim|dotted|drab|dull|faded|light|mirrored|muted|pale|plaid|posh|shiny|striped|vibrant|wavy) (?<color>aqua|beige|black|blue|bronze|brown|chartreuse|coral|crimson|cyan|fuchsia|gold|gray|green|indigo|lavender|lime|magenta|maroon|olive|orange|plum|purple|red|salmon|silver|tan|teal|tomato|turquoise|violet|white|yellow) bags contain ((?:d+ (?:bright|clear|dark|dim|dotted|drab|dull|faded|light|mirrored|muted|pale|plaid|posh|shiny|striped|vibrant|wavy) (?:aqua|beige|black|blue|bronze|brown|chartreuse|coral|crimson|cyan|fuchsia|gold|gray|green|indigo|lavender|lime|magenta|maroon|olive|orange|plum|purple|red|salmon|silver|tan|teal|tomato|turquoise|violet|white|yellow))|no other) bags$/;

/**
 * Main function for Puzzle B
 *
 * @param {string} data - Puzzle input as a single string.
 */
async function main(data) {
  const entries = data
    .split(/\r?\n/)
    .map((line) => line.match(ENTRY_REGEX_PREGENERATED));

  const contained = {};
  const contains = {};
  return [data.split(/\r?\n/)[0], entries[0], ENTRY_REGEX];
}

fs.readFile("input", (err, data) => {
  if (err) throw err;
  main(data.toString()).then(console.log).catch(console.error);
});
