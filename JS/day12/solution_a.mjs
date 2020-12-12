import fs from "fs";

/**
 * Main function for Puzzle A
 *
 * @param {string} data - Puzzle input as a single string.
 * @returns {Promise<number>} Manhattan distance of final position to starting position.
 */
async function main(data) {
  const orders = data
    .split(/\r?\n/)
    .map((line) => [line[0], parseInt(line.substring(1))]);

  let facing = 90;
  let position = [0, 0];

  for (const order of orders) {
    switch (order[0]) {
      case "R":
        facing = (facing + 360 + order[1]) % 360;
        break;
      case "L":
        facing = (facing + 360 - order[1]) % 360;
        break;
      case "F":
        const radians = (facing * Math.PI) / 180;
        position[0] += Math.sin(radians) * order[1];
        position[1] += Math.cos(radians) * order[1];
        break;

      case "N":
        position[1] += order[1];
        break;
      case "E":
        position[0] += order[1];
        break;
      case "S":
        position[1] -= order[1];
        break;
      case "W":
        position[0] -= order[1];
        break;
    }
  }
  return Math.abs(position[0]) + Math.abs(position[1]);
}

fs.readFile("input", (err, data) => {
  if (err) throw err;
  main(data.toString()).then(console.log).catch(console.error);
});
