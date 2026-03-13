from setuptools import setup, find_packages

with open("README.md", "r", encoding="utf-8") as fh:
    long_description = fh.read()

setup(
    name="techscript-lang",
    version="1.0.4.3",
    author="Tanmoy",
    author_email="tanmoy@example.com",
    description="A friendly native programming language that reads like plain English.",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/Tcode-Motion/techscript",
    packages=["techscript_wrapper"],
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
    ],
    python_requires=">=3.7",
    package_data={
        "techscript_wrapper": ["examples/*.txs"],
    },
    include_package_data=True,
    entry_points={
        "console_scripts": [
            "tech=techscript_wrapper.__main__:main",
            "techscript=techscript_wrapper.__main__:main"
        ],
    },
)
