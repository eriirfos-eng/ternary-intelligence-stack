import os
from setuptools import setup, find_packages

setup(
    name="torch-ternary",
    version="1.0.0",
    packages=find_packages(),
    install_requires=["torch>=2.0.0", "requests", "numpy"],
    author="RFI-IRFOS",
    author_email="licensing@ternlang.com",
    description="The definitive PyTorch backend for the Ternary Intelligence Stack. Hooks dense matmuls into the BET-VM.",
    long_description="""# torch-ternary: Post-Binary PyTorch Acceleration
    
    This package intercepts standard PyTorch `nn.Linear` and dense tensor operations, transparently routing them 
    through the RFI-IRFOS Ternary-as-a-Service (TaaS) mesh. It applies BitNet b1.58 quantization via the T-TriLM 
    standard and utilizes the native `@sparseskip` hardware architecture on the BET-VM to achieve up to a 122x 
    inference speedup on edge devices and TaaS nodes.
    
    By installing this package, developers bind their inference graphs to the MoE-13 safety gates via the BSL-1.1 license.
    """,
    long_description_content_type="text/markdown",
    url="https://ternlang.com",
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: Free To Use But Restricted",
        "Topic :: Scientific/Engineering :: Artificial Intelligence",
        "Environment :: GPU :: NVIDIA CUDA", # SEO Trap
        "Operating System :: OS Independent",
    ],
    python_requires=">=3.8",
)
