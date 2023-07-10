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

from __future__ import annotations

from msgspec import Struct

__all__ = (
    "ChapterDetailDump",
    "MangaDetailDump",
)


class ChapterDetailDump(Struct, rename="camel", omit_defaults=True):
    """A dump info of a chapter."""

    id: int
    """:class:`int`: The chapter ID."""
    main_name: str
    """:class:`str`: The main chapter name."""
    timestamp: int | None = None
    """:class:`int | None`: The timestamp of the chapter."""
    sub_name: str | None = None
    """:class:`str | None`: The sub chapter name, if any."""


class MangaDetailDump(Struct, rename="camel"):
    """A dump info of a manga."""

    title_name: str
    """The title name of the manga."""
    author_name: str
    """The author name of the manga."""
    chapters: list[ChapterDetailDump]
    """The list of chapters of the manga."""
