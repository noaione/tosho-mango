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

import click

from tosho_mango.cli.impl import kmkc, musq, tools
from tosho_mango.cli.importer import auto_import_implementations

__all__ = (
    "musq_source",
    "kmkc_source",
    "common_tools",
)


@click.group(name="mu", help="Download manga from MU!")
def musq_source():
    """Download manga from MU!"""
    pass


@click.group(name="km", help="Download manga from KM")
def kmkc_source():
    """Download manga from KM."""
    pass


@click.group(name="tools", help="Extra tools to help manage the downloaded manga")
def common_tools():
    """Extra tools to help manage the downloaded manga."""
    pass


auto_import_implementations(musq_source, musq)
auto_import_implementations(kmkc_source, kmkc)
auto_import_implementations(common_tools, tools)
