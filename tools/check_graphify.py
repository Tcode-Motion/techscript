# tools/check_graphify.py
import os
import sys
import json
import re
import subprocess
from pathlib import Path

# Standardized Exit Codes
EXIT_SUCCESS = 0
EXIT_VALIDATION_FAILED = 1
EXIT_MISSING_DEPENDENCY = 2
EXIT_EXECUTION_FAILED = 3
EXIT_INVALID_OUTPUT = 4
EXIT_CONFIG_ERROR = 5

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
    return missing

def validate_graphify_outputs(output_dir):
    """
    Dynamically scans output_dir and validates generated files.
    Excludes files starting with '.' and manifest.json or cache folder.
    Returns (success, results)
    """
    if not output_dir.exists():
        return False, {"error": f"Output directory does not exist: {output_dir}"}
    
    # Get all files, excluding .*, manifest.json, cache/
    generated_files = []
    try:
        for p in output_dir.iterdir():
            if p.name.startswith("."):
                continue
            if p.name == "manifest.json":
                continue
            if p.is_dir() and p.name == "cache":
                continue
            if p.is_file():
                generated_files.append(p)
    except Exception as e:
        return False, {"error": f"Failed to list output directory: {e}"}
        
    if not generated_files:
        return False, {"error": f"No generated output files found in {output_dir}"}
        
    validation_results = {}
    overall_success = True
    
    for f in generated_files:
        filename = f.name
        size = f.stat().st_size
        if size == 0:
            validation_results[filename] = {"status": "FAIL", "reason": "File is empty (0 bytes)"}
            overall_success = False
            continue
            
        if f.suffix == ".json":
            try:
                with open(f, "r", encoding="utf-8") as file:
                    data = json.load(file)
                if not isinstance(data, dict):
                    validation_results[filename] = {"status": "FAIL", "reason": "Root element is not a JSON object"}
                    overall_success = False
                elif "nodes" not in data or ("edges" not in data and "links" not in data):
                    validation_results[filename] = {"status": "FAIL", "reason": "Missing 'nodes' and either 'edges' or 'links' keys"}
                    overall_success = False
                else:
                    edges_key = "links" if "links" in data else "edges"
                    validation_results[filename] = {
                        "status": "PASS",
                        "info": f"Valid JSON with {len(data.get('nodes', []))} nodes and {len(data.get(edges_key, []))} {edges_key} ({size / 1024:.2f} KB)"
                    }
            except json.JSONDecodeError as e:
                validation_results[filename] = {"status": "FAIL", "reason": f"Invalid JSON format: {e}"}
                overall_success = False
            except Exception as e:
                validation_results[filename] = {"status": "FAIL", "reason": f"Error reading file: {e}"}
                overall_success = False
                
        elif f.suffix == ".html":
            try:
                with open(f, "r", encoding="utf-8") as file:
                    content = file.read(500) # Read start
                if "<html>" not in content.lower() and "<!doctype html>" not in content.lower():
                    validation_results[filename] = {"status": "FAIL", "reason": "Not a valid HTML file"}
                    overall_success = False
                else:
                    validation_results[filename] = {
                        "status": "PASS",
                        "info": f"Valid HTML structure ({size / 1024:.2f} KB)"
                    }
            except Exception as e:
                validation_results[filename] = {"status": "FAIL", "reason": f"Error reading file: {e}"}
                overall_success = False
                
        elif f.suffix == ".md":
            try:
                with open(f, "r", encoding="utf-8") as file:
                    content = file.read(500)
                # Check for markdown headers or content
                if not any(line.strip().startswith("#") for line in content.splitlines()):
                    validation_results[filename] = {"status": "FAIL", "reason": "Markdown does not contain headings"}
                    overall_success = False
                else:
                    validation_results[filename] = {
                        "status": "PASS",
                        "info": f"Valid Markdown with headings ({size / 1024:.2f} KB)"
                    }
            except Exception as e:
                validation_results[filename] = {"status": "FAIL", "reason": f"Error reading file: {e}"}
                overall_success = False
        else:
            # Other file types, check size only
            validation_results[filename] = {
                "status": "PASS",
                "info": f"Generic file checked ({size / 1024:.2f} KB)"
            }
            
    return overall_success, validation_results

def main():
    print("==================================================")
    print("         Graphify Verification Utility            ")
    print("==================================================")
    
    workspace_dir = find_repo_root()
    output_dir = workspace_dir / "graphify-out"
    
    checks = {}
    
    # 1. Python version check
    py_version = f"{sys.version_info.major}.{sys.version_info.minor}.{sys.version_info.micro}"
    if sys.version_info >= (3, 10):
        checks["Python Version"] = ("PASS", f"Python {py_version} (>= 3.10)")
    else:
        checks["Python Version"] = ("FAIL", f"Python {py_version} is unsupported (requires >= 3.10)")
        
    # 2. Python dependencies check
    missing_deps = check_python_dependencies()
    if not missing_deps:
        checks["Python Dependencies"] = ("PASS", "All packages imported successfully")
    else:
        checks["Python Dependencies"] = ("FAIL", f"Missing packages: {', '.join(missing_deps)}. Run: pip install -r requirements.txt")
        
    # 3. Rust toolchain check
    rustc_ver = None
    try:
        res = subprocess.run(["rustc", "--version"], capture_output=True, text=True, check=False)
        if res.returncode == 0:
            rustc_ver = res.stdout.strip()
    except Exception:
        pass
        
    if rustc_ver:
        checks["Rust Installed"] = ("PASS", rustc_ver)
    else:
        checks["Rust Installed"] = ("FAIL", "rustc is not installed. Install via https://rustup.rs/")
        
    # 4. Cargo check
    cargo_ver = None
    try:
        res = subprocess.run(["cargo", "--version"], capture_output=True, text=True, check=False)
        if res.returncode == 0:
            cargo_ver = res.stdout.strip()
    except Exception:
        pass
        
    if cargo_ver:
        checks["Cargo Installed"] = ("PASS", cargo_ver)
    else:
        checks["Cargo Installed"] = ("FAIL", "cargo is not installed. Install via https://rustup.rs/")
        
    # 5. Graphify CLI check
    graphify_cli_ver = None
    try:
        res = subprocess.run(["graphify", "--version"], capture_output=True, text=True, check=False)
        if res.returncode == 0:
            graphify_cli_ver = res.stdout.strip()
    except Exception:
        pass
        
    if not graphify_cli_ver:
        # Try via python -m graphify
        try:
            res = subprocess.run([sys.executable, "-m", "graphify", "--version"], capture_output=True, text=True, check=False)
            if res.returncode == 0:
                graphify_cli_ver = f"python -m {res.stdout.strip()}"
        except Exception:
            pass
            
    if graphify_cli_ver:
        match = re.search(r"(\d+\.\d+\.\d+)", graphify_cli_ver)
        if match:
            checks["Graphify CLI"] = ("PASS", f"Graphify version {match.group(1)}")
        else:
            checks["Graphify CLI"] = ("PASS", f"Graphify version found but unparsed format: {graphify_cli_ver}")
    else:
        checks["Graphify CLI"] = ("FAIL", "Graphify is not installed. Run: pip install -r requirements.txt")
        
    # 6. Configuration Check (.graphifyignore)
    ignore_path = workspace_dir / ".graphifyignore"
    if ignore_path.exists():
        try:
            with open(ignore_path, "r", encoding="utf-8") as f:
                content = f.read()
            if "graphify-out/" in content or "graphify-out" in content:
                checks["Graphify Config"] = ("PASS", ".graphifyignore exists and contains graphify-out")
            else:
                checks["Graphify Config"] = ("FAIL", ".graphifyignore exists but does not ignore graphify-out/ to prevent recursion")
        except Exception as e:
            checks["Graphify Config"] = ("FAIL", f"Failed to read .graphifyignore: {e}")
    else:
        checks["Graphify Config"] = ("FAIL", ".graphifyignore is missing")
        
    # 7. Output directory check
    if output_dir.exists() and output_dir.is_dir():
        checks["Output Directory"] = ("PASS", f"Exists: {output_dir}")
    else:
        checks["Output Directory"] = ("FAIL", f"Does not exist: {output_dir}")
        
    # Print environment checklist
    print("\nEnvironment & Configuration Checks:")
    print("-----------------------------------")
    env_failed = False
    for name, (status, detail) in checks.items():
        symbol = "[OK]" if status == "PASS" else "[FAIL]"
        print(f"[{status}] {symbol} {name}: {detail}")
        if status == "FAIL":
            env_failed = True
            
    # Run dynamic output validation if output dir exists
    output_failed = False
    output_results = {}
    if checks["Output Directory"][0] == "PASS":
        print("\nDynamic Output Validation:")
        print("---------------------------")
        valid, results = validate_graphify_outputs(output_dir)
        output_results = results
        if not valid:
            output_failed = True
            if "error" in results:
                print(f"[FAIL] [ERROR] Error scanning: {results['error']}")
            else:
                for file, res in results.items():
                    symbol = "[OK]" if res["status"] == "PASS" else "[FAIL]"
                    reason = f" ({res['reason']})" if res["status"] == "FAIL" else f" - {res.get('info', '')}"
                    print(f"[{res['status']}] {symbol} {file}{reason}")
        else:
            for file, res in results.items():
                print(f"[PASS] [OK] {file} - {res.get('info', '')}")
    else:
        output_failed = True
        
    print("\n==================================================")
    
    # Determine exit code and print summary
    if env_failed:
        if checks["Python Version"][0] == "FAIL":
            print("Summary: FAIL (Python version unsupported)")
            sys.exit(EXIT_VALIDATION_FAILED)
        elif checks["Python Dependencies"][0] == "FAIL":
            print("Summary: FAIL (Missing Python dependencies)")
            sys.exit(EXIT_MISSING_DEPENDENCY)
        elif checks["Graphify CLI"][0] == "FAIL":
            print("Summary: FAIL (Graphify CLI not installed)")
            sys.exit(EXIT_MISSING_DEPENDENCY)
        elif checks["Graphify Config"][0] == "FAIL":
            print("Summary: FAIL (Graphify configuration error)")
            sys.exit(EXIT_CONFIG_ERROR)
        elif checks["Rust Installed"][0] == "FAIL" or checks["Cargo Installed"][0] == "FAIL":
            print("Summary: FAIL (Rust/Cargo toolchain missing)")
            sys.exit(EXIT_MISSING_DEPENDENCY)
        else:
            print("Summary: FAIL (Environment checks failed)")
            sys.exit(EXIT_VALIDATION_FAILED)
            
    if output_failed:
        print("Summary: FAIL (Graph generation outputs missing or invalid)")
        sys.exit(EXIT_INVALID_OUTPUT)
        
    print("Summary: PASS (All checks completed successfully)")
    sys.exit(EXIT_SUCCESS)

if __name__ == "__main__":
    main()
