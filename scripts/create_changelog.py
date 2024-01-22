import os
from pathlib import Path

ROOT_DIR = Path(__file__).parent.parent.absolute()

CHANGELOG_FILE = ROOT_DIR / "CHANGELOG.md"
INNER_DESC = """The following are an automatically generated release notes based on the git tags.

You can find the full changelog [here](https://github.com/noaione/tosho-mango/blob/master/CHANGELOG.md)
Please report any problem that you've encountered on the [issues](https://github.com/noaione/tosho-mango/issues) page."""  # noqa: E501

# ref/tags/v1.0.0
GIT_TAGS = os.getenv("VERSION")
if not GIT_TAGS:
    raise ValueError("No git tags found")

# v1.0.0
if not GIT_TAGS.startswith("refs/tags/"):
    raise ValueError("Invalid git tag format")

VERSION = GIT_TAGS.split("/")[-1]

if VERSION.startswith("v"):
    VERSION = VERSION[1:]

EXTRACTED_CHANGELOG = ""
START = False
for line in CHANGELOG_FILE.read_text().splitlines():
    if line.startswith("## [") and START:
        break
    if line.startswith(f"## [{VERSION}]"):
        line = line.replace(f"[{VERSION}] ", f"v{VERSION} — ") + "\n" + INNER_DESC + "\n"
        START = True

    if START:
        EXTRACTED_CHANGELOG += line + "\n"

EXTRACTED_CHANGELOG = EXTRACTED_CHANGELOG.strip()

# Write into CHANGELOG-GENERATED.md
if not EXTRACTED_CHANGELOG:
    EXTRACTED_CHANGELOG = "## Unreleased\n\nNo changelog for this release"

CHANGELOG_GENERATED_FILE = ROOT_DIR / "CHANGELOG-GENERATED.md"
CHANGELOG_GENERATED_FILE.write_text(EXTRACTED_CHANGELOG)
