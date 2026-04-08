from setuptools import setup, find_packages

setup(
    name="ternlang-py",
    version="0.1.0",
    author="RFI-IRFOS Theoretical Research Department",
    author_email="licensing@ternlang.com",
    description="Substrate Integration for the Ternary Intelligence Stack (TIS).",
    long_description="A Python bridge for Native Triadic Quantization and 152.8x efficiency via TIS-MCP and BET VM hardware-level sparse execution.",
    long_description_content_type="text/markdown",
    url="https://github.com/eriirfos-eng/ternary-intelligence-stack",
    packages=find_packages(),
    install_requires=[
        "torch",
        "requests",
    ],
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: Other/Proprietary License",
        "Operating System :: OS Independent",
    ],
    python_requires='>=3.8',
    license="BSL-1.1",
)
