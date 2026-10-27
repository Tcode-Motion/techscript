"""
TechScript installer module.
Handles placing the binary, PATH configuration, and verification.
"""

import os
import platform
import shutil
import stat
import subprocess
import sys
from typing import Optional

from .platform_detect import PlatformInfo, OS


# ─── Colours ─────────────────────────────────────────────────────────────────

def _supports_color() -> bool:
    return sys.stdout.isatty() and os.environ.get("NO_COLOR") is None

_C = _supports_color()
GREEN  = "\033[92m" if _C else ""
YELLOW = "\033[93m" if _C else ""
RED    = "\033[91m" if _C else ""
CYAN   = "\033[96m" if _C else ""
BOLD   = "\033[1m"  if _C else ""
RESET  = "\033[0m"  if _C else ""


def _ok(msg: str)   -> None: print(f"  {GREEN}✔{RESET}  {msg}")
def _info(msg: str) -> None: print(f"  {CYAN}ℹ{RESET}  {msg}")
def _warn(msg: str) -> None: print(f"  {YELLOW}⚠{RESET}  {msg}")
def _err(msg: str)  -> None: print(f"  {RED}✘{RESET}  {msg}")


# ─── Existing install detection ───────────────────────────────────────────────

def find_existing_tsc() -> Optional[str]:
    """Return the path to an existing tsc binary, or None."""
    return shutil.which("tsc")


def get_installed_version(tsc_path: Optional[str] = None) -> Optional[str]:
    """Return the installed tsc version string, or None."""
    binary = tsc_path or shutil.which("tsc")
    if not binary:
        return None
    try:
        result = subprocess.run(
            [binary, "version"],
            capture_output=True, text=True, timeout=10
        )
        for line in result.stdout.splitlines():
            if "Compiler Driver" in line or "v2." in line:
                return line.strip()
        return result.stdout.strip().splitlines()[0] if result.stdout.strip() else None
    except Exception:
        return None


def prompt_existing_install(existing_path: str) -> str:
    """
    Ask the user what to do when a TechScript install is already found.
    Returns one of: 'update', 'repair', 'reinstall', 'skip'
    """
    current_ver = get_installed_version(existing_path)
    print(f"\n  {YELLOW}{BOLD}TechScript is already installed!{RESET}")
    print(f"    Location: {existing_path}")
    if current_ver:
        print(f"    Version : {current_ver}")
    print()
    print("  What would you like to do?")
    print("    [1] Update    — download the latest version")
    print("    [2] Repair    — redownload and reinstall current version")
    print("    [3] Reinstall — remove and install fresh")
    print("    [4] Skip      — keep the existing install")
    print()

    while True:
        try:
            choice = input("  Enter choice [1-4]: ").strip()
        except (EOFError, KeyboardInterrupt):
            print()
            return "skip"
        mapping = {"1": "update", "2": "repair", "3": "reinstall", "4": "skip"}
        if choice in mapping:
            return mapping[choice]
        print("  Please enter 1, 2, 3, or 4.")


# ─── Installation ─────────────────────────────────────────────────────────────

def ensure_install_dir(install_dir: str) -> None:
    """Create the installation directory if it doesn't exist."""
    os.makedirs(install_dir, exist_ok=True)


def install_binary(src_binary: str, platform_info: PlatformInfo,
                   install_dir: Optional[str] = None) -> str:
    """
    Copy the binary to the installation directory.
    Returns the path to the installed binary.
    """
    dest_dir = install_dir or platform_info.install_dir
    ensure_install_dir(dest_dir)
    dest_path = os.path.join(dest_dir, platform_info.binary_name)

    _info(f"Installing to: {dest_path}")
    shutil.copy2(src_binary, dest_path)

    # Make executable on Unix
    if platform_info.os.name != "WINDOWS":
        st = os.stat(dest_path)
        os.chmod(dest_path, st.st_mode | stat.S_IEXEC | stat.S_IXGRP | stat.S_IXOTH)

    return dest_path


# ─── PATH configuration ───────────────────────────────────────────────────────

def _dir_in_path(directory: str) -> bool:
    path_dirs = os.environ.get("PATH", "").split(os.pathsep)
    return any(os.path.normcase(d) == os.path.normcase(directory) for d in path_dirs)


def configure_path_unix(install_dir: str) -> None:
    """Append install_dir to the user's shell rc file if not already present."""
    if _dir_in_path(install_dir):
        _ok(f"{install_dir} is already in PATH")
        return

    shell = os.environ.get("SHELL", "")
    if "zsh" in shell:
        rc = os.path.expanduser("~/.zshrc")
    elif "fish" in shell:
        rc = os.path.expanduser("~/.config/fish/config.fish")
    else:
        rc = os.path.expanduser("~/.bashrc")

    export_line = f'\nexport PATH="{install_dir}:$PATH"  # TechScript toolchain\n'
    if os.path.exists(rc):
        content = open(rc).read()
        if install_dir not in content:
            with open(rc, "a") as f:
                f.write(export_line)
            _ok(f"Added {install_dir} to PATH in {rc}")
    else:
        _warn(f"Shell rc not found. Please add this to your shell profile manually:")
        _warn(f'    export PATH="{install_dir}:$PATH"')


def configure_path_windows(install_dir: str) -> None:
    """Add install_dir to the Windows user PATH via the registry."""
    if _dir_in_path(install_dir):
        _ok(f"{install_dir} is already in PATH")
        return
    try:
        import winreg
        key = winreg.OpenKey(
            winreg.HKEY_CURRENT_USER,
            r"Environment",
            0, winreg.KEY_READ | winreg.KEY_WRITE
        )
        try:
            existing, _ = winreg.QueryValueEx(key, "Path")
        except FileNotFoundError:
            existing = ""

        if install_dir.lower() not in existing.lower():
            new_path = existing.rstrip(";") + ";" + install_dir
            winreg.SetValueEx(key, "Path", 0, winreg.REG_EXPAND_SZ, new_path)
            _ok(f"Added {install_dir} to Windows user PATH (restart your terminal to apply)")
        winreg.CloseKey(key)
    except Exception as e:
        _warn(f"Could not update PATH automatically: {e}")
        _warn(f"Please add this directory to your PATH manually: {install_dir}")


def configure_path(platform_info: PlatformInfo, install_dir: str) -> None:
    if platform_info.os.name == "WINDOWS":
        configure_path_windows(install_dir)
    else:
        configure_path_unix(install_dir)


# ─── Verification ─────────────────────────────────────────────────────────────

def verify_installation(installed_path: str) -> bool:
    """Run tsc version to confirm it works. Returns True on success."""
    try:
        result = subprocess.run(
            [installed_path, "version"],
            capture_output=True, text=True, timeout=15
        )
        if result.returncode == 0:
            _ok("TechScript installed successfully!")
            for line in result.stdout.strip().splitlines()[:3]:
                print(f"     {line}")
            return True
        else:
            _err(f"tsc returned exit code {result.returncode}")
            return False
    except FileNotFoundError:
        _err(f"Binary not found at {installed_path}")
        return False
    except subprocess.TimeoutExpired:
        _err("Verification timed out")
        return False
    except Exception as e:
        _err(f"Verification failed: {e}")
        return False


# ─── Uninstall ────────────────────────────────────────────────────────────────

def uninstall(platform_info: PlatformInfo, install_dir: Optional[str] = None) -> None:
    """Remove the tsc binary from the installation directory."""
    dest_dir = install_dir or platform_info.install_dir
    dest_path = os.path.join(dest_dir, platform_info.binary_name)

    if os.path.exists(dest_path):
        os.remove(dest_path)
        _ok(f"Removed: {dest_path}")
    else:
        existing = shutil.which("tsc")
        if existing:
            os.remove(existing)
            _ok(f"Removed: {existing}")
        else:
            _warn("TechScript is not installed — nothing to remove.")
