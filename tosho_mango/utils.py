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

import inspect
from datetime import datetime, timedelta, timezone
from enum import Enum
from typing import Any, Callable, TypeAlias, TypeVar

from typing_extensions import ParamSpec

__all__ = (
    "get_dt_now",
    "get_date_from_unix",
    "format_date",
    "copy_doc",
    "peek_enum_docstring",
)

T = TypeVar("T")
P = ParamSpec("P")
WrappedFuncDeco: TypeAlias = Callable[[Callable[P, T]], Callable[P, T]]


def get_dt_now(tz_shift: int = 0):
    """Get current :class:`datetime.datetime` with timezone shift.

    Parameters
    ----------
    tz_shift: :class:`int`
        The timezone shift in hours, default to 0 (UTC).

    Returns
    -------
    :class:`datetime.datetime`
        The current datetime with timezone shift.
    """
    tz = timezone(timedelta(hours=tz_shift))
    return datetime.now(tz)


def get_date_from_unix(unix_time: int | float, tz_shift: int = 0):
    """Get :class:`datetime.datetime` from unix timestamp with timezone shift.

    Parameters
    ----------
    unix_time: :class:`int` | :class:`float`
        The unix timestamp.
    tz_shift: :class:`int`
        The timezone shift in hours, default to 0 (UTC).

    Returns
    -------
    :class:`datetime.datetime`
        The datetime from unix timestamp with timezone shift.
    """
    tz = timezone(timedelta(hours=tz_shift))
    return datetime.fromtimestamp(unix_time, timezone.utc).replace(tzinfo=tz)


def format_date(dt: datetime, fmt: str = "%Y-%m-%d %H:%M:%S"):
    """Format :class:`datetime.datetime` to string.

    Parameters
    ----------
    dt: :class:`datetime.datetime`
        The datetime to format.
    fmt: :class:`str`
        The string format that will be used, by default "%Y-%m-%d %H:%M:%S"

    Returns
    -------
    :class:`str`
        The formatted datetime string.
    """
    return dt.strftime(fmt)


def copy_doc(copy_func: Callable[..., Any]) -> WrappedFuncDeco[P, T]:
    """Copies the doc string of the given function to another.

    This function is intended to be used as a decorator.

    Usage:
    ```py
        def foo():
            '''This is a foo doc string'''
            ...

        @copy_doc(foo)
        def bar():
            ...
    ```
    """

    def wrapped(func: Callable[P, T]) -> Callable[P, T]:
        func.__doc__ = copy_func.__doc__
        return func

    return wrapped


def peek_enum_docstring(enum_value: Enum) -> str | None:
    """Get the docstring of an enum value.

    This is a hacky way to get the docstring of an enum value, might break on some edge cases.

    Parameters
    ----------
    enum_value: :class:`Enum`
        The enum value to get the docstring from.

    Returns
    -------
    :class:`str` | :class:`None`
        The docstring of the enum value, or None if it doesn't have any.
    """

    src_lines, _ = inspect.getsourcelines(enum_value.__class__)
    end_at = [idx for idx, src in enumerate(src_lines) if src.strip().startswith("@") or src.strip().startswith("def")]
    if not end_at:
        end_at = len(src_lines)
    else:
        end_at = end_at[0]
    src_lines = src_lines[:end_at]
    collected_lines = []
    start_collect = False
    multiline = False
    for src in src_lines:
        src_s = src.strip()
        if src_s.startswith(enum_value.name) and not start_collect:
            start_collect = True
            collected_lines.append(src_s)
            continue
        if start_collect and not multiline and not src_s.startswith('"'):
            break

        if start_collect:
            if not multiline and src_s == '"""':
                multiline = True
                collected_lines.append(src_s)
                continue
            elif not multiline and src_s.startswith('"') and src_s.endswith('"'):
                collected_lines.append(src_s)

            if multiline and src_s == '"""':
                multiline = False
                collected_lines.append(src_s)
            elif multiline:
                collected_lines.append(src_s)
    recollected_lines = [r for r in [r.strip('"""') for r in collected_lines[1:]] if r]  # noqa: B005
    if not recollected_lines:
        return None
    return "\n".join(recollected_lines)
