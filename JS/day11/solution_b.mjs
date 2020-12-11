import fs from "fs";

const EMPTY = "L";
const OCCUPIED = "#";
const FLOOR = ".";

class Field {
  /**
   * @param {string[][]} lines Lines of the field
   */
  constructor(lines) {
    this.lines = lines;
  }

  toString() {
    return this.lines.map((line) => line.join("")).join("\n");
  }

  get occupiedSeatCount() {
    return this.lines.reduce(
      (result, line) => result + line.filter((val) => val === OCCUPIED).length,
      0
    );
  }

  /**
   * Checks visible seats from a given position
   *
   * @param {number} x X-Coordinate of the seat to check
   * @param {number} y Y-Coordinate of the seat to check
   * @returns {number} Amount of seats visible from the given position
   */
  checkVisibleSeats(x, y) {
    let occupiedNeighbors = 0;
    for (let dY = -1; dY <= 1; dY++) {
      seatLoop: for (let dX = -1; dX <= 1; dX++) {
        let distance = 1;
        if (this.lines[y + dY * distance] === undefined) continue;

        if (dX === 0 && dY === 0) continue;

        let curCell = this.lines[y + dY * distance][x + dX * distance];

        while (curCell === FLOOR) {
          const line = this.lines[y + dY * distance];

          if (line === undefined) continue seatLoop;

          curCell = line[x + dX * distance];

          distance++;
        }

        if (curCell === OCCUPIED) occupiedNeighbors++;
      }
    }
    return occupiedNeighbors;
  }

  /**
   * @returns {number} Amount of seats that changed states.
   */
  step() {
    let changes = 0;
    /** @type {string[][]} */
    const changedLines = JSON.parse(JSON.stringify(this.lines));

    for (let y = 0; y < this.lines.length; y++) {
      for (let x = 0; x < this.lines[y].length; x++) {
        const currentCell = this.lines[y][x];
        if (currentCell === FLOOR) continue;

        const occupiedNeighbors = this.checkVisibleSeats(x, y);
        if (currentCell === EMPTY && occupiedNeighbors === 0) {
          changedLines[y][x] = OCCUPIED;
          changes++;
        } else if (currentCell === OCCUPIED && occupiedNeighbors >= 5) {
          changedLines[y][x] = EMPTY;
          changes++;
        }
      }
    }
    this.lines = changedLines;
    return changes;
  }
}

/**
 * Main function for Puzzle B
 *
 * @param {string} data - Puzzle input as a single string.
 * @returns {Promise<number>} Amount of open seats after applying all rules until stable.
 */
async function main(data) {
  const lines = data.split(/\r?\n/).map((line) => line.split(""));
  const field = new Field(lines);
  let steps = 0;
  while (field.step() > 0) {
    // console.log("Step", ++steps);
  }
  return field.occupiedSeatCount;
}

fs.readFile("input", (err, data) => {
  if (err) throw err;
  main(data.toString()).then(console.log).catch(console.error);
});
