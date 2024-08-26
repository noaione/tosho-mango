from pathlib import Path

import toml

CURRENT_DIR = Path(__file__).parent.absolute()
ROOT_DIR = CURRENT_DIR.parent

cargo_lock = toml.load(ROOT_DIR / "Cargo.lock")
cargo_toml = toml.load(ROOT_DIR / "Cargo.toml")

# Get members
members = cargo_toml["workspace"]["members"]
members = [member.replace("_", "-") for member in members]
print(members)


def walk_through_deps(base_deps: list[str]) -> list[str]:
    deps = []
    for dep in base_deps:
        splitted = dep.split(" ")
        # If have 2 elements, that means it's a specific package version
        version = None
        if len(splitted) == 2:
            name, version = splitted
        else:
            name = dep
        for package in cargo_lock["package"]:
            if "dependencies" not in package:
                continue
            if package["name"] == name and (
                version is None or package["version"] == version
            ):
                # Matched
                deps += walk_through_deps(package["dependencies"])
                if version is not None:
                    deps.append(f"{name} {version}")
                else:
                    deps.append(name)
    # Make sure to remove duplicates
    return list(set(deps))


for package in cargo_lock["package"]:
    name = package["name"]

    if name not in members:
        continue
    version = package["version"]

    print(f"{name} v{version}:")
    pkg_tree = package["dependencies"]
    pkg_tree = walk_through_deps(pkg_tree)
    print(f"  {len(pkg_tree)} dependencies")
