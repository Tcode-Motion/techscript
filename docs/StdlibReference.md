# TechScript 2.0 Standard Library Reference

TechScript 2.0 provides 34 default namespaces/modules for real-world application building.

## 1. std.io
- `print(args...)`: prints arguments to stdout.
- `println(args...)`: prints arguments with trailing newline.
- `readline()`: reads one line from stdin.

## 2. std.fs (Capability: FileSystem)
- `read_file(path)`: returns file contents as a string.
- `write_file(path, content)`: writes content to file.
- `exists(path)`: checks if a file/directory path exists.

## 3. std.json
- `parse(json_str)`: parses JSON string into map or list.
- `stringify(val)`: serializes value to JSON string.

## 4. std.http (Capability: Network)
- `get(url)`: performs HTTP GET request.
- `post(url, body)`: performs HTTP POST request.
- `listen(port, handler)`: binds HTTP server on port.
