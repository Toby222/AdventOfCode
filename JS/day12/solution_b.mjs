import fs from "fs";

/**
 * Main function for Puzzle B
 *
 * @param {string} data - Puzzle input as a single string.
 * @returns {Promise<number>} Manhattan distance of final position to starting position.
 */
async function main(data) {
  const orders = data
    .split(/\r?\n/)
    .map((line) => [line[0], parseInt(line.substring(1))]);

  const ship = [0, 0];

  let waypoint = [10, 1];

  for (const order of orders) {
    const radians = (-order[1] * Math.PI) / 180;
    switch (order[0]) {
      case "L":
        waypoint = [
          Math.round(
            waypoint[0] * Math.cos(-radians) - waypoint[1] * Math.sin(-radians)
          ),
          Math.round(
            waypoint[0] * Math.sin(-radians) + waypoint[1] * Math.cos(-radians)
          ),
        ];
        break;
      case "R":
        waypoint = [
          Math.round(
            waypoint[0] * Math.cos(radians) - waypoint[1] * Math.sin(radians)
          ),
          Math.round(
            waypoint[0] * Math.sin(radians) + waypoint[1] * Math.cos(radians)
          ),
        ];
        break;

      case "F":
        ship[0] += order[1] * waypoint[0];
        ship[1] += order[1] * waypoint[1];
        break;

      case "N":
        waypoint[1] += order[1];
        break;
      case "E":
        waypoint[0] += order[1];
        break;
      case "S":
        waypoint[1] -= order[1];
        break;
      case "W":
        waypoint[0] -= order[1];
        break;
    }
  }
  return Math.abs(ship[0]) + Math.abs(ship[1]);
}

fs.readFile("input", (err, data) => {
  if (err) throw err;
  main(data.toString()).then(console.log).catch(console.error);
});
