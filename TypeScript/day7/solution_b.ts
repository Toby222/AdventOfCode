import fs from "fs";
const input = fs.readFileSync("input").toString().split("\n");
const COMMAND_REGEX = /[A-Z]+/g;
const ARGUMENTS_REGEX = /[a-z0-9]+/g;

// Dictionary of our bitwise methods
const BITWISE_METHODS = {
  AND: (a: number, b: number) => a & b,
  OR: (a: number, b: number) => a | b,
  NOT: (a: number, b?: number) => ~a,
  LSHIFT: (a: number, b: number) => a << b,
  RSHIFT: (a: number, b: number) => a >> b,
};

// Parse instruction from input and return object with command, arguments and destination wire
function parseInstruction(instruction: string): Instruction & { destination: string } {
  const raw_command = instruction.match(COMMAND_REGEX);
  const args = instruction.match(ARGUMENTS_REGEX) ?? [];
  const destination = args.pop()!;
  const command = raw_command === null ? raw_command : (raw_command[0] as keyof typeof BITWISE_METHODS);

  return {
    command,
    args: args.map((arg) => (isNaN(Number(arg)) ? arg : Number(arg))) as [Wire | number, Wire | number | undefined],
    destination,
  };
}

type Wire = string;
type Instruction = {
  command: keyof typeof BITWISE_METHODS | null;
  args: [Wire | number, Wire | number | undefined];
};

// Calculate value for one of the wires (recursively)
function calculateWire(wireName: Wire, wires: Map<Wire, any>) {
  const wire = wires.get(wireName);

  if (typeof wireName === "number") return wireName;
  if (typeof wire === "number") return wire;
  if (typeof wire === "undefined") return undefined;

  if (!wire.command) {
    wires.set(wireName, calculateWire(wire.args[0], wires));
  } else if (Object.keys(BITWISE_METHODS).includes(wire.command)) {
    wires.set(wireName, BITWISE_METHODS[wire.command as keyof typeof BITWISE_METHODS](calculateWire(wire.args[0], wires), calculateWire(wire.args[1], wires)));
  }

  return wires.get(wireName);
}
async function main() {
  // Our parsed wires in format {wire: value} or {wire: instruction}
  const wires = new Map<Wire, Instruction>();

  // Fill WIRES with parsed instructions and their future values
  input.forEach((instruction) => {
    const parsedInstruction = parseInstruction(instruction.trim());
    wires.set(parsedInstruction.destination, { command: parsedInstruction.command, args: parsedInstruction.args });
  });
  const parsedInstruction = parseInstruction("46065 -> b");
  wires.set(parsedInstruction.destination, { command: parsedInstruction.command, args: parsedInstruction.args });

  return calculateWire("a", wires);
}

main().then(console.log).catch(console.error);
