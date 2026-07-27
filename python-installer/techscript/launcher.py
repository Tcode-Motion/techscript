"""
TechScript launcher module.
Used to forward `tsc` commands when invoked through `python -m techscript run`.
"""

import os
import shutil
import subprocess
import sys


def find_tsc() -> str:
    """Find the tsc binary or raise an error."""
    tsc = shutil.which("tsc")
    if not tsc:
        # Also check the default install locations explicitly
        candidates = []
        local_app = os.environ.get("LOCALAPPDATA", "")
        if local_app:
            candidates.append(os.path.join(local_app, "TechScript", "bin", "tsc.exe"))
        candidates.append(os.path.expanduser("~/.local/bin/tsc"))
        prefix = os.environ.get("PREFIX", "")
        if prefix:
            candidates.append(os.path.join(prefix, "bin", "tsc"))

        for path in candidates:
            if os.path.isfile(path) and os.access(path, os.X_OK):
                return path

        print("\n  ✘  TechScript compiler (tsc) is not installed or not found in PATH.", file=sys.stderr)
        print("     Run:  techscript install", file=sys.stderr)
        sys.exit(1)
    return tsc


def launch(args: list) -> int:
    """Forward args to the tsc binary and return the exit code."""
    tsc = find_tsc()
    try:
        result = subprocess.run([tsc] + args)
        return result.returncode
    except KeyboardInterrupt:
        return 130
