import fs from "fs";

/**
 * @param string
 * @param index
 * @param replacement
 */
function replaceAt(string, index, replacement) {
  return (
    string.substr(0, index) +
    replacement +
    string.substr(index + replacement.length)
  );
}

/**
 * Main function for Puzzle A
 *
 * @param {string} data - Puzzle input as a single string.
 */
async function main(data) {
  /**
   * 1 mem|mask
   * 2 x
   * 3 y
   * 4 new_mask
   */

  /** @type {["mask"|"mem", number?, number?, string?][]} */
  const input = data
    .split(/\r?\n/)
    .map((line) => line.match(/^(mem\[(\d+)\] = (\d+)|mask = ([10X]{36}))$/m))
    .map(([_, op, x, y, new_mask]) => [
      op.match(/^\w+/)[0],
      parseInt(x),
      parseInt(y),
      new_mask,
    ]);
  /** @type {number[]} */
  const memory = [];
  let mask = "";

  for (let [op, x, y, new_mask] of input) {
    op = op.split(" ")[0];
    if (op === "mask") {
      mask = new_mask;
    } else if (op === "mem") {
      let num = y.toString(2).padStart(36, "0");
      for (let i in mask) {
        i = parseInt(i);

        if (mask[i] === "X") continue;
        num = replaceAt(num, i, mask[i]);
      }
      y = parseInt(num, 2);
      memory[x] = y;
    } else {
      throw new Error("invalid op: " + op);
    }
  }

  return memory.reduce((prev, cur) => prev + cur, 0);
}

fs.readFile("input", (err, data) => {
  if (err) throw err;
  main(data.toString()).then(console.log).catch(console.error);
});
