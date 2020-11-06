import * as fs from "fs";
const input = fs
  .readFileSync("input")
  .toString()
  .split("\n")
  .map((inp) => inp.split(","));

async function trace(ops: string[]) {
  let board: number[][] = [];
  let steps = 0;
  let x = 0;
  let y = 0;

  for (const op of ops) {
    const dir = op.substring(0, 1);
    let len = parseInt(op.substring(1));
    switch (dir) {
      case "R":
        while (len-- > 0) {
          (board[y] ?? (board[y] = []))[++x] = Math.min(
            board[y][x] ?? Infinity,
            ++steps
          );
        }
        break;
      case "L":
        while (len-- > 0) {
          (board[y] ?? (board[y] = []))[--x] = Math.min(
            board[y][x] ?? Infinity,
            ++steps
          );
        }
        break;
      case "U":
        while (len-- > 0) {
          y++;
          (board[y] ?? (board[y] = []))[x] = Math.min(
            board[y][x] ?? Infinity,
            ++steps
          );
        }
        break;
      case "D":
        while (len-- > 0) {
          y--;
          (board[y] ?? (board[y] = []))[x] = Math.min(
            board[y][x] ?? Infinity,
            ++steps
          );
        }
        break;
    }
  }

  return board;
}

async function main() {
  const [wireA, wireB] = [await trace(input[0]), await trace(input[1])];
  const crossings: [y: number, x: number, val: number][] = [];

  for (let y = 0; y < Math.min(wireA.length, wireB.length); y++) {
    for (let x = 0; x < Math.min(wireA[y].length, wireB[y].length); x++) {
      if (wireA[y][x] > 0 && wireB[y][x] > 0) {
        crossings.push([y, x, wireA[y][x] + wireB[y][x]]);
      }
    }
  }
  return crossings.sort(
    (crossingA, crossingB) => crossingA[2] - crossingB[2]
  )[0];
}

main().then(console.log);
