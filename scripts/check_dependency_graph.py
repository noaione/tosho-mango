from pathlib import Path

import toml

CURRENT_DIR = Path(__file__).parent.absolute()
ROOT_DIR = CURRENT_DIR.parent

cargo_lock = toml.load(ROOT_DIR / "Cargo.lock")


def walk_up_deps(package: str, version: str):
    deps_name = f"{package} {version}"
    package_rows = []
    for package in cargo_lock["package"]:
        if "dependencies" not in package:
            continue
        if deps_name in package["dependencies"]:
            package_rows.append(f"{package['name']} {package['version']}")
            package_rows.extend(walk_up_deps(package["name"], package["version"]))
        elif package in package["dependencies"]:
            package_rows.append(f"{package['name']} {package['version']}")
            package_rows.extend(walk_up_deps(package["name"], package["version"]))
    return package_rows


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
        print(f"  {name}:")
        # package_tree = walk_up_deps(name, main_ver)
        # package_tree_txt = " -> ".join(package_tree)
        # print(f"    - {main_ver} ({package_tree_txt})")
        print(f"    - {main_ver}")
        for _, version, _ in versions:
            # pkg_else_tree = walk_up_deps(name, version)
            # pkg_else_tree_txt = " -> ".join(pkg_else_tree)
            # print(f"    - {version} ({pkg_else_tree_txt})")
            print(f"    - {version}")
else:
    print("No duplicate dependencies found.")
