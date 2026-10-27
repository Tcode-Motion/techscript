"""
TechScript 2.0 — Official Python Installer / Bootstrapper

Detects your platform, downloads the correct native binary from GitHub Releases,
and installs the `tsc` compiler into the right location on your PATH.

Usage:
    techscript install          # Install or update the compiler
    techscript install --force  # Force reinstall
    techscript uninstall        # Remove the compiler
    techscript version          # Show installed version
    techscript check            # Check for updates
"""

__version__ = "2.0.0"
__author__ = "Tcode-Motion"
__repo__ = "https://github.com/Tcode-Motion/techscript"
