const fs = require('fs');
const content = fs.readFileSync('../FIX_GUIDE.md', 'utf-8');
const lines = content.split('\n');
let data = [
  { keyword: 'say', category: 'I/O', desc: 'Prints text to the console.', code: 'say "Hello, World!"' },
  { keyword: 'ask', category: 'I/O', desc: 'Prompts the user for input.', code: 'make name = ask "Name: "' },
  { keyword: 'make', category: 'Variables', desc: 'Declares a variable.', code: 'make age = 22' },
  { keyword: 'attempt', category: 'Control Flow', desc: 'Starts an error handling block.', code: 'attempt {\n  make x = 10 / 0\n}' },
  { keyword: 'catch', category: 'Control Flow', desc: 'Catches an error from an attempt block.', code: 'catch err {\n  say err.message\n}' },
  { keyword: 'when', category: 'Control Flow', desc: 'If statement.', code: 'when age >= 18 {\n  say "Adult!"\n}' },
  { keyword: 'or when', category: 'Control Flow', desc: 'Else-if statement.', code: '} or when age > 12 {\n  say "Teen"\n}' },
  { keyword: 'else', category: 'Control Flow', desc: 'Fallback block.', code: '} else {\n  say "Child"\n}' },
  { keyword: 'each', category: 'Loops', desc: 'Iterates over a range or list.', code: 'each i in 1..10 {\n  say i\n}' },
  { keyword: 'repeat', category: 'Loops', desc: 'While loop.', code: 'repeat x < 5 {\n  x = x + 1\n}' },
  { keyword: 'model', category: 'OOP', desc: 'Defines a class-like structure.', code: 'model User {\n  ...\n}' },
  { keyword: 'build', category: 'OOP', desc: 'Defines a method inside a model.', code: '  build init(self) {\n    self.x = 1\n  }' },
  { keyword: 'use', category: 'Modules', desc: 'Imports a built-in module.', code: 'use web\nuse math\nuse string' },
];

let currentCategory = '';
for (const line of lines) {
  const catMatch = line.match(/^### 5\.\d+\s+(.*?)\s+\(\d+ Functions\)/);
  if (catMatch) {
    currentCategory = catMatch[1].trim();
  } else {
    const fnMatch = line.match(/^\d+\.\s+`([^`]+)`:\s+(.*)/);
    if (fnMatch) {
      // Don't duplicate 'say'
      if (fnMatch[1].startsWith('say(')) continue;

      let codeStr = fnMatch[1];
      if (!codeStr.includes('(')) {
        codeStr += '()';
      }

      data.push({
        keyword: fnMatch[1],
        category: currentCategory || 'Built-in',
        desc: fnMatch[2],
        code: codeStr
      });
    }
  }
}

if (!fs.existsSync('src/data')) {
  fs.mkdirSync('src/data');
}
fs.writeFileSync('src/data/syntaxData.ts', 'export const syntaxData = ' + JSON.stringify(data, null, 2) + ';');
