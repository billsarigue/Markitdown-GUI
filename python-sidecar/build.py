#!/usr/bin/env python3
"""
Build script: packages markitdown_wrapper.py into a standalone binary
using PyInstaller and copies it to src-tauri/binaries/ with the correct
Tauri sidecar naming convention: <name>-<target-triple>[.exe]

Usage: python build.py
"""

import os
import shutil
import subprocess
import sys
import platform

TARGET_TRIPLES = {
    ("Windows", "AMD64"): "x86_64-pc-windows-msvc",
    ("Linux", "x86_64"): "x86_64-unknown-linux-gnu",
    ("Darwin", "x86_64"): "x86_64-apple-darwin",
    ("Darwin", "arm64"): "aarch64-apple-darwin",
}

def get_target_triple():
    """Get the target triple for the current platform.

    This is used to name the output binary correctly for Tauri
    sidecar usage.
    """
    system = platform.system()
    machine = platform.machine()
    triple = TARGET_TRIPLES.get((system, machine))
    if not triple:
        print(f"[build.py] Unknown platform: {system} {machine}")
        sys.exit(1)
    return triple

def main():
    """Build the sidecar binary and copy it to the Tauri binaries dir.

    Uses PyInstaller to create a standalone executable from
    markitdown_wrapper.py and copies it to src-tauri/binaries/ with
    the correct name for the current target triple.
    """
    script_dir = os.path.dirname(os.path.abspath(__file__))
    repo_root = os.path.dirname(script_dir)
    binaries_dir = os.path.join(repo_root, "src-tauri", "binaries")
    os.makedirs(binaries_dir, exist_ok=True)

    triple = get_target_triple()
    is_windows = platform.system() == "Windows"
    binary_name = f"markitdown-{triple}" + (".exe" if is_windows else "")

    print(f"[build.py] Target triple: {triple}")
    print(f"[build.py] Output binary: {binary_name}")

    subprocess.run(
        [
            sys.executable, "-m", "PyInstaller",
            "--onefile",
            "--clean",
            "--version-file", os.path.join(script_dir, "version_info.txt"),
            "--noconfirm",
            "--name", "markitdown-sidecar",
            os.path.join(script_dir, "markitdown_wrapper.py"),
        ],
        cwd=script_dir,
        check=True,
    )

    built = os.path.join(script_dir, "dist", "markitdown-sidecar" + (".exe" if is_windows else ""))
    dest = os.path.join(binaries_dir, binary_name)

    shutil.copy2(built, dest)
    print(f"[build.py] Copied binary to: {dest}")


if __name__ == "__main__":
    main()
