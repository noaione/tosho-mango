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
from enum import Enum
from typing import cast

from tosho_mango.utils import get_dt_now

__all__ = (
    "Quality",
    "WeeklyCode",
    "ConsumeCoin",
)


class WeeklyCode(str, Enum):
    MONDAY = "mon"
    """Monday"""
    TUESDAY = "tue"
    """Tuesday"""
    WEDNESDAY = "wed"
    """Wednesday"""
    THURSDAY = "thu"
    """Thursday"""
    FRIDAY = "fri"
    """Friday"""
    SATURDAY = "sat"
    """Saturday"""
    SUNDAY = "sun"
    """Sunday"""

    @classmethod
    def today(cls) -> "WeeklyCode":
        now = get_dt_now(9)
        weekday = now.weekday()
        mem_maps = cls._member_map_
        select = list(mem_maps.keys())[weekday]
        return cast(WeeklyCode, mem_maps[select])

    @property
    def indexed(self) -> int:
        return list(self.__class__).index(self)


class Quality(str, Enum):
    NORMAL = "middle"
    HIGH = "high"


@dataclass
class ConsumeCoin:
    free: int = 0
    event: int = 0
    paid: int = 0
    need: int = 0

    def is_possible(self) -> bool:
        """
        Check if you can get this chapter with your current coins.

        This is just a simple check to the :attr:`.need` attribute.

        Returns
        -------
        :class:`bool`
            ``True`` if you can get this chapter, ``False`` otherwise.
        """
        # Check if possible to get this chapter
        return self.free + self.event + self.paid >= self.need

    @property
    def is_free(self) -> bool:
        """
        Check if this chapter is free.

        Returns
        -------
        :class:`bool`
            ``True`` if this chapter is free, ``False`` otherwise.
        """
        return self.need == 0
