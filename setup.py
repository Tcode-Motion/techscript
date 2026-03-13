from setuptools import setup, find_packages

with open("README.md", "r", encoding="utf-8") as fh:
    long_description = fh.read()

setup(
    name="techscript-lang",
    version="1.0.4.5",
    author="Tanmoy",
    author_email="tanmoy@example.com",
    description="TechScript — A simple, friendly programming language (.txs)",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/Tcode-Motion/techscript",
    packages=find_packages(),
    include_package_data=True,
    package_data={
        "techscript_wrapper": [
            "examples/*.txs",
        ],
        "techscript": [
            "*.py",
            "*.md"
        ],
    },
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
    ],
    python_requires=">=3.7",
    entry_points={
        "console_scripts": [
            "tech=techscript_wrapper.__main__:main",
            "techscript=techscript_wrapper.__main__:main"
        ],
    },
)
