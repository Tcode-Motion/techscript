# tools/update_graphify.py
import os
import sys
import subprocess
import time
import re
from pathlib import Path

# Add tools directory to sys.path to import check_graphify
tools_dir = Path(__file__).resolve().parent
if str(tools_dir) not in sys.path:
    sys.path.insert(0, str(tools_dir))

# Standardized Exit Codes
EXIT_SUCCESS = 0
EXIT_VALIDATION_FAILED = 1
EXIT_MISSING_DEPENDENCY = 2
EXIT_EXECUTION_FAILED = 3
EXIT_INVALID_OUTPUT = 4
EXIT_CONFIG_ERROR = 5

def check_python_version():
    py_version = f"{sys.version_info.major}.{sys.version_info.minor}.{sys.version_info.micro}"
    if sys.version_info < (3, 10):
        print(f"[ERROR] Python {py_version} is unsupported (requires >= 3.10)")
        sys.exit(EXIT_VALIDATION_FAILED)
    return py_version

# Verify Python dependencies before importing them (self-healing / friendly messages)
def check_python_dependencies():
    missing = []
    try:
        import dotenv
    except ImportError:
        missing.append("python-dotenv")
    try:
        import requests
    except ImportError:
        missing.append("requests")
    if sys.version_info < (3, 11):
        try:
            import tomli
        except ImportError:
            missing.append("tomli")
            
    if missing:
        print(f"[ERROR] Missing Python package(s): {', '.join(missing)}")
        print("Fix:")
        print("  pip install -r requirements.txt")
        sys.exit(EXIT_MISSING_DEPENDENCY)

# Verify dependencies and import
check_python_version()
check_python_dependencies()

# Now we can safely import dotenv
import dotenv

def find_repo_root():
    current = Path(__file__).resolve().parent
    for _ in range(10):
        if (current / ".git").exists() or (current / "Cargo.toml").exists() or (current / ".graphifyignore").exists():
            return current
        parent = current.parent
        if parent == current:
            break
        current = parent
    return Path(__file__).resolve().parent.parent

# Set up Workspace Paths
WORKSPACE_DIR = find_repo_root()
DOTENV_PATH = WORKSPACE_DIR / ".env"
GRAPHIFY_IGNORE_PATH = WORKSPACE_DIR / ".graphifyignore"
OUTPUT_DIR = WORKSPACE_DIR / "graphify-out"
LOG_DIR = WORKSPACE_DIR / "logs"
LOG_FILE = LOG_DIR / "graphify.log"

def log_message(msg, level="INFO"):
    formatted = f"[{time.strftime('%Y-%m-%d %H:%M:%S')}] [{level}] {msg}"
    print(formatted)
    try:
        os.makedirs(LOG_DIR, exist_ok=True)
        with open(LOG_FILE, "a", encoding="utf-8") as f:
            f.write(formatted + "\n")
    except Exception as e:
        print(f"[WARNING] Could not write to log file: {e}")

def check_graphify_installed():
    # Check if CLI is available directly
    try:
        res = subprocess.run(["graphify", "--version"], capture_output=True, text=True, check=False)
        if res.returncode == 0:
            version_str = res.stdout.strip()
            match = re.search(r"(\d+\.\d+\.\d+)", version_str)
            if match:
                return True, ["graphify"], match.group(1)
            return True, ["graphify"], version_str
    except Exception:
        pass

    # Check python module path
    python_exe = sys.executable
    try:
        res = subprocess.run([python_exe, "-m", "graphify", "--version"], capture_output=True, text=True, check=False)
        if res.returncode == 0:
            version_str = res.stdout.strip()
            match = re.search(r"(\d+\.\d+\.\d+)", version_str)
            if match:
                return True, [python_exe, "-m", "graphify"], match.group(1)
            return True, [python_exe, "-m", "graphify"], version_str
    except Exception:
        pass

    log_message("Graphify CLI is not installed.", "ERROR")
    print("Fix:")
    print("  pip install -r requirements.txt")
    sys.exit(EXIT_MISSING_DEPENDENCY)

def check_rust_toolchain():
    rustc_version = None
    cargo_version = None
    try:
        res = subprocess.run(["rustc", "--version"], capture_output=True, text=True, check=False)
        if res.returncode == 0:
            rustc_version = res.stdout.strip()
    except Exception:
        pass
    try:
        res = subprocess.run(["cargo", "--version"], capture_output=True, text=True, check=False)
        if res.returncode == 0:
            cargo_version = res.stdout.strip()
    except Exception:
        pass
    return rustc_version, cargo_version

def check_api_keys():
    keys = ["GEMINI_API_KEY", "GOOGLE_API_KEY", "OPENAI_API_KEY", "ANTHROPIC_API_KEY", "DEEPSEEK_API_KEY"]
    for key in keys:
        if os.environ.get(key):
            return key
            
    if DOTENV_PATH.exists():
        try:
            # Load environment variables from .env
            dotenv.load_dotenv(DOTENV_PATH)
            for key in keys:
                if os.environ.get(key):
                    return key
        except Exception as e:
            log_message(f"Failed to read .env file: {e}", "WARNING")
    return None

def run_command(cmd_args, dry_run=False):
    cmd_str = " ".join(cmd_args)
    log_message(f"Executing: {cmd_str}")
    if dry_run:
        log_message(f"[DRY-RUN] Would run command: {cmd_str}")
        return True
        
    start_time = time.time()
    result = subprocess.run(cmd_args, cwd=WORKSPACE_DIR, capture_output=True, text=True)
    duration = time.time() - start_time
    
    if result.returncode != 0:
        log_message(f"Command failed in {duration:.2f}s with exit code {result.returncode}", "ERROR")
        if result.stderr.strip():
            log_message(f"Error output:\n{result.stderr.strip()}", "ERROR")
        return False
        
    log_message(f"Command completed in {duration:.2f}s")
    return True

def main():
    start_time = time.time()
    
    # Parse CLI flags
    dry_run = "--dry-run" in sys.argv
    
    log_message("=" * 60)
    log_message(f"Starting Graphify Knowledge Graph Update (dry-run: {dry_run})")
    
    # 1. Verify Repository Root and Config
    if not GRAPHIFY_IGNORE_PATH.exists():
        log_message(f"Configuration error: .graphifyignore is missing at {GRAPHIFY_IGNORE_PATH}", "ERROR")
        sys.exit(EXIT_CONFIG_ERROR)
        
    log_message(f"Workspace root detected: {WORKSPACE_DIR}")
    
    # 2. Check Python and Toolchain
    py_ver = check_python_version()
    log_message(f"Python Version: {py_ver}")
    
    installed, base_cmd, graphify_ver = check_graphify_installed()
    log_message(f"Graphify Version: {graphify_ver} (command: {' '.join(base_cmd)})")
    
    rustc_ver, cargo_ver = check_rust_toolchain()
    if rustc_ver:
        log_message(f"Rust Version: {rustc_ver}")
    else:
        log_message("rustc is not installed.", "WARNING")
        
    if cargo_ver:
        log_message(f"Cargo Version: {cargo_ver}")
    else:
        log_message("cargo is not installed.", "WARNING")
        
    # Determine if cargo can be used
    use_cargo = cargo_ver is not None
    if not use_cargo:
        log_message("Proceeding without --cargo flag due to missing Cargo installation.", "WARNING")
        
    # 3. Check LLM Keys
    active_key = check_api_keys()
    if active_key:
        log_message(f"Found active API key: {active_key}")
    else:
        log_message("No LLM API key detected. Running in AST-only mode.", "INFO")
        
    # Create output directory
    if not dry_run:
        os.makedirs(OUTPUT_DIR, exist_ok=True)
        
    # 4. Graphify Extraction & Clustering
    success = False
    if active_key:
        # Full extraction (includes Markdown and documentation)
        cmd = base_cmd + ["extract", "."]
        if use_cargo:
            cmd.append("--cargo")
        success = run_command(cmd, dry_run=dry_run)
    else:
        # Code-only extraction fallback by temporarily ignoring Markdown and PDF files
        log_message("Temporarily modifying .graphifyignore for AST-only fallback...")
        
        original_ignore = ""
        try:
            with open(GRAPHIFY_IGNORE_PATH, "r", encoding="utf-8-sig") as f:
                original_ignore = f.read()
        except Exception as e:
            log_message(f"Failed to read .graphifyignore: {e}", "ERROR")
            sys.exit(EXIT_CONFIG_ERROR)
            
        temp_ignore = (
            original_ignore +
            "\ndocs/\n.agents/\n.github/\nresearch/\nassets/\nreleases/\nlogs/\nscripts/\n"
            "Executive Summary.pdf\n*.md\n**/*.md\n*.pdf\n**/*.pdf\n"
            "*.docx\n**/*.docx\n*.txt\n**/*.txt\n*.png\n**/*.png\n"
            "*.jpg\n**/*.jpg\n*.jpeg\n**/*.jpeg\n*.gif\n**/*.gif\n"
            "*.svg\n**/*.svg\n*.ico\n**/*.ico\n*.sh\n**/*.sh\n"
            "requirements.txt\n*.bat\n**/*.bat\n*.toml\n**/*.toml\n"
            "*.lock\n**/*.lock\n"
            "LICENSE\n**/LICENSE\n"
            "NOTICE\n**/NOTICE\n"
            "COPYRIGHT\n**/COPYRIGHT\n"
            ".graphifyignore\n"
        )
        
        ignore_modified = False
        try:
            if not dry_run:
                with open(GRAPHIFY_IGNORE_PATH, "w", encoding="utf-8") as f:
                    f.write(temp_ignore)
                ignore_modified = True
                log_message(f"Wrote temporary ignore file to {GRAPHIFY_IGNORE_PATH}")
                
            cmd = base_cmd + ["extract", ".", "--no-cluster"]
            if use_cargo:
                cmd.append("--cargo")
            success = run_command(cmd, dry_run=dry_run)
        finally:
            if ignore_modified:
                try:
                    with open(GRAPHIFY_IGNORE_PATH, "w", encoding="utf-8") as f:
                        f.write(original_ignore)
                    log_message("Restored original .graphifyignore")
                except Exception as e:
                    log_message(f"Failed to restore .graphifyignore: {e}", "ERROR")
                    sys.exit(EXIT_CONFIG_ERROR)
                    
    if not success:
        log_message("Graphify extraction failed.", "ERROR")
        sys.exit(EXIT_EXECUTION_FAILED)
        
    # 5. Run clustering and generate report / HTML
    log_message("Running clustering and generating reports...")
    cmd = base_cmd + ["cluster-only", "."]
    if not active_key:
        cmd.append("--no-label")
    if not run_command(cmd, dry_run=dry_run):
        log_message("Clustering failed.", "ERROR")
        sys.exit(EXIT_EXECUTION_FAILED)
        
    # 6. Validate generated outputs (only if not dry-run)
    if not dry_run:
        try:
            from check_graphify import validate_graphify_outputs
            valid, results = validate_graphify_outputs(OUTPUT_DIR)
            if not valid:
                log_message("Output validation failed.", "ERROR")
                if "error" in results:
                    log_message(results["error"], "ERROR")
                else:
                    for f, r in results.items():
                        if r["status"] == "FAIL":
                            log_message(f"{f}: {r['reason']}", "ERROR")
                sys.exit(EXIT_INVALID_OUTPUT)
                
            log_message("Output validation succeeded:")
            for f, r in results.items():
                log_message(f"  - {f}: {r.get('info', 'Verified')}")
        except ImportError:
            log_message("Warning: check_graphify.py validation module could not be imported.", "WARNING")
            
    execution_time = time.time() - start_time
    log_message(f"Graphify update completed successfully in {execution_time:.2f}s.")
    log_message("=" * 60 + "\n")
    sys.exit(EXIT_SUCCESS)

if __name__ == "__main__":
    main()
