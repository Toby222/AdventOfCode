import * as fs from "fs";
const fsPromises = fs.promises;
import * as readline from "readline";

const rl = readline.createInterface(process.stdin, process.stdout);

enum ParameterMode {
  position = 0,
  immediate = 1,
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

  async run(simulatedInput: string[], debug: boolean = false) {
    let index = 0;
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
            `${(++debugIteration).toString().padStart(5, "0")};${index.toString().padStart(5, "0")};[${this.memory
              .map((op) => op.literalValue)
              .join(",")}]\n`
          );
        }
        const opCode = this.memory[index];
        if (!opCode.executable) {
          throw new Error(`Tried executing non-executable value ${this.memory[index]} at index ${index}`);
        }

        const parameterModes = opCode.parameterModes!;
        const valueA = this.memory[index + 1]?.literalValue ?? 0;
        const valueB = this.memory[index + 2]?.literalValue ?? 0;
        const valueC = this.memory[index + 3]?.literalValue ?? 0;
        const paramA = parameterModes[2] === ParameterMode.immediate ? valueA : this.memory[valueA]?.literalValue ?? 0;
        const paramB = parameterModes[1] === ParameterMode.immediate ? valueB : this.memory[valueB]?.literalValue ?? 0;
        const paramC = parameterModes[0] === ParameterMode.immediate ? valueC : this.memory[valueC]?.literalValue ?? 0;

        /*console.debug(
          opCode,
          this.memory.slice(index, index + 4).map((val) => val.literalValue)
        );*/

        switch (opCode.instruction) {
          case Instruction.add:
            if (debug) {
              console.debug(`Adding ${paramA}+${paramB} and writing ${paramA + paramB} to ${valueC}`);
            }
            this.memory[valueC] = new OpCode(paramA + paramB);
            index += 4;
            break;
          case Instruction.mul:
            if (debug) {
              console.debug(`Multiplying ${paramA}*${paramB} and writing ${paramA * paramB} to ${valueC}`);
            }
            this.memory[valueC] = new OpCode(paramA * paramB);
            index += 4;
            break;
          case Instruction.inp:
            /*this.memory[valueA] = await new Promise<OpCode>((resolve, reject) => {
              rl.question(`[${index.toString().padStart(5, "0")}]> `, (answer) => {
                try {
                  const op = new OpCode(answer);
                  resolve(op);
                } catch (e) {
                  reject(e);
                }
              });
            });*/
            if (simulatedInput.length === 0) throw new Error("Asked for more input than was given!");
            if (debug) console.debug(simulatedInput, simulatedInput[0]);
            let inputValue = simulatedInput.shift()!;
            if (debug) {
              console.debug(`Writing opCode with value ${inputValue} to ${valueA}`);
            }
            this.memory[valueA] = new OpCode(inputValue);
            index += 2;
            break;
          case Instruction.out:
            if (debug) {
              console.debug(`Outputting ${paramA}`);
            }
            output.push(paramA);
            index += 2;
            break;
          case Instruction.jnz:
            if (debug) {
              console.debug(`Testing if ${paramA}!=0, if so, jumping to ${paramB}`);
            }
            if (paramA !== 0) {
              index = paramB;
            } else {
              index += 3;
            }
            break;
          case Instruction.jz:
            if (debug) {
              console.debug(`Testing if ${paramA}==0, if so, jumping to ${paramB}`);
            }
            if (paramA === 0) {
              index = paramB;
            } else {
              index += 3;
            }
            break;
          case Instruction.lt:
            if (debug) {
              console.debug(`Testing if ${paramA}<${paramB}, writing result to ${valueC}`);
            }
            this.memory[valueC] = new OpCode(paramA < paramB ? 1 : 0);
            index += 4;
            break;
          case Instruction.eq:
            if (debug) {
              console.debug(`Testing if ${paramA}==${paramB}, writing result to ${valueC}`);
            }
            this.memory[valueC] = new OpCode(paramA === paramB ? 1 : 0);
            index += 4;
            break;
          case Instruction.end:
            return output;
          default:
            throw new Error(`HCF: Not implemented <${opCode.instruction}>`);
        }
      }
    } catch (e) {
      // console.debug(`Execution failed at index ${index}. Dumping memory.`);
      if (debug) {
        debugLog!.write(
          `${debugIteration.toString().padStart(5, "0")};${index.toString().padStart(5, "0")};[${this.memory
            .map((op) => op.literalValue)
            .join(",")}]\n`
        );
      }
      throw e;
    }
    return output;
  }

  reset() {
    this.memory = this.initialMemory;
  }

  static memdump(memory: Memory) {
    return memory.map((op) => op.toString()).join(",");
  }
}

function permute<T>(inputArr: T[]) {
  let result: T[][] = [];

  function _permute(arr: T[], m: T[] = []) {
    if (arr.length === 0) {
      result.push(m);
    } else {
      for (let i = 0; i < arr.length; i++) {
        let curr = arr.slice();
        let next = curr.splice(i, 1);
        _permute(curr.slice(), m.concat(next));
      }
    }
  }

  _permute(inputArr);

  return result;
}

async function main(code: string) {
  const memdump: Memory = code.split(",").map((val) => new OpCode(val));
  const machine = new Machine(memdump);

  let maxResult: number = -Infinity;
  const permutations = permute([4, 3, 2, 1, 0].map((n) => n.toString()));
  for (const value of permutations) {
    try {
      let curResult = 0;
      let result: number = -Infinity;
      for (const setting of value) {
        result = (await machine.run([setting, curResult.toString()], maxResult === -Infinity))[0];
        curResult = result;
      }
      maxResult = Math.max(result, maxResult);
      machine.reset();
      console.debug("Testing", value, "got", result);
    } catch {
      console.error(value);
    }
  }
  rl.close();
  return maxResult;
}

const input = fs.readFileSync("input").toString();
main(input)
  .then(console.log)
  .catch((...args) => {
    rl.close();
    console.error(args);
  });
