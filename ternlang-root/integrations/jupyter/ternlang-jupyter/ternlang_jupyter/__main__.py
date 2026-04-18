"""Entry point: python -m ternlang_jupyter -f <connection_file>"""
from ipykernel.kernelapp import IPKernelApp
from .kernel import TernlangKernel

IPKernelApp.launch_instance(kernel_class=TernlangKernel)
