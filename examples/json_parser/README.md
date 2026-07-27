# JSON Parser Example

This example demonstrates how to encode TechScript maps into JSON strings and decode JSON strings back into maps.

## Code (`parser.txs`)
```txs
use json

raw_json = "{\"name\": \"TechScript\", \"version\": 2, \"active\": true}"

# Decode JSON string to map
data = json.decode(raw_json)

say $"Decoded Name: {data['name']}"
say $"Decoded Version: {data['version']}"
say $"Decoded Active: {data['active']}"

# Modify and re-encode to JSON
data["status"] = "stable"
encoded = json.encode(data)
say $"Encoded JSON: {encoded}"
```

## Running the Example
```bash
tech run parser.txs
```

## Expected Output
```
Decoded Name: TechScript
Decoded Version: 2
Decoded Active: true
Encoded JSON: {"active":true,"name":"TechScript","status":"stable","version":2}
```
*(Note: JSON key serialization order may depend on dictionary sorting)*
