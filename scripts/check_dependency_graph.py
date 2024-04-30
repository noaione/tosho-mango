from pathlib import Path

import toml

CURRENT_DIR = Path(__file__).parent.absolute()
ROOT_DIR = CURRENT_DIR.parent

cargo_lock = toml.load(ROOT_DIR / "Cargo.lock")

duplicate_deps = {}
merged_deps = {}
for package in cargo_lock["package"]:
    name = package["name"]
    version = package["version"]
    if name not in merged_deps:
        merged_deps[name] = version
    elif merged_deps[name] != version:
        if name not in duplicate_deps:
            duplicate_deps[name] = [(name, version, merged_deps[name])]
        else:
            duplicate_deps[name].append((name, version, merged_deps[name]))


if duplicate_deps:
    print("Duplicate dependencies found:")
    for name, versions in duplicate_deps.items():
        main_ver = merged_deps[name]
        print(f"  {name} {main_ver}:")
        for _, version, _ in versions:
            print(f"    - {version}")
else:
    print("No duplicate dependencies found.")
