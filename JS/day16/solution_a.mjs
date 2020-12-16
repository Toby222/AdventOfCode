import fs from "fs"

/**
 * Main function for Puzzle A
 *
 * @param {string} data - Puzzle input as a single string.
 * @returns {[errorRate: number, {invalidTicketIds: number[], myTicket: number[], nearbyTickets: number[][], parsedRules: [string, [[number, number],[number, number]]][]}]} Stuff
 */
export async function main(data) {
  console.time("Part 1")
  const blocks = data.split(/\r?\n\r?\n/).map((block) => block.split(/\r?\n/));

  /** @type {[string, [[number, number], [number, number]]][]} */
  const parsedRules = blocks[0]
    .map((line) => line.split(": "))
    .map((rule) => [
      rule[0],
      rule[1]
        .split(" or ")
        .map((val) => val.split("-").map((num) => parseInt(num))),
    ]);
  /** @type {[number, number][]} */
  const rules = parsedRules.flatMap((rule) => rule[1]);
  const myTicket = blocks[1][1].split(",").map((val) => parseInt(val));
  const nearbyTickets = blocks[2]
    .slice(1)
    .map((line) => line.split(",").map((val) => parseInt(val)));

  /** @type {number[]} */
  const invalidTicketIds = [];
  let errorRate = 0;
  nearbyTickets.forEach((ticket, i) => {
    for (let k = 0; k < ticket.length; k++) {
      if (!rules.some((rule) => ticket[k] >= rule[0] && ticket[k] <= rule[1])) {
        invalidTicketIds.push(i);
        errorRate += ticket[k];
      }
    }
  });

  console.timeEnd("Part 1")
  return [
    errorRate,
    { invalidTicketIds, myTicket, nearbyTickets, parsedRules },
  ];
}

/*
fs.readFile("input", (err, data) => {
  if (err) throw err;
  main(data.toString()).then(console.log).catch(console.error);
});
*/
