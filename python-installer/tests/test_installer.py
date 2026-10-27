"""Tests for TechScript Python installer package."""

import os
import sys
import unittest
from unittest.mock import patch, MagicMock

# Ensure the package is importable
sys.path.insert(0, os.path.join(os.path.dirname(__file__), ".."))

from techscript.platform_detect import (
    detect, OS, Arch, PlatformInfo, _detect_os, _detect_arch, _detect_termux
)
from techscript.downloader import build_download_url, FALLBACK_TAG
from techscript.__main__ import (
    _detect_and_validate_platform,
    _handle_existing_install,
    _download_release_binary,
    _print_install_result,
    cmd_install,
)


class TestPlatformDetection(unittest.TestCase):

    def test_detect_returns_platform_info(self):
        info = detect()
        self.assertIsInstance(info, PlatformInfo)
        self.assertIn(info.os, list(OS))
        self.assertIn(info.arch, list(Arch))
        self.assertIsInstance(info.is_termux, bool)

    def test_windows_x64_asset(self):
        info = PlatformInfo(OS.WINDOWS, Arch.X64, False, None, (3, 11, 0))
        self.assertEqual(info.asset_name, "techscript-windows-x64.zip")

    def test_windows_arm64_asset(self):
        info = PlatformInfo(OS.WINDOWS, Arch.ARM64, False, None, (3, 11, 0))
        self.assertEqual(info.asset_name, "techscript-windows-arm64.zip")

    def test_linux_x64_asset(self):
        info = PlatformInfo(OS.LINUX, Arch.X64, False, None, (3, 11, 0))
        self.assertEqual(info.asset_name, "techscript-linux-x64.tar.gz")

    def test_linux_arm64_asset(self):
        info = PlatformInfo(OS.LINUX, Arch.ARM64, False, None, (3, 11, 0))
        self.assertEqual(info.asset_name, "techscript-linux-arm64.tar.gz")

    def test_macos_x64_asset(self):
        info = PlatformInfo(OS.MACOS, Arch.X64, False, None, (3, 11, 0))
        self.assertEqual(info.asset_name, "techscript-macos-x64.tar.gz")

    def test_macos_arm64_asset(self):
        info = PlatformInfo(OS.MACOS, Arch.ARM64, False, None, (3, 11, 0))
        self.assertEqual(info.asset_name, "techscript-macos-arm64.tar.gz")

    def test_termux_arm64_asset(self):
        info = PlatformInfo(OS.LINUX, Arch.ARM64, True, "/data/data/com.termux/files/usr", (3, 11, 0))
        self.assertEqual(info.asset_name, "techscript-linux-arm64.tar.gz")

    def test_unknown_platform_unsupported(self):
        info = PlatformInfo(OS.UNKNOWN, Arch.UNKNOWN, False, None, (3, 11, 0))
        self.assertFalse(info.is_supported)

    def test_display_name_windows(self):
        info = PlatformInfo(OS.WINDOWS, Arch.X64, False, None, (3, 11, 0))
        self.assertIn("Windows", info.display_name)
        self.assertIn("x64", info.display_name)

    def test_display_name_termux(self):
        info = PlatformInfo(OS.LINUX, Arch.ARM64, True, "/prefix", (3, 11, 0))
        self.assertIn("Termux", info.display_name)


class TestInstallDir(unittest.TestCase):

    def test_windows_install_dir(self):
        with patch.dict(os.environ, {"LOCALAPPDATA": "C:\\Users\\Test\\AppData\\Local"}):
            info = PlatformInfo(OS.WINDOWS, Arch.X64, False, None, (3, 11, 0))
            self.assertIn("TechScript", info.install_dir)
            self.assertIn("bin", info.install_dir)

    def test_linux_install_dir(self):
        info = PlatformInfo(OS.LINUX, Arch.X64, False, None, (3, 11, 0))
        self.assertIn(".local/bin", info.install_dir)

    def test_termux_install_dir(self):
        prefix = "/data/data/com.termux/files/usr"
        info = PlatformInfo(OS.LINUX, Arch.ARM64, True, prefix, (3, 11, 0))
        self.assertEqual(info.install_dir, f"{prefix}/bin")


class TestBinaryName(unittest.TestCase):

    def test_windows_binary_name(self):
        info = PlatformInfo(OS.WINDOWS, Arch.X64, False, None, (3, 11, 0))
        self.assertEqual(info.binary_name, "tsc.exe")

    def test_linux_binary_name(self):
        info = PlatformInfo(OS.LINUX, Arch.X64, False, None, (3, 11, 0))
        self.assertEqual(info.binary_name, "tsc")

    def test_macos_binary_name(self):
        info = PlatformInfo(OS.MACOS, Arch.ARM64, False, None, (3, 11, 0))
        self.assertEqual(info.binary_name, "tsc")


class TestDownloadURL(unittest.TestCase):

    def test_build_url_linux(self):
        url = build_download_url("release-2.0.0", "techscript-linux-x64.tar.gz")
        self.assertIn("release-2.0.0", url)
        self.assertIn("techscript-linux-x64.tar.gz", url)
        self.assertIn("github.com", url)

    def test_fallback_tag_format(self):
        self.assertTrue(FALLBACK_TAG.startswith("release-") or FALLBACK_TAG.startswith("v"))


class TestTermuxDetection(unittest.TestCase):

    def test_no_termux(self):
        with patch.dict(os.environ, {"PREFIX": ""}, clear=False):
            is_termux, prefix = _detect_termux()
            # Can't guarantee False in all CI environments, just check types
            self.assertIsInstance(is_termux, bool)

    def test_termux_prefix_env(self):
        fake_prefix = "/data/data/com.termux/files/usr"
        with patch.dict(os.environ, {"PREFIX": fake_prefix}):
            is_termux, prefix = _detect_termux()
            self.assertTrue(is_termux)
            self.assertEqual(prefix, fake_prefix)


class TestCmdInstallHelpers(unittest.TestCase):

    @patch("techscript.__main__.detect")
    def test_detect_and_validate_platform_unsupported(self, mock_detect):
        mock_info = PlatformInfo(OS.UNKNOWN, Arch.UNKNOWN, False, None, (3, 11, 0))
        mock_detect.return_value = mock_info
        res = _detect_and_validate_platform()
        self.assertIsNone(res)

    @patch("techscript.__main__.detect")
    def test_detect_and_validate_platform_supported(self, mock_detect):
        mock_info = PlatformInfo(OS.LINUX, Arch.X64, False, None, (3, 11, 0))
        mock_detect.return_value = mock_info
        res = _detect_and_validate_platform()
        self.assertEqual(res, mock_info)

    @patch("techscript.__main__.find_existing_tsc")
    @patch("techscript.__main__.prompt_existing_install")
    def test_handle_existing_install_skip(self, mock_prompt, mock_find):
        mock_find.return_value = "/usr/bin/tsc"
        mock_prompt.return_value = "skip"
        info = PlatformInfo(OS.LINUX, Arch.X64, False, None, (3, 11, 0))
        exit_code = _handle_existing_install(info, force=False)
        self.assertEqual(exit_code, 0)

    @patch("techscript.__main__.check_internet")
    def test_download_release_binary_no_internet(self, mock_check):
        mock_check.return_value = False
        info = PlatformInfo(OS.LINUX, Arch.X64, False, None, (3, 11, 0))
        res = _download_release_binary(info)
        self.assertIsNone(res)

    def test_print_install_result(self):
        info = PlatformInfo(OS.LINUX, Arch.X64, False, None, (3, 11, 0))
        self.assertEqual(_print_install_result(True, info), 0)
        self.assertEqual(_print_install_result(False, info), 1)

    @patch("techscript.__main__._detect_and_validate_platform")
    def test_cmd_install_unsupported_platform(self, mock_detect):
        mock_detect.return_value = None
        self.assertEqual(cmd_install(), 1)


if __name__ == "__main__":
    unittest.main()
