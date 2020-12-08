import fs from "fs";

/**
 * Main function for Puzzle A
 *
 * @param {string} data - Puzzle input as a single string.
 */
async function main(data) {
  /** @type {[string, number][]} */
  const asm = data
    .split(/\r?\n/)
    .map((line) => line.split(" "))
    .map((line) => [line[0], parseInt(line[1])]);

  let accumulator = 0;
  /** @type {Set<number>} */
  const visited = new Set();

  for (let i = 0; i < asm.length; i++) {
    if (!visited.has(i)) {
      visited.add(i);
      console.log(asm[i]);
      switch (asm[i][0]) {
        case "acc":
          accumulator += asm[i][1];
        case "nop":
          break;
        case "jmp":
          i += asm[i][1] - 1;
          break;
      }
    } else {
      break;
    }
  }

  return accumulator;
}

fs.readFile("input", (err, data) => {
  if (err) throw err;
  main(data.toString()).then(console.log).catch(console.error);
});
