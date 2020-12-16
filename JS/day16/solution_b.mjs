import fs from "fs";
import { main as PartOneMain } from "./solution_a.mjs";

/**
 * Main function for Puzzle B
 *
 * @param {string} data - Puzzle input as a single string.
 */
async function main(data) {
  /** @type {[partOneResult: number, {invalidTicketIds: number[], myTicket: number[], nearbyTickets: number[][], parsedRules: [keyName: string, [[number, number],[number, number]]][]}]} */
  const [partOneResult, {
    invalidTicketIds,
    myTicket,
    nearbyTickets: unfilteredNearbyTickets,
    parsedRules,
  }] = (await PartOneMain(data));

  console.time("Part 2")
  /** @type {Map<string, Map<number, number>>} */
  const count = new Map();
  const nearbyTickets = unfilteredNearbyTickets.filter(
    (_, i) => !invalidTicketIds.includes(i)
  );

  for (const rule of parsedRules) {
    for (const ticket of nearbyTickets) {
      for (let k = 0; k < ticket.length; k++) {
        if (
          (ticket[k] >= rule[1][0][0] && ticket[k] <= rule[1][0][1]) ||
          (ticket[k] >= rule[1][1][0] && ticket[k] <= rule[1][1][1])
        ) {
          const map = count.get(rule[0]) ?? new Map();
          map.set(k, (map.get(k) ?? 0) + 1);
          if (!count.has(rule[0])) count.set(rule[0], map);
        }
      }
    }
  }

  /** @type {string[]} */
  let done = [];
  let currentIndex = NaN;
  let currentKey = "";

  /** @type {Map<string, number[]>} */
  const indices = new Map();
  for (const [countedKey, occurenceCounter] of count.entries()) {
    const occurences = Array.from(occurenceCounter.entries())
      .filter((entry) => entry[1] === Math.max(...occurenceCounter.values()))
      .map((el) => el[0]);
    indices.set(countedKey, occurences);

    if (occurences.length === 1) {
      currentIndex = occurences[0];
      currentKey = countedKey;
    }
  }

  while (true) {
    for (const key of indices.keys()) {
      if (key === currentKey) continue;
      indices.set(
        key,
        indices
          .get(key)
          .filter((possibleIndex) => possibleIndex !== currentIndex)
      );
    }
    done.push(currentKey);

    // Find next key that only has one possible place
    const single = Array.from(indices.entries()).find(
      ([key, value]) => !done.includes(key) && value.length === 1
    );
    if (!single) break;
    [currentKey, [currentIndex]] = single;
  }

  let result = 1
  for(const [key, [index]] of indices.entries()) {
    if(!key.startsWith("departure ")) continue
    result *= myTicket[index]
  }
  console.timeEnd("Part 2")
  return [partOneResult, result];
}

fs.readFile("input", (err, data) => {
  if (err) throw err;
  main(data.toString()).then(console.log).catch(console.error);
});
