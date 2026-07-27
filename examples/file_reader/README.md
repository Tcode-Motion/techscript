# File Reader Example

This example shows how to write content to a file, read it back, print the contents, and delete the file using the `fs` standard library module.

## Code (`reader.txs`)
```txs
use fs

filename = "test_file.txt"
content = "Line 1: TechScript is cool!\nLine 2: Zero overhead memory safety."

say $"Writing to {filename}..."
fs.write(filename, content)

say $"Reading from {filename}..."
read_content = fs.read(filename)

say "File Content:"
say read_content

# Clean up
fs.delete(filename)
```

## Running the Example
```bash
tech run reader.txs
```

## Expected Output
```
Writing to test_file.txt...
Reading from test_file.txt...
File Content:
Line 1: TechScript is cool!
Line 2: Zero overhead memory safety.
```
