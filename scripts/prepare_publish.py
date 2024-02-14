import subprocess
from pathlib import Path

TARGET_CRATES = [
    "tosho",
    "tosho_amap",
    "tosho_musq",
    "tosho_kmkc",
    "tosho_sjv",
]

ROOT_DIR = Path(__file__).absolute().parent.parent

# Change every tosho-* related crates to the current version for publishing

workspace_toml = ROOT_DIR / "Cargo.toml"
workspace_content = workspace_toml.read_text()

package_version_idx = workspace_content.find(".package]\nversion = ")
package_version = workspace_content[package_version_idx + 21:].split("\n")[0][:-1].strip()
print(f"Current version: {package_version}")

for CRATE in TARGET_CRATES:
    crate_toml = ROOT_DIR / CRATE / "Cargo.toml"
    crate_content = crate_toml.read_text()

    for line in crate_content.split("\n"):
        if line.startswith("tosho-") and "path" in line:
            print(f"Updating {CRATE} to version {package_version}")
            new_line = line.replace("path = ", f'version = "{package_version}", path = ')
            crate_content = crate_content.replace(line, new_line)
    print(f"Writing to {crate_toml}")
    crate_toml.write_text(crate_content)

# Commit the changes temporarily so that we can publish the crates

# Set username and email
subprocess.run(["git", "config", "user.name", "noaione"], cwd=ROOT_DIR)
subprocess.run(["git", "config", "user.email", "noaione@n4o.xyz"], cwd=ROOT_DIR)

# Add all crate changes
for CRATE in TARGET_CRATES:
    subprocess.run(["git", "add", f"{CRATE}/Cargo.toml"], cwd=ROOT_DIR)
# Commit root and lock
subprocess.run(["git", "add", "Cargo.toml"], cwd=ROOT_DIR)
subprocess.run(["git", "add", "Cargo.lock"], cwd=ROOT_DIR)

# Commit the changes
subprocess.run(["git", "commit", "-m", f"chore: prepare publish {package_version}"], cwd=ROOT_DIR)
