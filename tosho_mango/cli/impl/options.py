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
from typing import Any

import click
from click.core import Parameter

__all__ = (
    "COMMA_SEPARATED_NUMBER",
    "COMMA_SEPARATED_STR",
    "account_id",
    "title_id",
    "output_dir",
)


class CommaSeparatedBase(click.ParamType):
    name = "comma_separated"

    def __init__(self, expect_type: type) -> None:
        super().__init__()
        self.expect_type = expect_type
        self._etype_name = expect_type.__name__

    def convert(self, value, param, ctx):
        if not isinstance(value, str):
            self.fail(f"{value!r} must be a string", param, ctx)

        split_vals: list[str] = [val.strip() for val in value.split(",") if val.strip()]

        converted: list[Any] = []
        for split in split_vals:
            try:
                if isinstance(split, self.expect_type):
                    converted.append(split)
                    continue
                converted.append(self.expect_type(split))
            except Exception:
                self.fail(f"{split!r} is not a valid {self._etype_name}", param, ctx)
            converted.append(int(split))

        return converted

    def get_metavar(self, param: Parameter) -> str | None:
        up = self._etype_name.upper()
        if param.required and param.param_type_name == "argument":
            return "{{INT,INT,INT,...}}".replace("INT,", f"{up},")
        return "[INT,INT,INT,...]".replace("INT,", f"{up},")


COMMA_SEPARATED_NUMBER = CommaSeparatedBase(int)
COMMA_SEPARATED_STR = CommaSeparatedBase(str)


account_id = click.option(
    "-a",
    "--account",
    "account_id",
    type=str,
    default=None,
    help="Account ID to use",
    required=False,
    metavar="ACCOUNT_ID",
)
title_id = click.argument("title_id", type=int, metavar="TITLE_ID", required=True)
output_dir = click.option(
    "-o",
    "--output",
    "output_dir",
    help="Output directory for downloaded files",
    type=click.Path(file_okay=False, dir_okay=True, writable=True, resolve_path=True, path_type=Path),
    required=True,
    default=Path.cwd() / "DOWNLOADS",
)
