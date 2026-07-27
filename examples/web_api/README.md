# Web API Example

This example demonstrates how to perform HTTP GET requests to an external API (like GitHub's API) and parse the JSON response body using TechScript's standard modules.

## Code (`web_api.txs`)
```txs
use http
use json

url = "https://api.github.com/repos/Tcode-Motion/techscript"

say $"Sending GET request to {url}..."

# Send request
response = http.get(url, {
    "User-Agent": "TechScript-Compiler"
})

say $"Status Code: {response['status']}"

# Parse response body
data = json.decode(response["body"])
say $"Repository: {data['name']}"
say $"Owner: {data['owner']['login']}"
say $"Open Issues: {data['open_issues_count']}"
```

## Running the Example
```bash
tech run web_api.txs
```

## Expected Output
```
Sending GET request to https://api.github.com/repos/Tcode-Motion/techscript...
Status Code: 200
Repository: techscript
Owner: Tcode-Motion
Open Issues: 0
```
