#!/usr/bin/env python3
"""TechScript setup.py — legacy setuptools installer.

Prefer `pip install .` or `pip install -e .` which uses pyproject.toml.
This file exists for environments that still require setup.py.
"""

from setuptools import setup, find_packages
from pathlib import Path

long_description = ""
readme = Path("README.md")
if readme.exists():
    long_description = readme.read_text(encoding="utf-8")

setup(
    name="techscript",
    version="1.0.1",
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
