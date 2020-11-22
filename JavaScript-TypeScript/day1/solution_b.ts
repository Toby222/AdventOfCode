import * as fs from "fs";
import * as readline from "readline";

async function main() {
  const readStream = fs.createReadStream("input");
  const lines = readline.createInterface(readStream);
  const fuelReqs: number[] = [];

  for await (const line of lines) {
    let mass = parseInt(line, 10);
    if (isNaN(mass)) {
      throw new Error(`Invalid value ${mass}`);
    }
    mass = Math.floor(mass / 3) - 2;

    while (mass > 0) {
      fuelReqs.push(mass);
      mass = Math.floor(mass / 3) - 2;
    }
  }

  return fuelReqs.reduce((x, y) => x + y, 0);
}

main().then(console.log);
