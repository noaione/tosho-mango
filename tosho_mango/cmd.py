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

from pathlib import Path

import click

from ._metadata import __appname__, __author__, __version__
from .cli.manager import common_tools, kmkc_source, musq_source

CONTEXT_SETTINGS = dict(help_option_names=["-h", "--help"])
WORKING_DIR = Path.cwd().absolute()

__all__ = ("main",)


@click.group(context_settings=CONTEXT_SETTINGS)
@click.version_option(
    __version__,
    "--version",
    "-V",
    prog_name=__appname__,
    message="%(prog)s v%(version)s - Created by {}".format(__author__),
)
@click.pass_context
def main(ctx: click.Context):
    """Tosho or tosho-mango is a CLI tool to download manga from various sources."""
    pass


main.add_command(kmkc_source)
main.add_command(musq_source)
main.add_command(common_tools)
