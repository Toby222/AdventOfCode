import fs from "fs";

/**
 * @typedef Instruction A single program instruction
 * @type { object }
 * @property { string } instruction Instruction to execute
 * @property { number } argument Argument to the instruction
 */

/**
 * Main function for Puzzle B
 *
 * @param {string} data - Puzzle input as a single string.
 */
async function main(data) {
  /** @type {Instruction[]} */
  const orig_asm = data
    .split(/\r?\n/)
    .map((line) => line.split(" "))
    .map((line) => ({ instruction: line[0], argument: parseInt(line[1]) }));

  let accumulator = 0;
  let finished = false;
  change_op_loop: for (let a = 0; a < orig_asm.length && !finished; a++) {
    // Reset
    accumulator = 0;
    /** @type {Set<number>} */
    const visited = new Set();
    const asm = orig_asm.slice();
    const orig_instruction = asm[a].instruction;
    //console.log(a, "before", asm[a])
    switch (asm[a].instruction) {
      case "acc":
        continue;
      case "nop":
        asm[a].instruction = "jmp";
        break;
      case "jmp":
        asm[a].instruction = "nop";
        break;
    }
    //console.log(a, "after", asm[a], "\n")

    let i = 0;
    while (i < asm.length) {
      if (visited.has(i)) {
        asm[a].instruction = orig_instruction;
        continue change_op_loop;
      }
      visited.add(i);
      switch (asm[i].instruction) {
        case "acc":
          accumulator += asm[i].argument;
        case "nop":
          i++;
          break;
        case "jmp":
          i += asm[i].argument;
          break;
        default:
          throw new Error("Wtf");
      }
    }
    finished = true;
  }

  return finished ? accumulator : NaN;
}

fs.readFile("input", (err, data) => {
  if (err) throw err;
  main(data.toString()).then(console.log).catch(console.error);
});
