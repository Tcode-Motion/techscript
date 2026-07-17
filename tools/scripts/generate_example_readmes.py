# tools/generate_example_readmes.py
import os

EXAMPLES_DIR = "public-release/examples"

DESCRIPTIONS = {
    "hello_world": "Prints a friendly greeting to the terminal.",
    "variables": "Demonstrates declaring mutable variables and constants.",
    "functions": "Demonstrates function declarations, arguments, and return statements.",
    "structs": "Shows how to define custom structures and access their fields.",
    "enums": "Shows how to declare enumerations and match variants.",
    "modules": "Demonstrates modular imports and exports across multiple files.",
    "generics": "Demonstrates generic type parameters on structs and functions.",
    "pattern_matching": "Shows the usage of the when pattern matching expression.",
    "errors": "Demonstrates error recovery and throwing exceptions using attempt/catch.",
    "io": "Demonstrates reading from and writing to the standard console.",
    "fs": "Demonstrates fully sandboxed filesystem operations.",
    "net": "Demonstrates creating TCP socket listeners and client connections.",
    "http": "Demonstrates HTTP server binding and sending client requests.",
    "json": "Demonstrates JSON parsing and serialization.",
    "yaml": "Demonstrates YAML parsing and serialization.",
    "xml": "Demonstrates XML parsing and serialization.",
    "csv": "Demonstrates CSV parsing and serialization.",
    "crypto": "Demonstrates AES encryption and decryption.",
    "hash": "Demonstrates hashing algorithms including MD5 and SHA256.",
    "datetime": "Shows current epoch time querying and formatting.",
    "env": "Shows environment variables retrieval and configuration.",
    "process": "Demonstrates spawning external subprocesses and getting their outputs.",
    "random": "Demonstrates random number generation and choices from collections.",
    "regex": "Demonstrates checking text matching and replacements using regular expressions.",
    "path": "Shows filesystem path concatenation and extension extraction.",
    "thread": "Demonstrates spawning and joining OS-level native threads.",
    "sync": "Demonstrates thread synchronization using Mutex locks.",
    "async": "Demonstrates spawning cooperative async tasks.",
    "channel": "Demonstrates thread-safe message passing channels.",
    "testing": "Demonstrates writing assertions and unit tests.",
    "logging": "Demonstrates styled console logging outputs.",
    "compress": "Demonstrates archive zip/unzip actions.",
    "closures": "Demonstrates lexical closures and first-class functions.",
    "collections": "Demonstrates list and map manipulation helpers.",
    "complete_project": "A complete scaffolding example for a multi-crate project.",
    "filesystem": "Alternative filesystem sandboxing examples.",
    "loops": "Shows control structures including while and repeat loops.",
    "models": "Shows structural model declarations and fields.",
    "packages": "Shows multi-package imports and constraints.",
    "recursion": "Demonstrates recursive function evaluation."
}

EXPECTED_OUTPUTS = {
    "hello_world": "Hello, World!",
    "variables": "Mutable and constant variables initialized successfully.",
    "functions": "Function executed and returned expected results.",
    "structs": "Struct Point initialized with fields x: 10, y: 20",
    "enums": "Enum matched successfully.",
    "modules": "Module double function returned 20.",
    "generics": "Generic types resolved correctly.",
    "pattern_matching": "Match expression completed successfully.",
    "errors": "Exception caught and handled.",
    "io": "Console I/O completed.",
    "fs": "Files written and verified successfully.",
    "net": "TCP connection completed.",
    "http": "HTTP request succeeded with status 200.",
    "json": "JSON parsed and stringified successfully.",
    "yaml": "YAML parsed successfully.",
    "xml": "XML parsed successfully.",
    "csv": "CSV parsed successfully.",
    "crypto": "Text encrypted and decrypted successfully.",
    "hash": "SHA256 hash computed successfully.",
    "datetime": "Current epoch retrieved successfully.",
    "env": "Environment variable matched successfully.",
    "process": "Subprocess executed successfully.",
    "random": "Random value generated successfully.",
    "regex": "Regular expression match succeeded.",
    "path": "Paths joined successfully.",
    "thread": "Threads spawned and joined successfully.",
    "sync": "Mutex lock held and released successfully.",
    "async": "Async task resolved successfully.",
    "channel": "Message sent and received over channel.",
    "testing": "Assertions verified successfully.",
    "logging": "Log outputs verified successfully.",
    "compress": "Zip file created successfully."
}

def generate_readmes():
    if not os.path.exists(EXAMPLES_DIR):
        print(f"Directory {EXAMPLES_DIR} not found.")
        return

    for entry in os.listdir(EXAMPLES_DIR):
        path = os.path.join(EXAMPLES_DIR, entry)
        if os.path.isdir(path):
            main_path = os.path.join(path, "main.txs")
            code_content = ""
            if os.path.exists(main_path):
                with open(main_path, "r", encoding="utf-8") as f:
                    code_content = f.read()

            desc = DESCRIPTIONS.get(entry, f"Demonstrates the use of the {entry} module/features.")
            expected = EXPECTED_OUTPUTS.get(entry, "Completed successfully.")

            readme_content = f"""# Example: {entry.replace('_', ' ').title()}

## Description
{desc}

## Source Code (`main.txs`)
```techscript
{code_content.strip()}
```

## Running the Example
To run this example using the TechScript compiler driver:
```bash
tsc run main.txs
```

## Expected Output
```
{expected}
```
"""
            readme_path = os.path.join(path, "README.md")
            with open(readme_path, "w", encoding="utf-8") as f:
                f.write(readme_content)
            print(f"Generated README.md for {entry}")

if __name__ == "__main__":
    generate_readmes()
