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
    if (y > this.lines.length)
      throw new Error(`Out of range [${[x, y]}] max y ${this.lines.length}`);
    const line = this.lines[y];

    return line[x % line.length];
  }
}

/**
 * Main function for Puzzle A
 *
 * @param {string} data - Puzzle input as a single string.
 * @returns {number} The number of trees on all slopes, multiplied.
 */
async function main(data) {
  const grid = new Grid(data.split("\n").map((line) => line.trim()));
  const slopes = [
    [1, 1],
    [3, 1],
    [5, 1],
    [7, 1],
    [1, 2],
  ];

  let result = 1;
  for (const slope of slopes) {
    const [dX, dY] = slope;
    let x = 0;
    let y = 0;
    let trees = 0;

    while (y < grid.lines.length) {
      if (grid.get(x, y) === TREE) {
        trees++;
      }

      x += dX;
      y += dY;
    }
    result *= trees;
  }

  return result;
}

fs.readFile("input", (err, data) => {
  if (err) throw err;
  main(data.toString()).then(console.log).catch(console.error);
});
