import fs from "fs";

const TREE = "#";
const FREE = ".";

class Grid {
  /**
   * Construct a new Grid from an array of lines
   *
   * @param {string[]} lines The lines of the repeating Grid.
   */
  constructor(lines) {
    this.lines = lines;
  }

  /**
   * Get the field at the respective coordinate in the grid.
   *
   * @param {number} x X-Coordinate of the field to get.
   * @param {number} y Y-Coordinate of the field to get.
   *
   * @returns {string} The field at the coordinate.
   */
  get(x, y) {
    const line = this.lines[y];

    return line[x % line.length];
  }
}

/**
 * Main function for Puzzle A
 *
 * @param {string} data - Puzzle input as a single string.
 *
 * @returns {number} The amount of trees on the slope.
 */
async function main(data) {
  const grid = new Grid(data.split("\n").map((line) => line.trim()));

  let x = 0;
  let y = 0;
  let trees = 0;

  const dX = 3;
  const dY = 1;

  while (y < grid.lines.length) {
    if (grid.get(x, y) === TREE) {
      trees++;
    }

    x += dX;
    y += dY;
  }

  return trees;
}

fs.readFile("input", (err, data) => {
  if (err) throw err;
  main(data.toString()).then(console.log).catch(console.error);
});
