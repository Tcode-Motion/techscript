"""
TechScript platform detection module.
Identifies OS, CPU architecture, Termux environment, and Python version.
"""

import os
import platform
import struct
import sys
from dataclasses import dataclass
from enum import Enum, auto
from typing import Optional


class OS(Enum):
    WINDOWS = auto()
    LINUX = auto()
    MACOS = auto()
    UNKNOWN = auto()


class Arch(Enum):
    X64 = auto()
    ARM64 = auto()
    ARMV7 = auto()
    UNKNOWN = auto()


@dataclass
class PlatformInfo:
    os: OS
    arch: Arch
    is_termux: bool
    termux_prefix: Optional[str]
    python_version: tuple

    @property
    def display_name(self) -> str:
        os_names = {OS.WINDOWS: "Windows", OS.LINUX: "Linux", OS.MACOS: "macOS", OS.UNKNOWN: "Unknown"}
        arch_names = {Arch.X64: "x64", Arch.ARM64: "ARM64", Arch.ARMV7: "ARMv7", Arch.UNKNOWN: "Unknown"}
        termux_tag = " (Termux)" if self.is_termux else ""
        return f"{os_names[self.os]} {arch_names[self.arch]}{termux_tag}"

    @property
    def is_supported(self) -> bool:
        return self.asset_name is not None

    @property
    def asset_name(self) -> Optional[str]:
        """Return the GitHub Release asset filename for this platform."""
        # Termux on Android
        if self.is_termux:
            if self.arch == Arch.ARM64:
                return "techscript-linux-arm64.tar.gz"
            elif self.arch == Arch.ARMV7:
                return "techscript-linux-armv7.tar.gz"
            return None

        if self.os == OS.WINDOWS:
            if self.arch == Arch.X64:
                return "techscript-windows-x64.zip"
            elif self.arch == Arch.ARM64:
                return "techscript-windows-arm64.zip"

        elif self.os == OS.LINUX:
            if self.arch == Arch.X64:
                return "techscript-linux-x64.tar.gz"
            elif self.arch == Arch.ARM64:
                return "techscript-linux-arm64.tar.gz"

        elif self.os == OS.MACOS:
            if self.arch == Arch.ARM64:
                return "techscript-macos-arm64.tar.gz"
            elif self.arch == Arch.X64:
                return "techscript-macos-x64.tar.gz"

        return None

    @property
    def install_dir(self) -> str:
        """Return the default installation directory for this platform."""
        if self.is_termux and self.termux_prefix:
            return os.path.join(self.termux_prefix, "bin")
        if self.os == OS.WINDOWS:
            local_app_data = os.environ.get("LOCALAPPDATA", os.path.expanduser("~\\AppData\\Local"))
            return os.path.join(local_app_data, "TechScript", "bin")
        return os.path.join(os.path.expanduser("~"), ".local", "bin")

    @property
    def binary_name(self) -> str:
        """Return the name of the tsc binary file."""
        return "tsc.exe" if self.os == OS.WINDOWS else "tsc"


def _detect_os() -> OS:
    system = platform.system().lower()
    if system == "windows":
        return OS.WINDOWS
    elif system == "linux":
        return OS.LINUX
    elif system == "darwin":
        return OS.MACOS
    return OS.UNKNOWN


def _detect_arch() -> Arch:
    machine = platform.machine().lower()
    # Normalize common aliases
    if machine in ("x86_64", "amd64", "x64"):
        return Arch.X64
    elif machine in ("aarch64", "arm64"):
        return Arch.ARM64
    elif machine.startswith("armv7") or machine == "armhf":
        return Arch.ARMV7
    # Fallback: check pointer size
    if struct.calcsize("P") == 8:
        return Arch.X64
    return Arch.UNKNOWN


def _detect_termux() -> tuple[bool, Optional[str]]:
    """Detect if running inside Termux on Android."""
    prefix = os.environ.get("PREFIX", "")
    if "com.termux" in prefix or "termux" in prefix.lower():
        return True, prefix
    # Also check for ANDROID_DATA which is always set on Android
    if os.path.exists("/data/data/com.termux"):
        termux_prefix = os.environ.get("PREFIX", "/data/data/com.termux/files/usr")
        return True, termux_prefix
    return False, None


def detect() -> PlatformInfo:
    """Detect the current platform and return a PlatformInfo object."""
    is_termux, termux_prefix = _detect_termux()
    return PlatformInfo(
        os=_detect_os(),
        arch=_detect_arch(),
        is_termux=is_termux,
        termux_prefix=termux_prefix,
        python_version=sys.version_info[:3],
    )
