"""
TechScript downloader module.
Downloads the correct binary archive from GitHub Releases.
"""

import hashlib
import os
import shutil
import sys
import tempfile
from typing import Optional
from urllib.request import urlopen, Request
from urllib.error import URLError, HTTPError

GITHUB_REPO = "Tcode-Motion/techscript"
RELEASES_API = f"https://api.github.com/repos/{GITHUB_REPO}/releases/latest"
RELEASES_BASE = f"https://github.com/{GITHUB_REPO}/releases/download"
FALLBACK_TAG = "release-2.0.0"

# Try to use requests if available; otherwise fall back to urllib
try:
    import requests as _requests
    _HAS_REQUESTS = True
except ImportError:
    _HAS_REQUESTS = False


def _get(url: str, stream: bool = False):
    """Unified GET that works with or without the requests library."""
    if _HAS_REQUESTS:
        resp = _requests.get(url, stream=stream, timeout=30, headers={"User-Agent": "techscript-installer/2.0"})
        resp.raise_for_status()
        return resp
    # Fallback to urllib
    req = Request(url, headers={"User-Agent": "techscript-installer/2.0"})
    return urlopen(req, timeout=30)


def check_internet() -> bool:
    """Return True if a basic network connection is available."""
    try:
        urlopen("https://github.com", timeout=5)
        return True
    except Exception:
        return False


def get_latest_tag() -> str:
    """Fetch the latest release tag from the GitHub API."""
    try:
        if _HAS_REQUESTS:
            resp = _requests.get(RELEASES_API, timeout=10, headers={"User-Agent": "techscript-installer/2.0"})
            resp.raise_for_status()
            tag = resp.json().get("tag_name", "")
        else:
            import json
            req = Request(RELEASES_API, headers={"User-Agent": "techscript-installer/2.0"})
            with urlopen(req, timeout=10) as r:
                tag = json.loads(r.read()).get("tag_name", "")

        # Skip any v1.x tags
        if tag and not tag.startswith("v1."):
            return tag
    except Exception:
        pass
    return FALLBACK_TAG


def build_download_url(tag: str, asset_name: str) -> str:
    return f"{RELEASES_BASE}/{tag}/{asset_name}"


def download_file(url: str, dest_path: str, show_progress: bool = True) -> None:
    """Download a file from url to dest_path with optional progress bar."""
    try:
        if _HAS_REQUESTS:
            resp = _requests.get(url, stream=True, timeout=60,
                                 headers={"User-Agent": "techscript-installer/2.0"})
            resp.raise_for_status()
            total = int(resp.headers.get("content-length", 0))
            downloaded = 0
            with open(dest_path, "wb") as f:
                for chunk in resp.iter_content(chunk_size=65536):
                    f.write(chunk)
                    downloaded += len(chunk)
                    if show_progress and total:
                        pct = int(downloaded * 50 / total)
                        bar = "█" * pct + "░" * (50 - pct)
                        mb = downloaded / 1_048_576
                        total_mb = total / 1_048_576
                        sys.stdout.write(f"\r  [{bar}] {mb:.1f}/{total_mb:.1f} MB")
                        sys.stdout.flush()
            if show_progress:
                print()
        else:
            req = Request(url, headers={"User-Agent": "techscript-installer/2.0"})
            with urlopen(req, timeout=60) as resp, open(dest_path, "wb") as f:
                shutil.copyfileobj(resp, f)
    except HTTPError as e:
        raise RuntimeError(f"Download failed (HTTP {e.code}): {url}") from e
    except URLError as e:
        raise RuntimeError(f"Network error: {e.reason}") from e


def extract_archive(archive_path: str, dest_dir: str) -> None:
    """Extract .tar.gz or .zip archive to dest_dir."""
    if archive_path.endswith(".tar.gz") or archive_path.endswith(".tgz"):
        import tarfile
        with tarfile.open(archive_path, "r:gz") as tf:
            tf.extractall(dest_dir)
    elif archive_path.endswith(".zip"):
        import zipfile
        with zipfile.ZipFile(archive_path, "r") as zf:
            zf.extractall(dest_dir)
    else:
        raise RuntimeError(f"Unknown archive format: {archive_path}")


def find_binary_in_dir(directory: str, binary_name: str) -> Optional[str]:
    """Recursively find the tsc binary inside an extracted directory."""
    for root, _dirs, files in os.walk(directory):
        for fname in files:
            if fname == binary_name:
                return os.path.join(root, fname)
    return None


def download_and_extract(tag: str, asset_name: str, binary_name: str,
                         show_progress: bool = True) -> str:
    """
    Download the release archive and extract the binary.
    Returns the path to the extracted binary.
    """
    url = build_download_url(tag, asset_name)
    tmp_dir = tempfile.mkdtemp(prefix="techscript_install_")
    archive_path = os.path.join(tmp_dir, asset_name)

    try:
        print(f"  Downloading: {url}")
        download_file(url, archive_path, show_progress=show_progress)

        extract_dir = os.path.join(tmp_dir, "extracted")
        os.makedirs(extract_dir, exist_ok=True)
        extract_archive(archive_path, extract_dir)

        binary_path = find_binary_in_dir(extract_dir, binary_name)
        if not binary_path:
            raise RuntimeError(
                f"Binary '{binary_name}' not found inside archive '{asset_name}'."
            )
        return binary_path, tmp_dir  # caller must clean up tmp_dir
    except Exception:
        shutil.rmtree(tmp_dir, ignore_errors=True)
        raise
