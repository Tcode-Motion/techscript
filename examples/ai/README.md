# AI Module Example

This example demonstrates how to import and use the standard `ai` module to execute generative text completions using the Gemini API.

## Code (`prompt.txs`)
```txs
use ai

say "Loading Gemini AI model..."
model = ai.load("gemini-2.5")

say "Sending prompt: Explain stack-based VMs..."
response = ai.prompt(model, "Explain stack-based VMs")

say "AI Response Received:"
say response
```

## Running the Example
```bash
tech run prompt.txs
```

## Expected Output
```
Loading Gemini AI model...
Sending prompt: Explain stack-based VMs...
AI Response Received:
[Gemini 2.5] A stack-based virtual machine evaluates expressions using an execution stack. Operators pop operands, calculate, and push results back.
```
*(Note: Output response content is simulated locally for verification)*
