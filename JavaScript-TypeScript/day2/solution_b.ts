import * as fs from 'fs';
const input = fs.readFileSync("input").toString().split(",").map(num => parseInt(num));

async function run(noun: number, verb: number) {
  let memory = input.slice();

  // Restore integrity
  memory[1] = noun
  memory[2] = verb
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

  return memory[0]
}

async function main(){
  let noun = 0;
  let verb = 0;
  while(noun < 99) {
    while(verb < 99) {
      console.log(`Testing [${noun}, ${verb}]`)
      if(await run(noun,verb) === 19690720) {
        console.log("Success!")
        return 100*noun+verb
      }
      verb++;
    }
    noun++;
    verb = 0;
  }
  throw new Error("Found no solution!")
}

main().then(console.log);
