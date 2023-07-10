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

# A folder merger for split chapter

from __future__ import annotations

import mimetypes
import re
from pathlib import Path
from typing import Any, cast

import click
import msgspec

from tosho_mango import models, term
from tosho_mango.cli.base import ToshoMangoCommandHandler

__all__ = ("tools_cli_split_merge",)
console = term.get_console()

_TITLE_RE = re.compile(r"(?:[\w]+ |#|[\w]+)(?P<base>0?[\d]+)?(?:[.\-( ][\-.\(]?)?(?P<split>[\d]+)?")


def safe_int(value: Any) -> int | None:
    try:
        return int(value)
    except ValueError:
        return None


def int_or_float(value: Any) -> int | float | None:
    if "." in value:
        try:
            return float(value)
        except ValueError:
            pass
    return safe_int(value)


class _PseudoMatch:
    """Simulate a Regex match object."""

    def __init__(self):
        self._contents: dict[str, str] = {}

    def set(self, key: str, value: str):
        self._contents[key] = value

    def get(self, key: str):
        return self._contents.get(key)

    def group(self, key: str | int):
        if isinstance(key, int):
            try:
                actual = list(self._contents.keys())[key]
            except IndexError:
                return None
            return self.get(actual)
        return self.get(key)


def _inquire_chapter_number(chapter: models.ChapterDetailDump, last_known_num: int) -> _PseudoMatch:
    console.warning(f"  Failed to parse chapter title: {chapter.main_name!r}")

    ch_number = console.inquire(
        f"  Chapter number (last known: Chapter {last_known_num})", lambda y: int_or_float(y) is not None
    )
    ch_number = str(cast(int | float, int_or_float(ch_number)))
    matching = _PseudoMatch()
    if "." in ch_number:
        base, floaty = ch_number.split(".")
        matching.set("base", base)
        matching.set("split", floaty)
    else:
        matching.set("base", ch_number)
    return matching


def _collect_chapters(chapters_dump: list[models.ChapterDetailDump]) -> dict[str, list[models.ChapterDetailDump]]:
    chapters_dump.sort(key=lambda x: x.id)
    chapters_mappings: dict[str, list[models.ChapterDetailDump]] = {}
    last_known_num = 0
    extra = 0
    for chapter in chapters_dump:
        if (match := _TITLE_RE.match(chapter.main_name)) is None:
            match = _inquire_chapter_number(chapter, last_known_num)

        base = match.group("base")
        split = match.group("split")

        if base is None and split is None:
            _temp = _inquire_chapter_number(chapter, last_known_num)
            base = _temp.group("base")
            split = _temp.group("split")

        if base is None:
            last_known_num += 1
            base = str(last_known_num)

        base = int(base)
        use_extra = False
        if last_known_num > base:
            console.warning(
                f"  Chapter {base} is lower than last known chapter {last_known_num}, assuming extra chapter"
            )
            base = last_known_num
            use_extra = True
        else:
            last_known_num = base

        if use_extra:
            chapters_mappings.setdefault(f"ex{base:03d}.{extra}", []).append(chapter)
            extra += 1
        else:
            chapters_mappings.setdefault(f"c{base:03d}", []).append(chapter)
    return chapters_mappings


def _is_all_folder_exist(base_dir: Path, all_dirs: list[models.ChapterDetailDump]):
    for path in all_dirs:
        tp = base_dir / str(path.id)
        if not tp.exists():
            return False
    return True


def _get_last_page(target_dir: Path):
    last_page = 0
    for file in target_dir.iterdir():
        if file.is_file() and file.stem.startswith("p"):
            last_page = max(last_page, int(file.stem[1:]) + 1)
    return last_page


def _is_image(file: Path):
    mimetype, _ = mimetypes.guess_type(file.name)
    if mimetype and mimetype.startswith("image/"):
        return True
    suf = file.suffix.lower()
    # Some modern image format
    return suf in [".avif", ".jxl", ".webp", ".heif", ".heic"]


@click.command(
    name="automerge",
    help="Automatically merge chapters based on _info.json.",
    cls=ToshoMangoCommandHandler,
)
@click.argument(
    "folder",
    type=click.Path(exists=True, dir_okay=True, file_okay=False, resolve_path=True, path_type=Path),
    metavar="PATH",
    required=True,
)
@click.option(
    "-sl",
    "--skip-last",
    "skip_last",
    is_flag=True,
    help="Skip the last chapter merge, useful since sometimes last chapter as incomplete split chapter.",
)
@click.option(
    "-v",
    "--verbose",
    "verbose_mode",
    is_flag=True,
    help="Show verbose output.",
)
def tools_cli_split_merge(folder: Path, skip_last: bool = False, verbose_mode: bool = False):
    info_json = folder / "_info.json"
    if not info_json.exists():
        console.error(f"Folder {folder!r} doesn't have _info.json which contains information dumps!")
        return

    try:
        info = msgspec.json.decode(info_json.read_bytes(), type=models.MangaDetailDump)
    except Exception as exc:
        console.error(f"Failed to load _info.json: {exc}")
        return

    console.info(f"Loaded {len(info.chapters)} chapters from _info.json, collecting...")

    chapters_maps = _collect_chapters(info.chapters)
    console.info(f"Collected {len(chapters_maps)} chapters")
    for key, chapters in chapters_maps.items():
        console.info(f"  {key}: has {len(chapters)} chapters")
        if verbose_mode:
            for chapter in chapters:
                console.info(f"   - {chapter.main_name}")

    total_values = sum(len(x) for x in chapters_maps.values())
    is_continue = console.confirm(f"Found {total_values} chapters, continue?")
    if not is_continue:
        console.warning("Aborting...")
        return

    if skip_last:
        console.warning("Skipping last chapter merge...")
        chapters_maps.popitem()

    console.info("Starting merge...")
    for ch_name, all_chapters in chapters_maps.items():
        console.info(f"  Merging {ch_name}...")
        if not _is_all_folder_exist(folder, all_chapters):
            console.warning(f"   Not all folders exist for {ch_name}, skipping...")
            continue

        target_dir = folder / ch_name
        target_dir.mkdir(exist_ok=True)
        last_page = _get_last_page(target_dir)
        for chapter in all_chapters:
            source_dir = folder / str(chapter.id)
            if not source_dir.exists():
                continue

            for file in source_dir.iterdir():
                if file.is_file() and _is_image(file):
                    try:
                        file.rename(target_dir / f"p{last_page:03d}{file.suffix}")
                    except Exception as exc:
                        console.warning(f"   Failed to move {file.name}: {exc}")
                    last_page += 1
        console.info(f"   Merged {chapter.main_name} with {last_page} images")
