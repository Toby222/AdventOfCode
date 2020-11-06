import * as fs from "fs";
import * as readline from "readline";

async function main() {
  const readStream = fs.createReadStream("input");
  const lines = readline.createInterface(readStream);
  const fuelReqs: number[] = [];

  for await (const line of lines) {
    const mass = parseInt(line, 10);
    if (isNaN(mass)) {
      throw new Error(`Invalid value ${mass}`);
    }

    fuelReqs.push(Math.floor(mass / 3) - 2);
  }
  return fuelReqs.reduce((x, y, i) => x + y);
}

main().then(console.log);
