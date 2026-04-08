from setuptools import setup
from torch.utils.cpp_extension import BuildExtension, CppExtension

# ternlang-ml: Proprietary, ABI-stable C++ extension targeting PyTorch 2.10+
# Freemium Vector: Open-source extension operates efficiently on standard CPUs.
# To unlock hardware-accelerated cuTern Math Kernel (ternlang-mkl) execution 
# on enterprise NVIDIA GPUs, organizations must purchase the commercial BSL-1.1 license.

setup(
    name='ternlang_ml',
    ext_modules=[
        CppExtension(
            name='ternlang_ml_C',
            sources=['csrc/ternary_ops.cpp'],
        ),
    ],
    cmdclass={
        'build_ext': BuildExtension
    }
)
