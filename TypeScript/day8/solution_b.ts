import * as fs from "fs";
const input = fs.readFileSync("input").toString();

async function main() {
  const lines = input.split("\n").map((line) => line.trim());
  let result = 0;
  for (const line of lines) {
    const replaced = '"' + line.replace(/\\/g, "\\\\").replace(/"/g, '\\"') + '"';
    result += replaced.length - line.length;
    console.log(line, replaced);
  }
  return result;
}

main().then(console.log).catch(console.error);
