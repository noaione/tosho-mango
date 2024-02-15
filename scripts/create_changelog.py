import os
from pathlib import Path

ROOT_DIR = Path(__file__).parent.parent.absolute()

CHANGELOG_FILE = ROOT_DIR / "CHANGELOG.md"
INNER_DESC = """The following release notes are automatically generated.

For the complete changelog, visit [here](https://github.com/noaione/tosho-mango/blob/master/CHANGELOG.md).
If you encounter any problems, please report them on the [issues](https://github.com/noaione/tosho-mango/issues/new/choose) page.

### Updating

Since [v0.3.1](https://github.com/noaione/tosho-mango/releases/tag/v0.3.1), you can update `tosho` using the following command:

```bash
$ tosho update
```

Which will automatically download the latest version of `tosho` and replace the old one.

## Changelog
"""  # noqa: E501

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
        line = INNER_DESC
        START = True

    if START:
        EXTRACTED_CHANGELOG += line + "\n"

EXTRACTED_CHANGELOG = EXTRACTED_CHANGELOG.strip()

# Write into CHANGELOG-GENERATED.md
if not EXTRACTED_CHANGELOG:
    EXTRACTED_CHANGELOG = f"{INNER_DESC}\n\nNo changelog found for version {VERSION}"

CHANGELOG_GENERATED_FILE = ROOT_DIR / "CHANGELOG-GENERATED.md"
CHANGELOG_GENERATED_FILE.write_text(EXTRACTED_CHANGELOG)
