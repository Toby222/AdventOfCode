import fs from "fs";

/**
 * 
 * @param {string} infix 
 */
function infixToPostfix(infix) {
  infix = `(${infix})`.replace(/\s+/g, '');
  const l = infix.length;
  /** @type {string[]} */
  const char_stack = [];
  /** @type {string[]} */
  const output = []

  function getPriority(c) {
    return ['-','+','*','/'].includes(c) ? 1 : 0
  }

  for(let i = 0; i < l; i++) {
    if(/[a-z0-9]/i.test(infix[i])) {
      output.push(infix[i])
    } else if (infix[i] === "(") {
      char_stack.push('(')
    } else if (infix[i] === ")") {
      while(char_stack[char_stack.length-1] !== "(") {
        output.push(char_stack.pop())
      }

      char_stack.pop();
    } else if (!/[a-z0-9]/i.test(char_stack[char_stack.length-1])) {
      while(getPriority(infix[i]) <= getPriority(char_stack[char_stack.length-1])) {
        output.push(char_stack.pop())
      }
      char_stack.push(infix[i])
    }
  }
  return output.join(" ")
}

function solvePostfix(postfix) {
  const resultStack = [];
  postfix = postfix.split(" ");
  for(let i = 0; i < postfix.length; i++) {
      if(!isNaN(parseFloat(postfix[i])) && isFinite(parseInt(postfix[i]))) {
          resultStack.push(postfix[i]);
      } else {
          const a = resultStack.pop();
          const b = resultStack.pop();
          if(postfix[i] === "+") {
              resultStack.push(parseInt(a) + parseInt(b));
          } else if(postfix[i] === "-") {
              resultStack.push(parseInt(b) - parseInt(a));
          } else if(postfix[i] === "*") {
              resultStack.push(parseInt(a) * parseInt(b));
          } else if(postfix[i] === "/") {
              resultStack.push(parseInt(b) / parseInt(a));
          } else if(postfix[i] === "^") {
              resultStack.push(Math.pow(parseInt(b), parseInt(a)));
          }
      }
  }
  if(resultStack.length > 1) {
      return "error";
  } else {
      return resultStack.pop();
  }

}

/**
 * Main function for Part 1
 *
 * @param {string} data - Puzzle input as a single string.
 */
async function main(data) {
  const results = data
    .split(/\r?\n/)
    .map((line) => solvePostfix(infixToPostfix(line)));
  return results.reduce((prev, cur) => prev+cur);
}

fs.readFile("input", (err, data) => {
  if (err) throw err;
  console.time("Part 1");
  main(data.toString())
    .then((...args) => {
      console.timeEnd("Part 1");
      console.log(...args);
    })
    .catch(console.error);
});
