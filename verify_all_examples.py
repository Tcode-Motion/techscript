import os
import subprocess
import sys

TSC_PATH = "tsc"  # Use the globally installed compiler on system PATH

# List of examples that have dynamic outputs (e.g. random numbers, memory addresses, JIT output)
DYNAMIC_EXAMPLES = [
    "09_datetime.txs",
    "10_uuid.txs",
    "03_os.txs",
    "04_system_info.txs",
    "05_process.txs",
    "06_compression.txs",
    "01_async.txs",
    "02_await.txs",
    "04_thread.txs",
    "05_sync.txs"
]

def main():
    print("=========================================================")
    print("     TECHSCRIPT 2.0 CATEGORIZED EXAMPLES VERIFIER        ")
    print("=========================================================")

    examples_base = r"c:\Users\Tanmoy\OneDrive\Documents\TechScript 2.0\examples"
    
    # Discover all category directories
    categories = [d for d in os.listdir(examples_base) if os.path.isdir(os.path.join(examples_base, d))]
    categories.sort()
    
    passed_count = 0
    total_count = 0
    results = []

    for cat in categories:
        cat_dir = os.path.join(examples_base, cat)
        # Find all .txs files in this category
        files = [f for f in os.listdir(cat_dir) if f.endswith(".txs")]
        files.sort()
        
        for fname in files:
            total_count += 1
            main_path = os.path.join(cat_dir, fname)
            expected_path = os.path.join(cat_dir, fname.replace(".txs", ".expected.txt"))
            
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
            run_proc = subprocess.run([TSC_PATH, "run", main_path], capture_output=True, text=True)
            run_ok = run_proc.returncode == 0
            
            actual_output = run_proc.stdout.strip().replace("\r\n", "\n").replace("\\", "/")
            
            # If expected file doesn't exist, capture current stdout (if run succeeded)
            if not os.path.exists(expected_path):
                if run_ok:
                    with open(expected_path, "w", encoding="utf-8") as f:
                        f.write(actual_output + "\n")
                    expected_clean = actual_output
                else:
                    expected_clean = ""
            else:
                with open(expected_path, "r", encoding="utf-8") as f:
                    expected_clean = f.read().strip().replace("\r\n", "\n").replace("\\", "/")
                    
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
            
            print(f"[{cat}/{fname:<25}] -> Check: {'OK' if check_ok else 'ERR'}, Run: {'OK' if run_ok else 'ERR'}, Output: {'OK' if output_ok else 'ERR'} -> {status}")
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
    for f in ["data.txt", "basics.zip", "test_canvas.png", "test_output.png"]:
        if os.path.exists(f):
            os.remove(f)
            
    if passed_count == total_count:
        sys.exit(0)
    else:
        sys.exit(1)

if __name__ == "__main__":
    main()
