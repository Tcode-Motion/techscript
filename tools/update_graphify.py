# tools/update_graphify.py
import os
import sys
import subprocess
from pathlib import Path

WORKSPACE_DIR = Path(__file__).resolve().parent.parent
DOTENV_PATH = WORKSPACE_DIR / ".env"
GRAPHIFY_IGNORE_PATH = WORKSPACE_DIR / ".graphifyignore"
OUTPUT_DIR = WORKSPACE_DIR / "graphify-out"

def check_graphify_installed():
    print("[Graphify] Verifying Graphify installation...")
    # Check if CLI is available directly
    try:
        res = subprocess.run(["graphify", "--version"], capture_output=True, text=True)
        if res.returncode == 0:
            print(f"[Graphify] Found global Graphify installation: {res.stdout.strip()}")
            return True, ["graphify"]
    except Exception:
        pass

    # Check python module path
    python_path_file = OUTPUT_DIR / ".graphify_python"
    python_exe = "python"
    if python_path_file.exists():
        with open(python_path_file, "r", encoding="utf-8") as f:
            python_exe = f.read().strip()

    try:
        res = subprocess.run([python_exe, "-m", "graphify", "--version"], capture_output=True, text=True)
        if res.returncode == 0:
            print(f"[Graphify] Found Python-module Graphify installation: {res.stdout.strip()}")
            return True, [python_exe, "-m", "graphify"]
    except Exception:
        pass

    print("[Graphify] Error: Graphify is not installed. Please run: pip install graphifyy")
    return False, []

def check_api_keys():
    keys = ["GEMINI_API_KEY", "GOOGLE_API_KEY", "OPENAI_API_KEY", "ANTHROPIC_API_KEY", "DEEPSEEK_API_KEY"]
    for key in keys:
        if os.environ.get(key):
            print(f"[Graphify] Found API key in environment: {key}")
            return True
            
    if DOTENV_PATH.exists():
        with open(DOTENV_PATH, "r", encoding="utf-8") as f:
            for line in f:
                if any(line.strip().startswith(f"{key}=") for key in keys):
                    print("[Graphify] Found API key in .env file")
                    return True
    return False

def run_command(cmd_args):
    print(f"[Graphify] Executing: {' '.join(cmd_args)}")
    result = subprocess.run(cmd_args, cwd=WORKSPACE_DIR, capture_output=True, text=True)
    if result.returncode != 0:
        print("[Graphify] Error output:")
        print(result.stderr)
        return False
    print(result.stdout)
    return True

def main():
    os.makedirs(OUTPUT_DIR, exist_ok=True)
    installed, base_cmd = check_graphify_installed()
    if not installed:
        sys.exit(1)

    has_keys = check_api_keys()
    
    # 1. Refresh Graphify index
    success = False
    if has_keys:
        # Full extraction (includes Markdown and documentation)
        cmd = base_cmd + ["extract", ".", "--cargo"]
        success = run_command(cmd)
    else:
        # Code-only extraction fallback by temporarily ignoring Markdown and PDF files
        print("[Graphify] Warning: No LLM API key detected. Running in code-only/AST extraction mode.")
        print("[Graphify] To run a full semantic extraction (including docs), set GEMINI_API_KEY in your environment or .env file.")
        
        # Read original ignore file
        original_ignore = ""
        if GRAPHIFY_IGNORE_PATH.exists():
            with open(GRAPHIFY_IGNORE_PATH, "r", encoding="utf-8") as f:
                original_ignore = f.read()
                
        # Temporarily append docs, .agents, .github, pdf and md to ignore files to prevent semantic extraction errors
        temp_ignore = original_ignore + "\ndocs/\n.agents/\n.github/\nExecutive Summary.pdf\n*.md\n*.pdf\n**/*.md\n**/*.pdf\n**/*.docx\n**/*.txt\n**/*.png\n**/*.jpg\n"
        with open(GRAPHIFY_IGNORE_PATH, "w", encoding="utf-8") as f:
            f.write(temp_ignore)
            
        try:
            cmd = base_cmd + ["extract", ".", "--cargo", "--no-cluster"]
            success = run_command(cmd)
        finally:
            # Restore original ignore file
            with open(GRAPHIFY_IGNORE_PATH, "w", encoding="utf-8") as f:
                f.write(original_ignore)
                
    if not success:
        print("[Graphify] Error: Index refresh/extraction failed.")
        sys.exit(1)

    # 2. Run clustering and generate report / HTML
    print("[Graphify] Running clustering and generating reports...")
    cmd = base_cmd + ["cluster-only", "."]
    if not run_command(cmd):
        print("[Graphify] Error: Clustering failed.")
        sys.exit(1)

    # 3. Validate outputs
    required_artifacts = {
        "graph.json": OUTPUT_DIR / "graph.json",
        "graph.html": OUTPUT_DIR / "graph.html",
        "GRAPH_REPORT.md": OUTPUT_DIR / "GRAPH_REPORT.md"
    }

    missing_artifacts = []
    for name, path in required_artifacts.items():
        if not (path.exists() and path.stat().st_size > 0):
            missing_artifacts.append(name)

    if missing_artifacts:
        print(f"[Graphify] Error: The following required artifacts are missing or empty: {', '.join(missing_artifacts)}")
        sys.exit(1)

    print("\n[Graphify] Success! All required knowledge graph artifacts generated successfully:")
    print(f"  - graph.json:      {required_artifacts['graph.json']}")
    print(f"  - graph.html:      {required_artifacts['graph.html']}")
    print(f"  - GRAPH_REPORT.md: {required_artifacts['GRAPH_REPORT.md']}")
    sys.exit(0)

if __name__ == "__main__":
    main()
