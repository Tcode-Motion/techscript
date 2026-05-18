#!/usr/bin/env python3
"""Build TechScript icon assets from a source PNG.

Generates:
  - Multi-size PNGs (16, 32, 64, 128, 256, 512)
  - icon.ico  (Windows multi-resolution icon)
  - icon.icns (macOS icon bundle — best-effort without macOS tools)
  - icon.svg  copy from assets/

Usage:
    pip install Pillow
    python scripts/build_icons.py [--source assets/icons/icon-512.png]
"""

from __future__ import annotations

import argparse
import os
import struct
import sys
from pathlib import Path

try:
    from PIL import Image
except ImportError:
    print("ERROR: Pillow is required.  Install with:  pip install Pillow")
    sys.exit(1)

SIZES = [16, 32, 64, 128, 256, 512]

ROOT = Path(__file__).resolve().parent.parent
ICONS_DIR = ROOT / "assets" / "icons"


def generate_pngs(source: Path) -> dict[int, Path]:
    """Resize source image into standard icon sizes."""
    img = Image.open(source).convert("RGBA")
    paths: dict[int, Path] = {}
    for size in SIZES:
        out = ICONS_DIR / f"icon-{size}.png"
        resized = img.resize((size, size), Image.LANCZOS)
        resized.save(out, "PNG")
        paths[size] = out
        print(f"  ✓ {out.name}")
    return paths


def generate_ico(png_paths: dict[int, Path]) -> Path:
    """Create a Windows .ico file containing multiple resolutions."""
    out = ICONS_DIR / "icon.ico"
    base = Image.open(png_paths[512]).convert("RGBA")
    sizes_for_ico = [(16, 16), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)]
    base.save(out, format="ICO", sizes=sizes_for_ico)
    print(f"  ✓ {out.name}")
    return out


def generate_icns_fallback(png_paths: dict[int, Path]) -> Path:
    """Create a minimal .icns file (macOS icon).

    Full ICNS generation normally requires macOS ``iconutil``.  This creates
    a best-effort ICNS with PNG-based icon types that modern macOS reads.
    """
    out = ICONS_DIR / "icon.icns"

    # ICNS PNG-based type codes (macOS 10.7+)
    type_map = {
        16:  b"icp4",   # 16x16
        32:  b"icp5",   # 32x32
        64:  b"icp6",   # 64x64  (non-standard but accepted)
        128: b"ic07",   # 128x128
        256: b"ic08",   # 256x256
        512: b"ic09",   # 512x512
    }

    entries = []
    for size, type_code in type_map.items():
        if size in png_paths:
            png_data = png_paths[size].read_bytes()
            # Each entry: 4-byte type + 4-byte length (includes header) + data
            entry_len = 8 + len(png_data)
            entries.append(type_code + struct.pack(">I", entry_len) + png_data)

    # ICNS header: 'icns' + 4-byte total file length
    body = b"".join(entries)
    total_len = 8 + len(body)
    icns_data = b"icns" + struct.pack(">I", total_len) + body

    out.write_bytes(icns_data)
    print(f"  ✓ {out.name}")
    return out


def copy_svg() -> None:
    """Copy the SVG logo into the icons directory."""
    src = ROOT / "assets" / "logo.svg"
    dst = ICONS_DIR / "icon.svg"
    if src.exists():
        import shutil
        shutil.copy2(src, dst)
        print(f"  ✓ {dst.name} (copied from logo.svg)")
    else:
        print(f"  ⚠ logo.svg not found, skipping SVG copy")


def main() -> None:
    parser = argparse.ArgumentParser(description="Build TechScript icon assets")
    parser.add_argument("--source", type=Path, default=None,
                        help="Source PNG (default: assets/logo.png or assets/icons/icon-512.png)")
    args = parser.parse_args()

    # Find source image
    source = args.source
    if source is None:
        candidates = [
            ICONS_DIR / "icon-512.png",
            ROOT / "assets" / "logo.png",
            ROOT / "assets" / "icon-256.png",
        ]
        for c in candidates:
            if c.exists():
                source = c
                break
    if source is None or not source.exists():
        print("ERROR: No source PNG found. Provide --source <path>")
        sys.exit(1)

    print(f"Source: {source}")
    ICONS_DIR.mkdir(parents=True, exist_ok=True)

    print("\n📐 Generating PNGs...")
    png_paths = generate_pngs(source)

    print("\n🪟 Generating Windows ICO...")
    generate_ico(png_paths)

    print("\n🍎 Generating macOS ICNS...")
    generate_icns_fallback(png_paths)

    print("\n🎨 Copying SVG...")
    copy_svg()

    # Also copy the 256 as the canonical icon.png
    import shutil
    shutil.copy2(png_paths[256], ICONS_DIR / "icon.png")
    print(f"  ✓ icon.png (256×256 canonical)")

    print(f"\n✅ All icons generated in {ICONS_DIR}/")
    print(f"   Files: {', '.join(f.name for f in sorted(ICONS_DIR.iterdir()))}")


if __name__ == "__main__":
    main()
