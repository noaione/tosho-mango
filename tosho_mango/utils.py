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

from datetime import datetime, timedelta, timezone

__all__ = (
    "get_dt_now",
    "get_date_from_unix",
    "format_date",
)


def get_dt_now(tz_shift: int = 0):
    tz = timezone(timedelta(hours=tz_shift))
    return datetime.now(tz)


def get_date_from_unix(unix_time: int | float, tz_shift: int = 0):
    tz = timezone(timedelta(hours=tz_shift))
    return datetime.fromtimestamp(unix_time, timezone.utc).replace(tzinfo=tz)


def format_date(dt: datetime, fmt: str = "%Y-%m-%d %H:%M:%S"):
    return dt.strftime(fmt)
