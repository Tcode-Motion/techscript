"""Tests for techscript.__main__ CLI module."""

import os
import sys
import unittest
from unittest.mock import patch, MagicMock

sys.path.insert(0, os.path.join(os.path.dirname(__file__), ".."))

from techscript.__main__ import (
    cmd_install,
    _detect_and_verify_platform,
    _handle_existing_install,
    _download_package,
    _install_and_configure,
    _verify_and_print_result,
    main,
)
from techscript.platform_detect import PlatformInfo, OS, Arch


class TestMainCLI(unittest.TestCase):

    @patch("techscript.__main__.detect")
    def test_detect_and_verify_platform_supported(self, mock_detect):
        mock_info = PlatformInfo(OS.LINUX, Arch.X64, False, None, (3, 11, 0))
        mock_detect.return_value = mock_info
        info = _detect_and_verify_platform()
        self.assertEqual(info, mock_info)

    @patch("techscript.__main__.detect")
    def test_detect_and_verify_platform_unsupported(self, mock_detect):
        mock_info = PlatformInfo(OS.UNKNOWN, Arch.UNKNOWN, False, None, (3, 11, 0))
        mock_detect.return_value = mock_info
        info = _detect_and_verify_platform()
        self.assertIsNone(info)

    @patch("techscript.__main__.find_existing_tsc")
    def test_handle_existing_install_skip(self, mock_find):
        mock_find.return_value = "/usr/local/bin/tsc"
        mock_info = PlatformInfo(OS.LINUX, Arch.X64, False, None, (3, 11, 0))
        with patch("techscript.__main__.prompt_existing_install", return_value="skip"):
            res = _handle_existing_install(mock_info, force=False)
            self.assertEqual(res, 0)

    @patch("techscript.__main__.find_existing_tsc")
    def test_handle_existing_install_force(self, mock_find):
        mock_find.return_value = "/usr/local/bin/tsc"
        mock_info = PlatformInfo(OS.LINUX, Arch.X64, False, None, (3, 11, 0))
        res = _handle_existing_install(mock_info, force=True)
        self.assertIsNone(res)

    @patch("techscript.__main__.check_internet", return_value=False)
    def test_download_package_no_internet(self, mock_check):
        mock_info = PlatformInfo(OS.LINUX, Arch.X64, False, None, (3, 11, 0))
        bin_path, tmp_dir = _download_package(mock_info, debug=False)
        self.assertIsNone(bin_path)
        self.assertIsNone(tmp_dir)

    @patch("techscript.__main__._detect_and_verify_platform")
    @patch("techscript.__main__._handle_existing_install")
    @patch("techscript.__main__._download_package")
    @patch("techscript.__main__._install_and_configure")
    @patch("techscript.__main__._verify_and_print_result")
    def test_cmd_install_success(
        self, mock_verify, mock_install, mock_download, mock_existing, mock_detect
    ):
        mock_info = PlatformInfo(OS.LINUX, Arch.X64, False, None, (3, 11, 0))
        mock_detect.return_value = mock_info
        mock_existing.return_value = None
        mock_download.return_value = ("/tmp/tsc", "/tmp/dir")
        mock_install.return_value = "/usr/local/bin/tsc"
        mock_verify.return_value = 0

        exit_code = cmd_install(force=False, debug=False)
        self.assertEqual(exit_code, 0)

    def test_main_help(self):
        with patch("sys.stdout"):
            code = main(["--help"])
            self.assertEqual(code, 0)


if __name__ == "__main__":
    unittest.main()
