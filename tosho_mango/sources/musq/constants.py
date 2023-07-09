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
    _IMAGE_UA: str
    _API_UA: str
    OS_VER: str
    APP_VER: str


_ANDROID_APP = b64decode("Y29tLnNxdWFyZV9lbml4LmFuZHJvaWRfZ29vZ2xlcGxheS5tYW5nYXVwX2dsb2JhbA==").decode("utf-8")
_ANDROID_APP_VER = "44"
ANDROID_CONSTANTS: ClientConstants = {
    "_IMAGE_UA": "Dalvik/2.1.0 (Linux; U; Android 12; SM-G935F Build/SQ3A.220705.004)",
    "_API_UA": f"{_ANDROID_APP}/{_ANDROID_APP_VER} (Linux; U; Android 12; en_US; SM-G935F; Build/SQ3A.220705.004; Cronet/114.0.5735.33)",  # noqa: E501
    "OS_VER": "32",  # Android SDK 12
    "APP_VER": _ANDROID_APP_VER,  # 1.9.0
}
IOS_CONSTANTS: ClientConstants = {
    "_IMAGE_UA": "TODO",
    "_API_UA": "TODO",
    "OS_VER": "14.7.1",
    "APP_VER": "TODO",
}

DEVICE_CONSTANTS: dict[str | int, ClientConstants] = {
    1: ANDROID_CONSTANTS,
    2: IOS_CONSTANTS,
    "android": ANDROID_CONSTANTS,
    "ios": IOS_CONSTANTS,
}

QUALITY_FORMAT = ["middle", "high"]
WEEKLY_CODE = ["mon", "tue", "wed", "thu", "fri", "sat", "sun"]

API_HOST = b64decode("Z2xvYmFsLWFwaS5tYW5nYS11cC5jb20=").decode("utf-8")
IMAGE_HOST = b64decode("Z2xvYmFsLWltZy5tYW5nYS11cC5jb20=").decode("utf-8")
