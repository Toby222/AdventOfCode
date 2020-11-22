import * as fs from "fs";
const input = fs.readFileSync("input").toString();

async function main() {
  let orbits = new Map<string, string>();
  input.split("\r\n").forEach((line) => orbits.set(line.split(")")[1], line.split(")")[0]));

  let distanceToYou = new Map<string, number>();
  let orbit = orbits.get("YOU")!;
  let distance = 0;

  while (true) {
    distanceToYou.set(orbit, distance);
    const newOrbit = orbits.get(orbit);
    if (newOrbit !== undefined) {
      orbit = newOrbit;
      distance++;
    } else {
      break;
    }
  }

  orbit = orbits.get("SAN")!;
  distance = 0;

  while (true) {
    let pathValue = distanceToYou.get(orbit);
    if (pathValue !== undefined) {
      distance += pathValue;
      break;
    }
    orbit = orbits.get(orbit)!;
    distance++;
  }
  return distance;
}

main().then(console.log).catch(console.error);
