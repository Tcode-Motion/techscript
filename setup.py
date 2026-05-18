#!/usr/bin/env python3
"""TechScript setup.py — legacy setuptools installer.

Prefer `pip install .` or `pip install -e .` which uses pyproject.toml.
This file exists for environments that still require setup.py.
"""

import shutil
import os
from setuptools import setup, find_packages
from pathlib import Path

long_description = ""
readme = Path("README.md")
if readme.exists():
    long_description = readme.read_text(encoding="utf-8")

# Copy the compiled native standalone binary to bundle it into the wheel
package_dir = Path("src/techscript")
package_dir.mkdir(parents=True, exist_ok=True)

# Copy tech.exe
for possible_exe in [
    Path("runtime/target/x86_64-pc-windows-msvc/release/tech.exe"),
    Path("runtime/target/release/tech.exe"),
    Path("runtime/target/debug/tech.exe"),
]:
    if possible_exe.exists():
        shutil.copy2(possible_exe, package_dir / "tech.exe")
        print(f"Bundled native binary from: {possible_exe}")
        break

# Copy tech_studio.exe
for possible_studio in [
    Path("runtime/target/x86_64-pc-windows-msvc/release/tech_studio.exe"),
    Path("runtime/target/release/tech_studio.exe"),
    Path("runtime/target/debug/tech_studio.exe"),
]:
    if possible_studio.exists():
        shutil.copy2(possible_studio, package_dir / "tech_studio.exe")
        print(f"Bundled native studio binary from: {possible_studio}")
        break

setup(
    name="techscript",
    version="1.0.6",
    description="TechScript — A simple, friendly programming language (.txs)",
    long_description=long_description,
    long_description_content_type="text/markdown",
    author="TechScript Team",
    license="MIT",
    python_requires=">=3.10",
    packages=find_packages(where="src"),
    package_dir={"": "src"},
    package_data={
        "techscript": [
            "tech.exe",
            "tech_studio.exe",
            "../assets/icons/*.png",
            "../assets/icons/*.ico",
            "../assets/icons/*.icns",
            "../assets/icons/*.svg",
        ]
    },
    entry_points={
        "console_scripts": [
            "tech=techscript.cli:main",
        ],
    },
    classifiers=[
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Programming Language :: Python :: 3.12",
        "Programming Language :: Python :: 3.13",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
        "Topic :: Software Development :: Interpreters",
        "Topic :: Software Development :: Compilers",
        "Intended Audience :: Education",
        "Intended Audience :: Developers",
    ],
    extras_require={
        "dev": ["pytest>=7.0", "pytest-cov", "black", "flake8"],
        "icons": ["Pillow>=9.0"],
    },
)
