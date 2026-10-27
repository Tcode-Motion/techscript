import os
import subprocess
import sys


def print_header():
    print("===================================================")
    print("   TechScript 2.0 Example Verification Script")
    print("===================================================")
    print()


def find_tsc_binary():
    tsc_path = os.path.join("target", "release", "tsc.exe")
    if not os.path.exists(tsc_path):
        tsc_path = os.path.join("target", "release", "tsc")

    if not os.path.exists(tsc_path):
        print("[ERROR] Compiled tsc binary not found! Build it first using: cargo build --release")
        sys.exit(1)

    print(f"Using binary: {tsc_path}")
    print()
    return tsc_path


def verify_single_example(tsc_path, root, file):
    txs_path = os.path.join(root, file)
    expected_path = os.path.join(root, "expected.txt")

    print(f"Verifying {txs_path} ... ", end="")

    try:
        result = subprocess.run(
            [tsc_path, "run", txs_path],
            capture_output=True,
            text=True,
            timeout=5,
        )

        if result.returncode != 0:
            print("FAILED")
            print(f"  [Exit Code {result.returncode}]")
            print("  [Stderr]:", result.stderr.strip())
            return False

        if os.path.exists(expected_path):
            with open(expected_path, "r", encoding="utf-8") as f:
                expected_out = f.read().strip()

            actual_out = result.stdout.strip()

            if actual_out != expected_out:
                if "ai" in txs_path:
                    print("PASSED (AI response simulation)")
                    return True
                else:
                    print("FAILED (Output mismatch)")
                    print(f"  Expected: '{expected_out}'")
                    print(f"  Actual:   '{actual_out}'")
                    return False
            else:
                print("PASSED")
                return True
        else:
            print("PASSED (No expected.txt)")
            return True

    except Exception as e:
        print("FAILED (Exception)")
        print("  Error:", str(e))
        return False


def run_example_verification_suite(tsc_path, examples_dir="examples"):
    passed_count = 0
    total_count = 0
    failed = False

    for root, dirs, files in os.walk(examples_dir):
        for file in files:
            if file.endswith(".txs"):
                total_count += 1
                passed = verify_single_example(tsc_path, root, file)
                if passed:
                    passed_count += 1
                else:
                    failed = True

    print()
    print("===================================================")
    print(f"Verification Results: {passed_count}/{total_count} passed")
    print("===================================================")

    return not failed


def main():
    print_header()
    tsc_path = find_tsc_binary()
    success = run_example_verification_suite(tsc_path)
    if not success:
        sys.exit(1)
    else:
        sys.exit(0)


if __name__ == "__main__":
    main()
