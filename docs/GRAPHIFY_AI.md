# Graphify AI Usage Guide

> **Target Audience**: AI Assistants / Compiler Agents
> **Purpose**: Complete guide to Graphify-Labs knowledge graph integration in TechScript.
> **Parent Link**: [AI_BOOTSTRAP](../AI_BOOTSTRAP.md)
> **Child Links**: [00_PROJECT](../docs/ai/00_PROJECT.md) · [02_MEMORY](../docs/ai/02_MEMORY.md)

---

## 1. Installation

To install the official `graphifyy` tool globally using `uv`:
```bash
uv tool install graphifyy
```
To register it as a project-scoped Google Antigravity skill:
```bash
graphify antigravity install
```
This writes the rules to `.agents/rules/graphify.md` and workflows to `.agents/workflows/graphify.md`.

---

## 2. Configuration

Exclusions are managed in the `.graphifyignore` configuration file at the root of the repository. By default, it ignores:
- `target/` (Rust build artifacts)
- `.git/` (VCS metadata)
- `node_modules/` (Node dependencies)
- `graphify-out/` (Graphify output directory itself)
- Temporary files (`*.tmp`, `*.log`)

---

## 3. Update Command

To refresh the index and regenerate all artifacts, run the official wrapper script:
```bash
python tools/update_graphify.py
```
This script:
1. Verifies the `graphify` installation.
2. Refreshes the Graphify index.
3. Generates `graph.html`.
4. Generates `graph.json`.
5. Generates `GRAPH_REPORT.md`.
6. Validates outputs and exits with `1` if any file is missing or empty.

---

## 4. Generated Files

All generated outputs reside in the `graphify-out/` directory:

- **`graphify-out/graph.json`**: Machine-readable JSON containing nodes and edges representing code ASTs and Cargo manifests.
- **`graphify-out/graph.html`**: Interactive D3-based force-directed visualization.
- **`graphify-out/GRAPH_REPORT.md`**: Textual summary of communities, core abstractions, and god nodes.

---

## 5. How to Use Graphify Outputs

### 5.1 How to Use `graph.html`
- **Interactive Force Graph**: Nodes represent files, crates, structs, or functions. Edges represent contains, depends_on, or calls.
- **Node Colors**: Colors represent Louvain-detected communities (loosely coupled components).
- **Search & Filters**: Use the search box to find specific symbols. Toggle checkboxes to filter relationship types.
- **Zoom & Pan**: Scroll to zoom, drag to pan the viewport.
- **Click-to-expand**: Click any node to open the inspector pane showing in-degree/out-degree connections, source lines, and community details.

### 5.2 How to Use `graph.json`
- Parsed directly by LLM agents. Contains:
  - `nodes`: List of IDs, types, names, file paths, and Louvain community numbers.
  - `edges`: Source/target pairs with confidence values (`EXTRACTED` for code, `INFERRED` for doc linkages).
  - `metadata`: Token costs and timestamps.

### 5.3 How to Use `GRAPH_REPORT.md`
- Core human-readable overview. Look here for:
  - **God Nodes**: Most connected abstractions (e.g., `main`, `CheckedProgram`).
  - **Surprising Connections**: Bridging nodes linking different communities.
  - **Suggested Questions**: Contextual follow-up exploration routes.

---

## 6. How to Run CLI Queries

### 6.1 `graphify explain`
Get a detailed summary of a specific concept node:
```bash
graphify explain "main"
```
*Example Output:*
```
Node: main()
  ID:        tools_update_graphify_main
  Source:    tools/update_graphify.py L38
  Type:      code
  Community: Community 0
```

### 6.2 `graphify path`
Determine the shortest path/dependency chain between two concepts:
```bash
graphify path "main" "check_api_keys"
```
*Example Output:*
```
Shortest path (1 hops):
  main() --calls [EXTRACTED]--> check_api_keys()
```

### 6.3 `graphify query`
Perform a BFS traversal to answer natural-language questions about the codebase:
```bash
graphify query "What does check_api_keys do?"
```
*Example Output:*
```
NODE check_api_keys() [src=tools/update_graphify.py loc=L11 community=Community 0]
EDGE check_api_keys() --calls [EXTRACTED context=call]--> main()
```

---

## 7. Troubleshooting & Limitations

- **Error: No LLM API Key**: If no `GEMINI_API_KEY` is present, scanning doc files will fail.
  - *Fix*: The update script automatically ignores `.md`/`.pdf` files during AST scanning to run in a code-only fallback mode. To run a full extraction, set the key.
- **Empty Graph**: If the graph is empty, verify that your files are not matches for patterns in `.graphifyignore`.
