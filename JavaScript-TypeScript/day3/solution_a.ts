import * as fs from "fs";
const input = fs
  .readFileSync("input")
  .toString()
  .split("\n")
  .map((inp) => inp.split(","));

async function trace(ops: string[]) {
  let board: boolean[][] = [];
  let x = 0;
  let y = 0;

  for (const op of ops) {
    const dir = op.substring(0, 1);
    let len = parseInt(op.substring(1));
    switch (dir) {
      case "R":
        while (len-- > 0) {
          (board[y] ?? (board[y] = []))[++x] = true;
        }
        break;
      case "L":
        while (len-- > 0) {
          (board[y] ?? (board[y] = []))[--x] = true;
        }
        break;
      case "U":
        while (len-- > 0) {
          y++;
          (board[y] ?? (board[y] = []))[x] = true;
        }
        break;
      case "D":
        while (len-- > 0) {
          y--;
          (board[y] ?? (board[y] = []))[x] = true;
        }
        break;
    }
  }

  return board;
}

async function main() {
  const [wireA, wireB] = [await trace(input[0]), await trace(input[1])];
  const crossings: [number, number][] = [];

  for (let y = 0; y < Math.min(wireA.length, wireB.length); y++) {
    for (let x = 0; x < Math.min(wireA[y].length, wireB[y].length); x++) {
      if (wireA[y][x] && wireB[y][x]) {
        crossings.push([y, x]);
      }
    }
  }
  return crossings;
}

main().then((crossings) =>
  console.log(Math.min(...crossings.map((coords) => coords[0] + coords[1])))
);
