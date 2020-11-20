import {Grid} from './grid'
import * as fs from "fs";
const input = fs.readFileSync("input").toString()
.split('\n')
.map(row => row.split('').map(v => (v === '#' ? 1 : 0)));

async function main() {
  const grid = new Grid(input)
  return grid.vaporizeAsteroidsFrom(grid.getAsteroidWithHighestCountInLineOfSight().best_coords)
}

main().then(console.log).catch(console.error);
