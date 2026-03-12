const fs = require('fs');
const path = require('path');

const mainReadmePath = path.join(__dirname, '..', '..', 'README.md');
const webReadmePath = path.join(__dirname, '..', 'src', 'data', 'readmeData.ts');

if (fs.existsSync(mainReadmePath)) {
  const content = fs.readFileSync(mainReadmePath, 'utf8');
  // Escape backticks and dollar signs to use within template literals
  const escapedContent = content.replace(/`/g, '\\`').replace(/\$/g, '\\$');
  const fileContent = `export const readmeContent = \`${escapedContent}\`;\n`;
  
  if (!fs.existsSync(path.dirname(webReadmePath))) {
    fs.mkdirSync(path.dirname(webReadmePath), { recursive: true });
  }
  
  fs.writeFileSync(webReadmePath, fileContent);
  console.log(`Successfully extracted ${content.length} characters from main project README.`);
} else {
  console.log("Main project README not found for extraction.");
}
