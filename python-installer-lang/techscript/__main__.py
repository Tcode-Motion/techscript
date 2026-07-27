"""
TechScript CLI entry point.

Usage:
    techscript install           Install or update the compiler
    techscript install --force   Force reinstall
    techscript uninstall         Remove the compiler
    techscript version           Show installed version
    techscript check             Check if an update is available
    techscript run <file>        Forward a command to the installed tsc binary
"""

import shutil
import sys
import os

from . import __version__
from .platform_detect import detect
from .installer import (
    _ok, _info, _warn, _err, GREEN, YELLOW, RED, CYAN, BOLD, RESET,
    find_existing_tsc, prompt_existing_install, install_binary,
    configure_path, verify_installation, uninstall, get_installed_version,
)
from .downloader import (
    check_internet, get_latest_tag, download_and_extract,
)
from .launcher import launch


BANNER = f"""\
{CYAN}{BOLD}
  ╔══════════════════════════════════════════╗
  ║      TechScript 2.0 — Installer         ║
  ║   Write like a Human. Run like Rust.    ║
  ╚══════════════════════════════════════════╝
{RESET}"""


def cmd_install(force: bool = False, debug: bool = False) -> int:
    """Download and install the TechScript compiler."""
    print(BANNER)

    # ── Platform detection ──────────────────────────────────────────────────
    platform_info = detect()
    _info(f"Platform detected: {platform_info.display_name}")
    _info(f"Python  : {'.'.join(str(x) for x in platform_info.python_version)}")

    if not platform_info.is_supported:
        _err(f"Unsupported platform: {platform_info.display_name}")
        _err("Please open an issue: https://github.com/Tcode-Motion/techscript/issues")
        return 1

    # ── Check for existing install ──────────────────────────────────────────
    existing = find_existing_tsc()
    if existing and not force:
        action = prompt_existing_install(existing)
        if action == "skip":
            _info("Skipping installation. Existing install kept.")
            return 0
        elif action == "uninstall":
            uninstall(platform_info)
            return 0
        # For update/repair/reinstall, continue with download
        print()

    # ── Internet check ──────────────────────────────────────────────────────
    _info("Checking internet connectivity...")
    if not check_internet():
        _err("No internet connection. Cannot download TechScript.")
        return 1

    # ── Fetch latest release ────────────────────────────────────────────────
    _info("Fetching latest release information...")
    tag = get_latest_tag()
    _info(f"Target release: {tag}")

    asset_name = platform_info.asset_name
    _info(f"Package       : {asset_name}")
    print()

    # ── Download & extract ──────────────────────────────────────────────────
    try:
        binary_path, tmp_dir = download_and_extract(
            tag, asset_name, platform_info.binary_name
        )
    except RuntimeError as e:
        _err(str(e))
        if debug:
            raise
        return 1

    # ── Install ─────────────────────────────────────────────────────────────
    try:
        print()
        installed_path = install_binary(binary_path, platform_info)
    except PermissionError as e:
        _err(f"Permission denied: {e}")
        _warn("Try running with elevated privileges, or set a custom install dir.")
        return 1
    finally:
        import shutil as _shutil
        _shutil.rmtree(tmp_dir, ignore_errors=True)

    # ── PATH setup ──────────────────────────────────────────────────────────
    configure_path(platform_info, platform_info.install_dir)

    # ── Verify ──────────────────────────────────────────────────────────────
    print()
    ok = verify_installation(installed_path)

    print()
    if ok:
        print(f"  {GREEN}{BOLD}══════════════════════════════════════════{RESET}")
        print(f"  {GREEN}{BOLD}   TechScript installed successfully! 🎉  {RESET}")
        print(f"  {GREEN}{BOLD}══════════════════════════════════════════{RESET}")
        print()
        print("  Create your first project:")
        print(f"    {CYAN}tsc new hello_world{RESET}")
        print(f"    {CYAN}cd hello_world{RESET}")
        print(f"    {CYAN}tsc run{RESET}")
        print()
        if platform_info.install_dir not in os.environ.get("PATH", ""):
            print(f"  {YELLOW}NOTE: Restart your terminal (or run `source ~/.bashrc`) to activate PATH changes.{RESET}")
        return 0
    else:
        _err("Installation verification failed.")
        _err("Try running: techscript install --force")
        return 1


def cmd_uninstall(debug: bool = False) -> int:
    platform_info = detect()
    print(BANNER)
    _info("Uninstalling TechScript...")
    uninstall(platform_info)
    return 0


def cmd_version() -> int:
    ver = get_installed_version()
    if ver:
        print(ver)
    else:
        _warn("TechScript is not installed.")
        print("  Run: techscript install")
    return 0


def cmd_check() -> int:
    print(BANNER)
    _info("Checking for updates...")
    installed_ver = get_installed_version()
    latest_tag = get_latest_tag()
    if installed_ver:
        _info(f"Installed : {installed_ver}")
    else:
        _warn("TechScript is not installed.")
    _info(f"Latest    : {latest_tag}")
    print()
    if installed_ver and latest_tag.replace("release-", "v") in installed_ver:
        _ok("You are up to date!")
    else:
        _warn("A newer version may be available.")
        print("  Run: techscript install  to update.")
    return 0


def main(argv=None) -> int:
    args = argv if argv is not None else sys.argv[1:]
    debug = "--debug" in args
    args = [a for a in args if a != "--debug"]

    if not args or args[0] in ("-h", "--help", "help"):
        print(__doc__)
        return 0

    cmd = args[0]

    if cmd == "install":
        force = "--force" in args or "-f" in args
        return cmd_install(force=force, debug=debug)

    elif cmd == "uninstall":
        return cmd_uninstall(debug=debug)

    elif cmd in ("version", "--version", "-V"):
        return cmd_version()

    elif cmd == "check":
        return cmd_check()

    elif cmd == "run":
        # Forward: techscript run <args...> → tsc run <args...>
        return launch(["run"] + args[1:])

    else:
        # Forward any other subcommand directly to tsc
        return launch(args)


if __name__ == "__main__":
    sys.exit(main())
