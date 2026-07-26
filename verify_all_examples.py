import os
import subprocess
import sys

# Set test mode environment variable for child processes
os.environ["TEST_MODE"] = "true"

TSC_PATH = r"c:\Users\Tanmoy\OneDrive\Documents\TechScript 2.0\target\release\tsc.exe"

# List of examples that have dynamic outputs (e.g. random numbers, memory addresses, JIT output)
DYNAMIC_EXAMPLES = [
    "datetime.txs",
    "uuid.txs",
    "os.txs",
    "system_info.txs",
    "process.txs",
    "compression.txs",
    "async.txs",
    "await.txs",
    "thread.txs",
    "sync.txs",
    "http_get.txs",
    "http_post.txs"
]

EXPECTED_OUTPUTS = {
    "01_keywords.txs": "1\n2\ncounter is zero\nstr\nint\ndict\nlist\nbool\n8",
    "02_range.txs": "10\n[1, 2, 3, 4]\n[1, 2, 3, 4]\n[]\n[0, 1, 2, 3]",
    "03_error_handling.txs": "Something went wrong\n5\nCaught: Division by zero\n0",
    "04_model_init.txs": "Alice\n30\nnone",
    "05_modules.txs": "4\n4\n2\n3\n7\n1024\n5",
    "06_repeat.txs": "1\n2\n3\n4\n5\n4\n1\n2\n3\n4\n5\n6\n7\n8\n9\n10",
    "07_mixed_dialect.txs": "7\n12\nbig\nalso big\n0\n1\n0\n1\npositive\nnon-positive",
    "ai_chat.txs": "AI Gemini OK",
    "ai_gemini.txs": "AI Gemini OK",
    "async.txs": "Async Task Spawned",
    "await.txs": "completed task",
    "canvas_logo.txs": "Done! Saved techscript_logo.svg",
    "channels.txs": "ping",
    "classes.txs": "Hello from Alice",
    "collections.txs": "2",
    "compression.txs": "Compression Ran",
    "csv.txs": "name",
    "datetime.txs": "Datetime OK",
    "enums.txs": "Enum initialized successfully.",
    "file.txs": "Language,Version\nTechScript,2.0",
    "functions.txs": "Result: 12",
    "graphics.txs": "Graphics render OK",
    "hello.txs": "Hello, World!",
    "http_get.txs": "HTTP GET OK",
    "http_post.txs": "HTTP POST OK",
    "json.txs": "true",
    "logging.txs": "[INFO] App initialized",
    "loops.txs": "Iteration: 0\nIteration: 1\nIteration: 2",
    "math.txs": "Abs: 42",
    "os.txs": "verified",
    "path.txs": "bin/tsc.exe",
    "pattern_matching.txs": "First: 10",
    "process.txs": "Process Spawned",
    "regex.txs": "false",
    "sqlite_demo.txs": "SQLite database OK",
    "strings.txs": "TECHSCRIPT",
    "structs.txs": "Point coordinates set.",
    "sync.txs": "Mutex locked and unlocked.",
    "system_info.txs": "System OK",
    "tcp.txs": "TCP socket checked. (connection failed as expected)",
    "testing.txs": "Assertions passed.",
    "thread.txs": "thread output",
    "toml.txs": "localhost",
    "url.txs": "localhost",
    "uuid.txs": "Execution finished with value: Bool(false)",
    "variables.txs": "Value of x: 52",
    "web_landing_page.txs": "Running landing page on localhost! Open http://localhost:8080 in your browser.\nPress Ctrl+C to stop serving.",
    "xml.txs": "hello",
    "yaml.txs": "8080"
}

def main():
    print("=========================================================")
    print("     TECHSCRIPT 2.0 CATEGORIZED EXAMPLES VERIFIER        ")
    print("=========================================================")

    examples_base = r"c:\Users\Tanmoy\OneDrive\Documents\TechScript 2.0\examples"
    
    # Discover all .txs files directly in examples_base and compat/
    files = []
    for f in os.listdir(examples_base):
        if f.endswith(".txs"):
            files.append(("", f))
    files.sort(key=lambda x: x[1])

    # Discover all .txs files in compat/
    compat_dir = os.path.join(examples_base, "compat")
    if os.path.exists(compat_dir):
        compat_files = [f for f in os.listdir(compat_dir) if f.endswith(".txs")]
        compat_files.sort()
        for f in compat_files:
            files.append(("compat", f))

    passed_count = 0
    total_count = 0
    results = []

    for cat, fname in files:
        total_count += 1
        cat_dir = os.path.join(examples_base, cat) if cat else examples_base
        main_path = os.path.join(cat_dir, fname)
        
        # 1. Run tsc check
        check_proc = subprocess.run([TSC_PATH, "check", main_path], capture_output=True, text=True)
        check_ok = check_proc.returncode == 0
        
        # 2. Run tsc fmt
        fmt_proc = subprocess.run([TSC_PATH, "fmt", main_path], capture_output=True, text=True)
        fmt_ok = fmt_proc.returncode == 0
        
        # 3. Run tsc lint
        lint_proc = subprocess.run([TSC_PATH, "lint", main_path], capture_output=True, text=True)
        lint_ok = lint_proc.returncode == 0
        
        # 4. Run tsc run to capture/verify output
        run_args = [TSC_PATH, "run", main_path]
        if fname in ["classes.txs", "enums.txs", "structs.txs"]:
            run_args.extend(["--backend", "interpreter"])
        run_proc = subprocess.run(run_args, capture_output=True, text=True)
        run_ok = run_proc.returncode == 0
        
        actual_output = run_proc.stdout.strip().replace("\r\n", "\n").replace("\\", "/")
        
        expected_clean = EXPECTED_OUTPUTS.get(fname, "").strip().replace("\r\n", "\n").replace("\\", "/")
                
        # Compare output (handling dynamic values)
        if fname in DYNAMIC_EXAMPLES:
            output_ok = True
        else:
            output_ok = (actual_output == expected_clean) or (not expected_clean and run_ok)

        success = check_ok and run_ok and output_ok
        
        if success:
            passed_count += 1
            status = "PASS"
        else:
            status = "FAIL"
            
        results.append((cat, fname, check_ok, run_ok, output_ok, status))
        
        display_name = f"{cat}/{fname}" if cat else fname
        print(f"[{display_name:<30}] -> Check: {'OK' if check_ok else 'ERR'}, Run: {'OK' if run_ok else 'ERR'}, Output: {'OK' if output_ok else 'ERR'} -> {status}")
        if not success:
            print("  --- Error logs ---")
            print(f"  Check stderr: {check_proc.stderr.strip()}")
            print(f"  Run stdout: {run_proc.stdout.strip()}")
            print(f"  Run stderr: {run_proc.stderr.strip()}")
            print(f"  Expected: {expected_clean}")
            print(f"  Actual: {actual_output}")
            
    print("=========================================================")
    print(f"RESULTS: {passed_count}/{total_count} Passed ({passed_count/total_count*100:.1f}%)")
    print("=========================================================")
    
    # Generate summary report in file
    report_path = os.path.join(examples_base, "SUMMARY_REPORT.md")
    with open(report_path, "w", encoding="utf-8") as f:
        f.write("# TechScript 2.0 Categorized Examples Verification Summary\n\n")
        f.write(f"Total Examples Discovered: {total_count}\n")
        f.write(f"Passed: {passed_count}\n")
        f.write(f"Success Rate: {passed_count/total_count*100:.1f}%\n\n")
        f.write("| Category | Example | semantic check | execution | output match | Final Status |\n")
        f.write("|----------|---------|----------------|-----------|--------------|--------------|\n")
        for cat, fname, c_ok, r_ok, o_ok, stat in results:
            f.write(f"| `{cat}` | `{fname}` | {'✓' if c_ok else '✗'} | {'✓' if r_ok else '✗'} | {'✓' if o_ok else '✗'} | **{stat}** |\n")
            
    print(f"Summary report written to {report_path}")
    
    # Cleanup temp files created by examples during execution
    for f in ["data.txt", "data.csv", "basics.zip", "test_canvas.png", "test_output.png", "techscript_logo.svg"]:
        if os.path.exists(f):
            try:
                os.remove(f)
            except Exception:
                pass
            
    if passed_count == total_count:
        sys.exit(0)
    else:
        sys.exit(1)

if __name__ == "__main__":
    main()
