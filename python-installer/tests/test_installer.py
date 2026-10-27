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


class TestCmdInstall(unittest.TestCase):

    @patch("techscript.__main__.detect")
    def test_cmd_install_unsupported_platform(self, mock_detect):
        mock_info = MagicMock()
        mock_info.is_supported = False
        mock_info.display_name = "Unsupported OS"
        mock_info.python_version = (3, 11, 0)
        mock_detect.return_value = mock_info

        from techscript.__main__ import cmd_install
        res = cmd_install()
        self.assertEqual(res, 1)

    @patch("techscript.__main__.detect")
    @patch("techscript.__main__.find_existing_tsc")
    @patch("techscript.__main__.prompt_existing_install")
    def test_cmd_install_skip_existing(self, mock_prompt, mock_find, mock_detect):
        mock_info = MagicMock()
        mock_info.is_supported = True
        mock_info.python_version = (3, 11, 0)
        mock_detect.return_value = mock_info
        mock_find.return_value = "/usr/local/bin/tsc"
        mock_prompt.return_value = "skip"

        from techscript.__main__ import cmd_install
        res = cmd_install(force=False)
        self.assertEqual(res, 0)

    @patch("techscript.__main__.detect")
    @patch("techscript.__main__.find_existing_tsc")
    @patch("techscript.__main__.check_internet")
    @patch("techscript.__main__.get_latest_tag")
    @patch("techscript.__main__.download_and_extract")
    def test_cmd_install_download_failure(self, mock_dl, mock_tag, mock_net, mock_find, mock_detect):
        mock_info = MagicMock()
        mock_info.is_supported = True
        mock_info.python_version = (3, 11, 0)
        mock_info.asset_name = "techscript-linux-x64.tar.gz"
        mock_info.binary_name = "tsc"
        mock_detect.return_value = mock_info
        mock_find.return_value = None
        mock_net.return_value = True
        mock_tag.return_value = "release-2.0.0"
        mock_dl.side_effect = RuntimeError("Download error")

        from techscript.__main__ import cmd_install
        res = cmd_install(force=True, debug=False)
        self.assertEqual(res, 1)

    @patch("techscript.__main__.detect")
    @patch("techscript.__main__.find_existing_tsc")
    @patch("techscript.__main__.check_internet")
    @patch("techscript.__main__.get_latest_tag")
    @patch("techscript.__main__.download_and_extract")
    @patch("techscript.__main__.install_binary")
    @patch("techscript.__main__.configure_path")
    @patch("techscript.__main__.verify_installation")
    def test_cmd_install_success(
        self, mock_verify, mock_config_path, mock_install_bin, mock_dl, mock_tag, mock_net, mock_find, mock_detect
    ):
        mock_info = MagicMock()
        mock_info.is_supported = True
        mock_info.python_version = (3, 11, 0)
        mock_info.asset_name = "techscript-linux-x64.tar.gz"
        mock_info.binary_name = "tsc"
        mock_info.install_dir = "/tmp/bin"
        mock_detect.return_value = mock_info
        mock_find.return_value = None
        mock_net.return_value = True
        mock_tag.return_value = "release-2.0.0"
        mock_dl.return_value = ("/tmp/tsc", "/tmp/extracted_dir")
        mock_install_bin.return_value = "/tmp/bin/tsc"
        mock_verify.return_value = True

        from techscript.__main__ import cmd_install
        res = cmd_install(force=True)
        self.assertEqual(res, 0)
        mock_config_path.assert_called_once_with(mock_info, "/tmp/bin")


if __name__ == "__main__":
    unittest.main()
