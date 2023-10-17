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

from base64 import b64decode
from typing import TypedDict


class ClientConstants(TypedDict):
    """A dict of client constants."""

    _IMAGE_UA: str
    """:class:`str`: The user-agent for image requests."""
    _API_UA: str
    """:class:`str`: The user-agent for API requests."""
    OS_VER: str
    """:class:`str`: The OS version."""
    APP_VER: str
    """:class:`str`: The app version."""


_ANDROID_APP = b64decode("Y29tLnNxdWFyZV9lbml4LmFuZHJvaWRfZ29vZ2xlcGxheS5tYW5nYXVwX2dsb2JhbA==").decode("utf-8")
_ANDROID_APP_VER = "45"  # 2.0.0
ANDROID_CONSTANTS: ClientConstants = {
    "_IMAGE_UA": "Dalvik/2.1.0 (Linux; U; Android 12; SM-G935F Build/SQ3A.220705.004)",
    "_API_UA": f"{_ANDROID_APP}/{_ANDROID_APP_VER} (Linux; U; Android 12; en_US; SM-G935F; Build/SQ3A.220705.004; Cronet/114.0.5735.33)",  # noqa: E501
    "OS_VER": "32",  # Android SDK 12
    "APP_VER": _ANDROID_APP_VER,
}
_IOS_APP = b64decode("Y29tLnNxdWFyZS1lbml4Lk1hbmdhVVB3").decode("utf-8")
_IOS_APP_PRE = b64decode("R2xlbndvb2RfUHJvZA==").decode("utf-8")
_IOS_APP_POST = b64decode("QWxhbW9maXJlLzUuNy4x").decode("utf-8")
_IOS_APP_VER = "2.0.1"
_IOS_APP_BUILD = "202307211728"
IOS_CONSTANTS: ClientConstants = {
    "_IMAGE_UA": f"{_IOS_APP_PRE}/{_IOS_APP_BUILD} CFNetwork/1410.0.3 Darwin/22.6.0",
    "_API_UA": f"{_IOS_APP_PRE}/{_IOS_APP_VER} ({_IOS_APP}; build:{_IOS_APP_BUILD}; iOS 16.7.0) {_IOS_APP_POST}",
    "OS_VER": "16.7",
    "APP_VER": _IOS_APP_VER,
}

DEVICE_CONSTANTS: dict[str | int, ClientConstants] = {
    1: ANDROID_CONSTANTS,
    2: IOS_CONSTANTS,
    "android": ANDROID_CONSTANTS,
    "ios": IOS_CONSTANTS,
}

QUALITY_FORMAT = ["middle", "high"]
WEEKLY_CODE = ["mon", "tue", "wed", "thu", "fri", "sat", "sun"]

BASE_HOST = b64decode("Z2xvYmFsLm1hbmdhLXVwLmNvbQ==").decode("utf-8")
API_HOST = b64decode("Z2xvYmFsLWFwaS5tYW5nYS11cC5jb20=").decode("utf-8")
IMAGE_HOST = b64decode("Z2xvYmFsLWltZy5tYW5nYS11cC5jb20=").decode("utf-8")
