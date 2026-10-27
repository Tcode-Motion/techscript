import os
import subprocess
import sys


def print_header() -> None:
    """Print the verification script header banner."""
    print("===================================================")
    print("   TechScript 2.0 Example Verification Script")
    print("===================================================")
    print()


def find_tsc_binary(root_dir: str = ".") -> str | None:
    """Locate the compiled TechScript executable binary."""
    candidates = [
        os.path.join(root_dir, "target", "release", "tsc"),
        os.path.join(root_dir, "target", "release", "tsc.exe"),
        os.path.join(root_dir, "target", "debug", "tsc"),
        os.path.join(root_dir, "target", "debug", "tsc.exe"),
    ]
    for candidate in candidates:
        if os.path.isfile(candidate):
            return candidate
    return None


def find_example_files(examples_dir: str = "examples") -> list[str]:
    """Find and return all .txs files in the examples directory, sorted."""
    txs_files = []
    if not os.path.isdir(examples_dir):
        return txs_files

    for root, _, files in os.walk(examples_dir):
        for file in files:
            if file.endswith(".txs"):
                txs_files.append(os.path.join(root, file))

    txs_files.sort()
    return txs_files


def normalize_output(text: str) -> str:
    """Normalize output text by stripping carriage returns and trailing whitespace."""
    lines = text.replace("\r", "").splitlines()
    stripped_lines = [line.rstrip() for line in lines]
    return "\n".join(stripped_lines).strip()


def verify_single_example(tsc_bin: str, txs_file: str) -> tuple[bool, str]:
    """Run a single example .txs file and verify its output against expected.txt if present."""
    dir_name = os.path.dirname(txs_file)
    expected_file = os.path.join(dir_name, "expected.txt")

    try:
        proc = subprocess.run(
            [tsc_bin, "run", txs_file],
            capture_output=True,
            text=True,
            check=False,
        )
    except Exception as err:
        return False, f"FAILED (Execution error: {err})"

    if proc.returncode != 0:
        return False, f"FAILED (Execution error, exit code {proc.returncode})"

    if os.path.isfile(expected_file):
        try:
            with open(expected_file, "r", encoding="utf-8") as f:
                expected_content = normalize_output(f.read())
        except Exception as err:
            return False, f"FAILED (Could not read expected file: {err})"

        actual_content = normalize_output(proc.stdout)
        if expected_content != actual_content:
            return False, "FAILED (Output mismatch)"

    return True, "OK"


def print_summary(passed: int, total: int, failed: int) -> None:
    """Print verification summary report."""
    print()
    print("===================================================")
    print(f"Summary: {passed} / {total} passed, {failed} failed.")
    print("===================================================")


def main() -> int:
    """Main function coordinating the example verification workflow."""
    print_header()

    tsc_bin = find_tsc_binary()
    if not tsc_bin:
        print("[ERROR] Compiled tsc binary not found! Run 'cargo build --release' first.")
        return 1

    print(f"Using binary: {tsc_bin}")
    print()

    txs_files = find_example_files()
    passed = 0
    failed = 0
    total = len(txs_files)

    for txs_file in txs_files:
        print(f"Verifying {txs_file} ... ", end="", flush=True)
        ok, msg = verify_single_example(tsc_bin, txs_file)
        print(msg)
        if ok:
            passed += 1
        else:
            failed += 1

    print_summary(passed, total, failed)

    return 1 if failed > 0 else 0


if __name__ == "__main__":
    sys.exit(main())
