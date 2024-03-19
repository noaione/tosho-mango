import argparse
import json
import subprocess as sp
import time
from pathlib import Path
from typing import Any, List, Tuple

import requests
import toml

ROOT_DIR = Path(__file__).absolute().parent.parent

parser = argparse.ArgumentParser(description="Publish all crates in the workspace")
parser.add_argument(
    "--dry-run",
    action="store_true",
    help="Only print the crates that will be published",
)
args = parser.parse_args()


def get_crate_index_path(crate_name: str) -> str:
    # https://doc.rust-lang.org/cargo/reference/registry-index.html#index-files
    if len(crate_name) == 1:
        return f"/1/{crate_name}"
    elif len(crate_name) == 2:
        return f"/2/{crate_name}"
    elif len(crate_name) == 3:
        return f"/3/{crate_name[0]}/{crate_name}"
    else:
        first_two = crate_name[:2]
        second_two = crate_name[2:4]
        return f"/{first_two}/{second_two}/{crate_name}"


def request_crate_index(crate_path: str) -> List[Any]:
    req = requests.get(f"https://index.crates.io{crate_path}")

    if req.status_code == 404:
        return []

    # Return json lines
    text_data = req.text.split("\n")
    return [json.loads(line) for line in text_data if line]


def get_all_crate_members(cargo_toml: Path) -> List[str]:
    read_data = toml.load(cargo_toml)

    return read_data.get("workspace", {}).get("members", [])


ALL_CRATES = get_all_crate_members(ROOT_DIR / "Cargo.toml")

print("Found:", ALL_CRATES)

PUBLISH_CRATE: List[Tuple[str, str]] = []
for crate in ALL_CRATES:
    crate_crate_toml = ROOT_DIR / crate / "Cargo.toml"

    crate_toml = toml.load(crate_crate_toml)
    package_name = crate_toml["package"]["name"]
    package_version = crate_toml["package"]["version"]

    print("Fetching crate index for:", package_name)
    crate_path = get_crate_index_path(package_name)
    crate_index = request_crate_index(crate_path)

    all_published = [x["vers"] for x in crate_index]

    if package_version in all_published:
        print(f" Skipping {package_name} as {package_version} is already published")
    else:
        print(f" Adding {package_name} to publish list")
        PUBLISH_CRATE.append(package_name)

# "tosho" should be last
if "tosho" in PUBLISH_CRATE:
    PUBLISH_CRATE.remove("tosho")
    PUBLISH_CRATE.append("tosho")
# macros should be first
if "tosho-macros" in PUBLISH_CRATE:
    PUBLISH_CRATE.remove("tosho-macros")
    PUBLISH_CRATE.insert(0, "tosho-macros")

print("Publishing:", PUBLISH_CRATE)

# Every 2 crates, we sleep for 3 minutes
if not args.dry_run:
    for i, crate in enumerate(PUBLISH_CRATE):
        print(f"Publishing {crate} ({i + 1}/{len(PUBLISH_CRATE)})")
        if i % 2 == 0 and i != 0:
            print(" Waiting 3 minutes...")
            time.sleep(180)

        sp.run(f"cargo publish -p {crate}", shell=True)
