import fs from "fs";

const SIZE = 25;
/** @type {[dX: number, dY: number, dZ: number, dW: number][]} */
const NEIGHBORS_4D = [
  [-1, -1, -1, -1],
  [-1, -1, -1, 0],
  [-1, -1, -1, 1],
  [-1, -1, 0, -1],
  [-1, -1, 0, 0],
  [-1, -1, 0, 1],
  [-1, -1, 1, -1],
  [-1, -1, 1, 0],
  [-1, -1, 1, 1],
  [-1, 0, -1, -1],
  [-1, 0, -1, 0],
  [-1, 0, -1, 1],
  [-1, 0, 0, -1],
  [-1, 0, 0, 0],
  [-1, 0, 0, 1],
  [-1, 0, 1, -1],
  [-1, 0, 1, 0],
  [-1, 0, 1, 1],
  [-1, 1, -1, -1],
  [-1, 1, -1, 0],
  [-1, 1, -1, 1],
  [-1, 1, 0, -1],
  [-1, 1, 0, 0],
  [-1, 1, 0, 1],
  [-1, 1, 1, -1],
  [-1, 1, 1, 0],
  [-1, 1, 1, 1],
  [0, -1, -1, -1],
  [0, -1, -1, 0],
  [0, -1, -1, 1],
  [0, -1, 0, -1],
  [0, -1, 0, 0],
  [0, -1, 0, 1],
  [0, -1, 1, -1],
  [0, -1, 1, 0],
  [0, -1, 1, 1],
  [0, 0, -1, -1],
  [0, 0, -1, 0],
  [0, 0, -1, 1],
  [0, 0, 0, -1],
  [0, 0, 0, 1],
  [0, 0, 1, -1],
  [0, 0, 1, 0],
  [0, 0, 1, 1],
  [0, 1, -1, -1],
  [0, 1, -1, 0],
  [0, 1, -1, 1],
  [0, 1, 0, -1],
  [0, 1, 0, 0],
  [0, 1, 0, 1],
  [0, 1, 1, -1],
  [0, 1, 1, 0],
  [0, 1, 1, 1],
  [1, -1, -1, -1],
  [1, -1, -1, 0],
  [1, -1, -1, 1],
  [1, -1, 0, -1],
  [1, -1, 0, 0],
  [1, -1, 0, 1],
  [1, -1, 1, -1],
  [1, -1, 1, 0],
  [1, -1, 1, 1],
  [1, 0, -1, -1],
  [1, 0, -1, 0],
  [1, 0, -1, 1],
  [1, 0, 0, -1],
  [1, 0, 0, 0],
  [1, 0, 0, 1],
  [1, 0, 1, -1],
  [1, 0, 1, 0],
  [1, 0, 1, 1],
  [1, 1, -1, -1],
  [1, 1, -1, 0],
  [1, 1, -1, 1],
  [1, 1, 0, -1],
  [1, 1, 0, 0],
  [1, 1, 0, 1],
  [1, 1, 1, -1],
  [1, 1, 1, 0],
  [1, 1, 1, 1],
];

const ACTIVE = "#";
const INACTIVE = ".";

/**
 * Main function for Part 2
 *
 * @param {string} data - Puzzle input as a single string.
 */
async function main(data) {
  let field = Array(SIZE)
    .fill()
    .map((_) =>
      Array(SIZE)
        .fill()
        .map((_) =>
          Array(SIZE)
            .fill()
            .map((_) =>
              Array(SIZE)
                .fill()
                .map((_) => false)
            )
        )
    );
  const input = data
    .split(/\r?\n/)
    .map((line) => line.split("").map((char) => char === ACTIVE));

  for (let y = 0; y < input.length; y++) {
    for (let x = 0; x < input[y].length; x++) {
      // Place in the middle instead of at the edge
      const rX = Math.floor(SIZE / 2 - input.length / 2 + x);
      const rY = Math.floor(SIZE / 2 - input.length / 2 + y);
      field[Math.floor(SIZE / 2 - 1)][Math.floor(SIZE / 2 - 1)][rY][rX] =
        input[y][x];
    }
  }

  for (let i = 0; i < 6; i++) {
    /** @type {boolean[][][][]} */
    const newField = JSON.parse(JSON.stringify(field));
    for (let x = 0; x < SIZE; x++) {
      for (let y = 0; y < SIZE; y++) {
        for (let z = 0; z < SIZE; z++) {
          for (let w = 0; w < SIZE; w++) {
            let neighbors = 0;
            for (const [dX, dY, dZ, dW] of NEIGHBORS_4D) {
              if (
                x + dX < 0 ||
                x + dX >= SIZE ||
                y + dY < 0 ||
                y + dY >= SIZE ||
                z + dZ < 0 ||
                z + dZ >= SIZE ||
                w + dW < 0 ||
                w + dW >= SIZE
              )
                continue;
              if (field[w + dW][z + dZ][y + dY][x + dX]) neighbors++;
            }
            if (field[w][z][y][x]) {
              if (neighbors !== 2 && neighbors !== 3) {
                newField[w][z][y][x] = false;
              }
            } else {
              if (neighbors === 3) {
                newField[w][z][y][x] = true;
              }
            }
          }
        }
      }
    }
    field = newField;
  }

  return field.flat(3).filter((val) => val).length;
}

fs.readFile("input", (err, data) => {
  console.time("Part 2");
  if (err) throw err;
  main(data.toString())
    .then((...args) => {
      console.timeEnd("Part 2");
      console.log(...args);
    })
    .catch(console.error);
});
