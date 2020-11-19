import * as fs from "fs";
const input = fs.readFileSync("input").toString();

async function main() {
  let layers = input.replace(/(.{150})/g, "$1,").split(",");
  let layers2: [number, number, number, string][] = layers.map((layer) => [
    layer.match(/0/g)?.length ?? 0,
    layer.match(/1/g)?.length ?? 0,
    layer.match(/2/g)?.length ?? 0,
    layer,
  ]);
  let selected = layers2.sort((a, b) => a[0] - b[0])[1];

  return [selected, selected[0] + selected[1] + selected[2], selected[1] * selected[2]];
}

main().then(console.log).catch(console.error);
