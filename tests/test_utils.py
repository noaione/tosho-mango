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

from datetime import datetime, timedelta, timezone
from enum import Enum
from textwrap import dedent

from tosho_mango.sources.kmkc.dto import MagazineCategory
from tosho_mango.utils import copy_doc, format_date, get_date_from_unix, get_dt_now, peek_enum_docstring

JST_TZ = timezone(timedelta(hours=9))


def test_get_date_from_unix():
    dt = get_date_from_unix(1672531200)
    assert dt == datetime(2023, 1, 1, 0, 0, 0, 0, tzinfo=timezone.utc)


def test_get_date_from_unix_shift_jst():
    dt = get_date_from_unix(1672531200, 9)
    assert dt == datetime(2023, 1, 1, 0, 0, 0, 0, tzinfo=JST_TZ)


def test_get_dt_now():
    dt = get_dt_now()
    assert dt.tzinfo == timezone.utc


def test_get_dt_now_shift_jst():
    dt = get_dt_now(9)
    assert dt.tzinfo == JST_TZ


def test_format_date():
    dt = datetime(2023, 1, 1, 0, 0, 0, 0, tzinfo=timezone.utc)
    assert format_date(dt) == "2023-01-01 00:00:00"


def _function_a():
    """This is from function A."""
    pass


@copy_doc(_function_a)
def _function_b():
    pass


def test_copy_doc():
    assert _function_b.__doc__ == _function_a.__doc__


class _EnumTest(Enum):
    """This is from Enum C."""

    A = 4

    def __str__(self) -> str:
        return super().__str__()


class _EnumTest2(Enum):
    """This is from Enum D."""

    A = 4
    """A is 4"""


def test_peek_enum_docstring():
    ichijinsha_doc = dedent(
        """
    Ichijinsha, a subsidiary of Kodansha
    Owns the following:
    - Febri
    - Comic Rex (4-koma)
    - Monthly Comic Zero Sum (Josei focused)
    - Comic Yuri Hime (Girls Love)
    - gateau
    - IDOLM@STER Million Live Magazine Plus+
    """
    ).strip()
    assert peek_enum_docstring(_EnumTest.A) is None
    assert peek_enum_docstring(_EnumTest2.A) == "A is 4"
    assert peek_enum_docstring(MagazineCategory.Original) == "KM Original series"
    assert peek_enum_docstring(MagazineCategory.MonthlyShounenMagazine) == "Monthly Shounen Magazine"
    assert (
        peek_enum_docstring(MagazineCategory.ShounenMagazineR)
        == "Shounen Magazine R, a supplement magazine for WSM. (Discontinued)\nOriginally a bi-monthly magazine, "
        "but now it's a monthly digital-only magazine."
    )
    assert peek_enum_docstring(MagazineCategory.BekkanGetsumaga) == "Bekkan Getsumaga"
    assert peek_enum_docstring(MagazineCategory.WeeklyYoungMagazine) == "Weekly Young Magazine, Seinen focused magazine"
    assert (
        peek_enum_docstring(MagazineCategory.BabyMofu)
        == "Baby Mofu, a website focused on books and manga related to childcare"
    )
    assert peek_enum_docstring(MagazineCategory.Ichijinsha) == ichijinsha_doc
    assert peek_enum_docstring(MagazineCategory.ComicCreate) == "Comic Create, a web comic magazine"
