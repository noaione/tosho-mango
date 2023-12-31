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

__all__ = (
    "KMAPIError",
    "KMNotEnoughPointError",
)


class KMAPIError(Exception):
    """A base error for KM KC API."""

    def __init__(self, error_code: int, message: str) -> None:
        """An error occured when using KM KC API.

        Parameters
        ----------
        error_code: :class:`int`
            The error code.
        message: :class:`str`
            The error message.
        """
        self.error_code = error_code
        self.message = message

        super().__init__(f"An error occured with status {error_code}: {message}")


class KMNotEnoughPointError(Exception):
    """An error when you don't have enough point to buy a manga."""

    pass
