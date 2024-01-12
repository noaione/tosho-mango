# Simple wrapper to cargo-tarpaulin

import subprocess as sp

IGNORED_PACKAGES = [
    "tosho",
]

IGNORED_PATHS = [
    "tosho_*/src/lib.rs",
    "tosho_*/src/constants.rs",
]

# fmt: off
BUILD_COMMAND = [
    "cargo",
    "tarpaulin",
    "--verbose",
    "--all-features",
    "--workspace",
    "--out", "xml",
    "--count",
    "-l",
]
# fmt: on

for package in IGNORED_PACKAGES:
    BUILD_COMMAND.append("-e")
    BUILD_COMMAND.append(package)
for path in IGNORED_PATHS:
    BUILD_COMMAND.append("--exclude-files")
    BUILD_COMMAND.append(path)

# Run the command
sp.run(BUILD_COMMAND)
