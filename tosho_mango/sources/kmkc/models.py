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

from dataclasses import dataclass

__all__ = (
    "RankingTab",
    "RankingTabs",
)


@dataclass
class RankingTab:
    """The ranking tab for KM."""

    id: int
    """:class:`int`: The ID of the tab."""
    name: str
    """:class:`str`: The name of the tab."""
    tab: str
    """:class:`str`: The tab name when used as :class:`click.Choice`."""


RankingTabs = [
    RankingTab(3, "Action", "action"),
    RankingTab(4, "Sports", "sports"),
    RankingTab(5, "Romance", "romance"),
    RankingTab(6, "Isekai", "isekai"),
    RankingTab(7, "Suspense", "romance"),
    RankingTab(8, "Outlaws", "outlaws"),
    RankingTab(9, "Drama", "drama"),
    RankingTab(10, "Fantasy", "fantasy"),
    RankingTab(11, "Slice of Life", "sol"),
    RankingTab(12, "All", "all"),
    RankingTab(13, "Today's Specials", "specials"),
]
