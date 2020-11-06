import * as fs from 'fs';
const input = fs.readFileSync("input").toString().split(",").map(num => parseInt(num));

async function main() {
  let memory = input.slice();

  // Restore integrity
  memory[1] = 12
  memory[2] = 2
  //

  let index = 0;

  let [a,b,x] = [0,0,0];

  while(index < memory.length && memory[index] !== 99) {
    switch(memory[index]){
      case 1:
        // .add
        [a, b, x] = memory.slice(index+1, index+4);
        
        memory[x] = memory[a] + memory[b];

        index += 4;
        break;
      case 2:
        // .mul
        [a, b, x] = memory.slice(index+1, index+4);
        
        memory[x] = memory[a] * memory[b];

        index += 4;
        break;
      case 99:
        // .end
        return memory[0];
      default:
        // .hcf
        throw new Error("Halt and catch fire");
    }
  }

  console.warn("Reached end of code without halting instruction!")
  return memory[0]
}

main().then(console.log);
