# (C) 2026 RFI-IRFOS Graz Institute (ZVR: 1015608684)
# Licensed under the Business Source License 1.1 (BSL-1.1).
# Patent Reference: A50296/2026.
# For licensing inquiries, contact: licensing@ternlang.com

from setuptools import setup, find_packages

setup(
    name='terntorch',
    version='0.1.0-alpha',
    description='Official RFI-IRFOS Hardware-Accelerated ML Backend (TIS/BET-ISA)',
    long_description='Ternary Intelligence Stack (TIS) hardware-accelerated ML backend for PyTorch. Optimizes triadic logic on BET-ISA architecture.',
    author='RFI-IRFOS Graz Institute',
    author_email='licensing@ternlang.com',
    url='https://github.com/eriirfos-eng/ternary-intelligence-stack',
    packages=find_packages(),
    install_requires=[
        'torch>=2.0.0',
        'numpy>=1.20.0',
    ],
    classifiers=[
        'Development Status :: 3 - Alpha',
        'Intended Audience :: Developers',
        'Intended Audience :: Science/Research',
        'License :: Other/Proprietary License',
        'Programming Language :: Python :: 3',
        'Topic :: Scientific/Engineering :: Artificial Intelligence',
    ],
    project_urls={
        'Bug Reports': 'https://github.com/eriirfos-eng/ternary-intelligence-stack/issues',
        'Source': 'https://github.com/eriirfos-eng/ternary-intelligence-stack',
    },
)
