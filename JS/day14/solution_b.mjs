import fs from "fs";

/**
 * @param mask
 * @param val
 */
function getValue(mask, val) {
  const binaryVal = val.toString(2).padStart(mask.length, "0");
  let valStr = "";
  for (let i = 0; i < mask.length; i++) {
    valStr += mask[i] === "X" ? binaryVal[i] : mask[i];
  }
  return parseInt(valStr, 2);
}

/**
 * @param mask
 */
function getAllMasks(mask) {
  if (!mask.includes("X")) return mask;

  return [
    getAllMasks(mask.replace("X", "0")),
    getAllMasks(mask.replace("X", "1")),
  ].flat();
}

/**
 * @param originalMask
 * @param replacedMasks
 * @param address
 */
function getDecodedAddresses(originalMask, replacedMasks, address) {
  return replacedMasks.map((mask) => {
    const binaryAddr = address.toString(2).padStart(mask.length, "0");
    let addrStr = "";
    for (let i = 0; i < mask.length; i++) {
      addrStr += originalMask[i] === "0" ? binaryAddr[i] : mask[i];
    }

    return parseInt(addrStr, 2);
  });
}

/**
 * Main function for Puzzle B
 *
 * @param {string} data - Puzzle input as a single string.
 */
async function main(data) {
  const instructions = data.split(/\r?\n/).map((line) => line.split(" = "));
  let memory = {}
  let originalMask
  let replacedMasks

  instructions.forEach(([op, args]) =>{
    if(op==="mask") {
      originalMask = args;
      replacedMasks = getAllMasks(originalMask);
    } else {
      const originalAddress = parseInt(op.match(/mem\[(\d+)\]/)[1], 10)
      const value = parseInt(args)

      getDecodedAddresses(originalMask, replacedMasks, originalAddress).forEach((decodedAddress) => {
        memory[decodedAddress] = value;
      })
    }
  })

  return Object.values(memory).reduce((cur, prev) => cur + prev, 0)
}

fs.readFile("input", (err, data) => {
  if (err) throw err;
  main(data.toString()).then(console.log).catch(console.error);
});
