import * as fs from "fs";
import { relative } from "path";
const fsPromises = fs.promises;
import * as readline from "readline";
import { factory } from "typescript";

const rl = readline.createInterface(process.stdin, process.stdout);

enum ParameterMode {
  position = 0,
  immediate = 1,
  relative = 2,
}

enum Instruction {
  add = 1,
  mul = 2,
  inp = 3,
  out = 4,
  jnz = 5,
  jz = 6,
  lt = 7,
  eq = 8,
  rel = 9,
  end = 99,
}

type Memory = Array<OpCode>;

class OpCode {
  private _executable: boolean = false;
  private _literalValue: number = 0;
  private _instruction?: Instruction;
  private _parameterModes?: [a: ParameterMode, b: ParameterMode, c: ParameterMode];

  constructor(value: number);
  constructor(value: string);
  constructor(value: string | number) {
    let valueNumber: number;
    let valueString: string;
    if (typeof value === "number") {
      valueNumber = value;
      valueString = value.toString();
    } else {
      valueString = value;
      valueNumber = parseInt(value);
    }
    if (isNaN(valueNumber)) throw new Error(`Invalid value '${value}' for OpCode.`);
    valueString = valueString.padStart(5, "0");

    this._literalValue = valueNumber;

    if (valueNumber >= 0 && valueNumber <= 99999 && valueNumber % 100 in Instruction) {
      const parameterModes: [ParameterMode, ParameterMode, ParameterMode] = [
        Math.floor(valueNumber / 10000) % 10,
        Math.floor(valueNumber / 1000) % 10,
        Math.floor(valueNumber / 100) % 10,
      ];
      if (parameterModes.filter((val) => val in ParameterMode).length === 3) {
        this._parameterModes = parameterModes;
        this._instruction = valueNumber % 100;
      } else {
        return;
      }

      this._executable = true;
    }
  }

  get executable() {
    if (this._instruction === undefined || this._parameterModes === undefined) {
      this._executable = false;
    }
    return this._executable;
  }

  get literalValue() {
    return this._literalValue;
  }
  get instruction() {
    return this._instruction;
  }
  get parameterModes() {
    return this._parameterModes;
  }

  toString() {
    return this.literalValue.toString();
  }
}

class Machine {
  private readonly initialMemory: Memory;
  private memory: Memory;
  constructor(program: Memory) {
    this.initialMemory = program;
    this.memory = program;
  }

  parseParam(value: number, mode: ParameterMode, relativeOffset: number) {
    switch (mode) {
      case ParameterMode.immediate:
        return value;
      case ParameterMode.position:
        return this.memory[value]?.literalValue ?? 0;
      case ParameterMode.relative:
        return this.memory[value + relativeOffset]?.literalValue ?? 0;
    }
  }

  async run(
    inputHandler: (context: Context) => Promise<OpCode>,
    outputHandler: (context: Context) => Promise<unknown>,
    debug: boolean = false
  ) {
    let index = 0;
    let relative = 0;
    let debugIteration = 0;
    const debugLog = debug ? await fsPromises.open("out.log", "w") : undefined;
    let output: number[] = [];
    try {
      while (index < this.memory.length) {
        /*for (let i = 0; i < this.memory.length; i++) {
          this.memory[i] = this.memory[i] ?? new OpCode(0);
        }*/
        if (debug) {
          debugLog!.write(
            `${(++debugIteration).toString().padStart(5, "0")};${relative
              .toString()
              .padStart(8, "0")};${index.toString().padStart(5, "0")};[${this.memory
              .map((op) => op.literalValue)
              .join(",")}]\n`
          );
        }
        const opCode = this.memory[index];
        if (!opCode.executable) {
          throw new Error(`Tried executing non-executable value ${opCode.literalValue} at index ${index}`);
        }

        const parameterModes = opCode.parameterModes!;
        const valueA = this.memory[index + 1]?.literalValue ?? 0;
        const valueB = this.memory[index + 2]?.literalValue ?? 0;
        const valueC = this.memory[index + 3]?.literalValue ?? 0;

        const paramA = this.parseParam(valueA, parameterModes[2 - 0], relative);
        const paramB = this.parseParam(valueB, parameterModes[2 - 1], relative);
        const paramC = this.parseParam(valueC, parameterModes[2 - 2], relative);

        if (debug) {
          console.debug(`${opCode.literalValue}(${valueA}, ${valueB}, ${valueC});${this.memory[1028]?.literalValue}`);
        }

        const context: Context = {
          parameterModes,
          valueA,
          valueB,
          valueC,
          paramA,
          paramB,
          paramC,
          debug,
          index,
          relative,
          opCode,
          output,
        };

        switch (opCode.instruction) {
          case Instruction.add:
            if (debug) {
              console.debug(
                `Adding ${paramA}+${paramB} and writing ${paramA + paramB} to ${valueC} - OpCode ${opCode.literalValue}`
              );
            }
            if (parameterModes[2 - 2] === ParameterMode.relative) {
              this.memory[valueC + relative] = new OpCode(paramA + paramB);
            } else {
              this.memory[valueC] = new OpCode(paramA + paramB);
            }
            index += 4;
            break;
          case Instruction.mul:
            if (debug) {
              console.debug(
                `Multiplying ${paramA}*${paramB} and writing ${paramA * paramB} to ${valueC} - OpCode ${
                  opCode.literalValue
                }`
              );
            }
            if (parameterModes[2 - 2] === ParameterMode.relative) {
              this.memory[valueC + relative] = new OpCode(paramA * paramB);
            } else {
              this.memory[valueC] = new OpCode(paramA * paramB);
            }
            index += 4;
            break;
          case Instruction.inp:
            let input = await inputHandler(context);

            if (parameterModes[2 - 0] === ParameterMode.relative) {
              this.memory[valueA + relative] = input;
            } else {
              this.memory[valueA] = input;
            }

            index += 2;
            break;
          case Instruction.out:
            outputHandler(context);
            index += 2;
            break;
          case Instruction.jnz:
            if (debug) {
              console.debug(`Testing if ${paramA}!=0, if so, jumping to ${paramB} - OpCode ${opCode.literalValue}`);
            }
            if (paramA !== 0) {
              index = paramB;
            } else {
              index += 3;
            }
            break;
          case Instruction.jz:
            if (debug) {
              console.debug(`Testing if ${paramA}==0, if so, jumping to ${paramB} - OpCode ${opCode.literalValue}`);
            }
            if (paramA === 0) {
              index = paramB;
            } else {
              index += 3;
            }
            break;
          case Instruction.lt:
            if (debug) {
              console.debug(
                `Checking if ${paramA}<${paramB} and writing result to ${
                  parameterModes[2 - 2] === ParameterMode.relative ? valueC + relative : valueC
                } - OpCode ${opCode.literalValue}`
              );
            }
            if (parameterModes[2 - 2] === ParameterMode.relative) {
              this.memory[valueC + relative] = new OpCode(paramA < paramB ? 1 : 0);
            } else {
              this.memory[valueC] = new OpCode(paramA < paramB ? 1 : 0);
            }
            index += 4;
            break;
          case Instruction.eq:
            if (debug) {
              console.debug(
                `Checking if ${paramA}==${paramB} and writing result to ${
                  parameterModes[2 - 2] === ParameterMode.relative ? valueC + relative : valueC
                } - OpCode ${opCode.literalValue}`
              );
            }
            if (parameterModes[2 - 2] === ParameterMode.relative) {
              this.memory[valueC + relative] = new OpCode(paramA === paramB ? 1 : 0);
            } else {
              this.memory[valueC] = new OpCode(paramA === paramB ? 1 : 0);
            }
            index += 4;
            break;
          case Instruction.rel:
            if (debug) {
              console.debug(
                `Adjusting relative offset from ${relative} +${paramA} to ${relative + paramA} - OpCode ${
                  opCode.literalValue
                }`
              );
            }
            relative += paramA;
            index += 2;
            break;
          case Instruction.end:
            // console.log("Final output:", output);
            return this.memory;
        }
      }
    } catch (e) {
      console.debug(`Execution failed at index ${index}. Dumping memory.`);
      if (debug) {
        debugLog!.write(
          `${debugIteration.toString().padStart(5, "0")};${index.toString().padStart(5, "0")};[${this.memory
            .map((op) => op.literalValue)
            .join(",")}]\n`
        );
      }
      throw e;
    }
    // console.log("Final output:", output);
    return this.memory;
  }

  reset() {
    this.memory = this.initialMemory;
  }

  static memdump(memory: Memory) {
    return memory.map((op) => op.toString()).join(",");
  }
}

interface Context {
  parameterModes: [ParameterMode, ParameterMode, ParameterMode];
  valueA: number;
  valueB: number;
  valueC: number;
  paramA: number;
  paramB: number;
  paramC: number;
  debug: boolean;
  index: number;
  relative: number;
  opCode: OpCode;
  output: number[];
}

async function main(code: string): Promise<[number, string]> {
let painted = new Set<string>();
const memdump: Memory = code.split(",").map((val) => new OpCode(val));

  const machine = new Machine(memdump);

  async function input(context: Context) {
    if (context.debug) {
      console.debug("inputting", field[coords[1]][coords[0]] === "⬜" ? 1 : 0);
    }
    return new OpCode(field[coords[1]][coords[0]] === "⬜" ? 1 : 0);
  }

  async function output(context: Context) {
    if (context.debug) {
      console.debug(`Outputting ${context.paramA} - OpCode ${context.opCode.literalValue}`);
    }
    context.output.push(context.paramA);
    if (awaiting === "color") {
      awaiting = "turning";
      const newColor = context.paramA === 1 ? "⬜" : "⬛";
      field[coords[1]][coords[0]] = newColor;
    } else {
      awaiting = "color";
      let turning = context.paramA === 0 ? -1 : context.paramA;

      if (!painted.has(`${coords[0]};${coords[1]}`)) {
        painted.add(`${coords[0]};${coords[1]}`);
      }

      facing = ((facing + turning + 4) % 4) as 0 | 1 | 2 | 3;
      switch (facing) {
        case 0:
          coords[1] = coords[1] - 1;
          break;
        case 1:
          coords[0] = coords[0] + 1;
          break;
        case 2:
          coords[1] = coords[1] + 1;
          break;
        case 3:
          coords[0] = coords[0] - 1;
          break;
      }
    }
  }

  let field: ("⬜" | "⬛")[][] = Array<("⬜" | "⬛")[]>(7)
    .fill([])
    .map((_) => Array(43).fill("⬛"));
  let coords: [number, number] = [0, 1];
  let facing: 0 | 1 | 2 | 3 = 0;
  let awaiting: "turning" | "color" = "color";
  field[coords[1]][coords[0]] = "⬜"

  await machine.run(input, output);
  rl.close();
  return [painted.size, field.map((line) => line.join("")).join("\n")];
}
const input = fs.readFileSync("input").toString();
main(input)
  .then(([count, field]) => {
    console.log(count);
    console.log(field);
  })
  .catch((...args) => {
    rl.close();
    console.error(args);
  });
