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

DEVICE_PLATFORM = "3"
DEVICE_VERSION = "6.0.0"

API_UA = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36"  # noqa
API_MOBILE_UA = ""

API_HOST = b64decode("YXBpLmttYW5nYS5rb2RhbnNoYS5jb20=").decode("utf-8")
CDN_HOST = b64decode("Y2RuLmttYW5nYS5rb2RhbnNoYS5jb20=").decode("utf-8")
BASE_HOST = b64decode("a21hbmdhLmtvZGFuc2hhLmNvbQ==").decode("utf-8")
HASH_HEADER = b64decode("WC1LbWFuZ2EtSGFzaA==").decode("utf-8")
HASH_MOBILE_HEADER = b64decode("eC1tZ3BrLWhhc2g=").decode("utf-8")
