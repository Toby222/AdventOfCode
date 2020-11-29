import * as fs from "fs";
const input = fs.readFileSync("input").toString();

async function main() {
  const lines = input.split("\n").map((line) => line.trim());
  return lines.reduce((prev, line) => prev + (line.length - eval(line).length), 0);
}

main().then(console.log).catch(console.error);
