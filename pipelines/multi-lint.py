"""
MIT License

Copyright (c) 2023-present noaione

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"""

import subprocess as sp
import sys
from pathlib import Path
from typing import Optional

to_be_linted = ["tosho_mango", "tests"]


def check_license_header(file: Path) -> bool:
    # Find the MIT License header on file that's not __init__.py
    with file.open("r") as fp:
        # Check until line 10
        for idx, line in enumerate(fp):
            if idx == 10:
                break
            if file.name == "__init__.py":
                if line.startswith(":copyright:") or line.startswith(":license:"):
                    return True
                return True
            else:
                if line.startswith("MIT License"):
                    return True
    return False


def missing_init_check(folder: Path):
    filenames = [file.name for file in folder.iterdir()]
    if len(filenames) < 1:
        # Ignore empty folder
        return False
    return "__init__.py" not in filenames


current_path = Path(__file__).absolute().parent.parent  # root dir
venv_dir = [
    current_path / ".venv",
    current_path / "venv",
    current_path / "env",
]
selected_venv_dir: Optional[Path] = None
for venv in venv_dir:
    if venv.exists():
        selected_venv_dir = venv

if selected_venv_dir is None:
    raise RuntimeError("No virtual environment found")


script_path = selected_venv_dir / "Scripts" if sys.platform == "win32" else selected_venv_dir / "bin"

print(f"[*] Running tests at {current_path}")

print("[*] Running isort test...")
isort_res = sp.Popen([script_path / "isort", "-c"] + to_be_linted).wait()
print("[*] Running ruff test...")
ruff_res = sp.Popen([script_path / "ruff", "check", "--statistics", "--show-fixes"] + to_be_linted).wait()

results = [(isort_res, "isort"), (ruff_res, "ruff")]
any_error = False

for res in results:
    if res[0] != 0:
        print(f"[-] {res[1]} returned an non-zero code")
        any_error = True
    else:
        print(f"[+] {res[1]} passed")


print("[*] Running license check test...")
any_license_error = False
folder_to_check: list[Path] = []
for folder in to_be_linted:
    if folder.endswith(".py"):
        files = [current_path / folder]
    else:
        files = (current_path / folder).glob("**/*.py")
    for file in files:
        parent = file.parent
        if parent not in folder_to_check:
            folder_to_check.append(parent)
        if not check_license_header(file):
            print(f"[?] {file} is missing license header")
            any_license_error = True
            any_error = True

print("[*] Running missing __init__.py check...")
any_missing_init_error = False
for folder in folder_to_check:
    if missing_init_check(folder):
        print(f"[?] {folder} is missing __init__.py")
        any_missing_init_error = True
        any_error = True

if any_license_error:
    print("[-] Please add the license header on the files above")
else:
    print("[+] License header check passed")
if any_missing_init_error:
    print("[-] Please add __init__.py on the folders above")
else:
    print("[+] Missing __init__.py check passed")

if any_error or any_license_error or any_missing_init_error:
    print("[-] Test finished, but some tests failed")
    exit(1)
print("[+] All tests passed")
exit(0)
