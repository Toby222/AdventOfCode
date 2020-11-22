import * as fs from "fs";
const input = fs.readFileSync("input").toString();

async function main() {
  let image: string[] = Array(6 * 25).fill("2"); //Array(6).map(_=>Array(25))
  let layers = input.replace(/(.{150})/g, "$1,").split(",");

  for (const layer of layers) {
    for (let i = 0; i < layer.length; i++) {
      if(image[i] === "2") {
        image[i] = layer[i]
      }
    }
  }

  return image.join("").replace(/(.{25})/g, "$1\n");
}

main().then(console.log).catch(console.error);
