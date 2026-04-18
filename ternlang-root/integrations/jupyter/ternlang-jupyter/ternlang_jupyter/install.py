"""
Install the Ternlang Jupyter kernel spec into the current Jupyter environment.

Usage:
    python -m ternlang_jupyter.install
    python -m ternlang_jupyter.install --sys-prefix   # inside virtualenv/conda
    python -m ternlang_jupyter.install --system       # system-wide
"""

import argparse
import json
import os
import shutil
import sys
import tempfile


KERNEL_NAME = "ternlang"
DISPLAY_NAME = "Ternlang (BET VM)"


def _kernelspec_dir() -> str:
    return os.path.join(os.path.dirname(__file__), "..", "kernelspec")


def install(user: bool = True, sys_prefix: bool = False, system: bool = False):
    from jupyter_client.kernelspec import KernelSpecManager

    src = os.path.abspath(_kernelspec_dir())

    with tempfile.TemporaryDirectory() as td:
        dest = os.path.join(td, KERNEL_NAME)
        shutil.copytree(src, dest)

        # Rewrite kernel.json to point at this Python interpreter
        spec_path = os.path.join(dest, "kernel.json")
        with open(spec_path) as f:
            spec = json.load(f)
        spec["argv"][0] = sys.executable
        with open(spec_path, "w") as f:
            json.dump(spec, f, indent=2)

        km = KernelSpecManager()
        install_dir = km.install_kernel_spec(
            dest,
            kernel_name=KERNEL_NAME,
            user=user and not system,
            replace=True,
            prefix=sys.prefix if sys_prefix else None,
        )

    print(f"Installed Ternlang kernel spec → {install_dir}")
    print(f"Kernel name: {KERNEL_NAME}")
    print(f"Display name: {DISPLAY_NAME}")
    print()
    print("Start Jupyter and select 'Ternlang (BET VM)' from the kernel menu.")


def main():
    parser = argparse.ArgumentParser(
        description="Install the Ternlang Jupyter kernel."
    )
    parser.add_argument("--user",       action="store_true", default=True,
                        help="Install for current user (default)")
    parser.add_argument("--sys-prefix", action="store_true",
                        help="Install into sys.prefix (virtualenv/conda)")
    parser.add_argument("--system",     action="store_true",
                        help="Install system-wide (requires sudo)")
    args = parser.parse_args()
    install(user=args.user, sys_prefix=args.sys_prefix, system=args.system)


if __name__ == "__main__":
    main()
