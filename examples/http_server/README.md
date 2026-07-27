# HTTP Server Example

This example demonstrates how to set up an HTTP server, register a GET route, and simulate a request cycle in TechScript.

## Code (`server.txs`)
```txs
use http

server = http.new_server()

# Register routes
http.route(server, "/hello", do(req)
    send http.response(200, "text/plain", "Hello from TechScript Server!")
end)

say "HTTP Server configured on http://localhost:8080"
say "Running route mock test..."

# Simulate a mock request locally
mock_request = http.mock_request("GET", "/hello")
response = http.handle_request(server, mock_request)

say $"Response Code: {response['status']}"
say $"Response Body: {response['body']}"
```

## Running the Example
```bash
tech run server.txs
```

## Expected Output
```
HTTP Server configured on http://localhost:8080
Running route mock test...
Response Code: 200
Response Body: Hello from TechScript Server!
```
