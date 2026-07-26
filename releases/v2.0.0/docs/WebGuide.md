# TechScript 2.0 Web Guide

TechScript provides native capabilities for building web clients and servers through the standard library `web` and `http` modules.

## HTTP Client API

To make HTTP requests, import the `http` module and use the qualified functions:

```txs
use http

# GET request
response = http.get("https://api.github.com/repos/Tcode-Motion/TechScript-2.0")
say response

# POST request
payload = {"title": "New Issue", "body": "Reported from TechScript 2.0"}
resp = http.post("https://api.github.com/repos/Tcode-Motion/TechScript-2.0/issues", json.stringify(payload))
say resp
```

## HTTP Server API

To build web servers, use the `web` module:

```txs
use web

# Create a simple server on port 8080
server = web.listen(8080)
say "Web server running on port 8080..."

# Handle incoming requests
repeat true
    request = server.accept()
    when request.path == "/"
        request.respond(200, "text/html", "<h1>Welcome to TechScript 2.0 Web Server!</h1>")
    else
        request.respond(404, "text/plain", "Not Found")
    end
end
```

## Compilation and Execution

Web capabilities require standard network permissions in the configuration:

```toml
# tech.toml
[package]
name = "web_app"
version = "1.0.0"
capabilities = ["Network"]
```
