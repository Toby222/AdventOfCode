import * as fs from "fs";
const input = fs.readFileSync("input").toString();

function verifyNotDecreasing(pass: string) {
  const digits = pass.split("").map((digit) => parseInt(digit));
  for (let i = 0; i < digits.length - 1; i++) {
    if (digits[i + 1] < digits[i]) {
      return false;
    }
  }
  return true;
}

function verifyHasDoubleButNotMore(pass: string) {
  const digits = pass.split("").map((digit) => parseInt(digit));
  for (let i = 0; i < digits.length; i++) {
    if (
      digits[i] === digits[i + 1] &&
      digits[i] !== digits[i + 2] &&
      digits[i] !== digits[i - 1]
    ) {
      return true;
    }
  }
  return false;
}

function verifyPass(pass: number) {
  return (
    pass >= 1e5 &&
    verifyNotDecreasing(pass.toString()) &&
    verifyHasDoubleButNotMore(pass.toString())
  );
}

async function main() {
  const [min, max] = input.split("-").map((val) => parseInt(val));
  let results = 0;
  for (let i = min; i <= max; i++) {
    if (verifyPass(i)) {
      results++;
    }
  }
  return results;
}

main().then(console.log);
