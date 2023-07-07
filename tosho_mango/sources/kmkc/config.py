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

import json
from dataclasses import dataclass
from typing import Type, TypedDict
from uuid import uuid4

import betterproto
from requests.cookies import RequestsCookieJar

from tosho_mango.constants import USER_PATH


class KMConfigDeviceType(betterproto.Enum):
    """
    Device type for KM KC session
    """

    MOBILE = 1  # Not implemented yet though
    """Mobile device"""
    WEB = 2
    """Web app"""


@dataclass
class KMConfigBase(betterproto.Message):
    """
    Represents the base config file for KM KC
    """

    id: str = betterproto.string_field(1)
    """The ID for KM KC"""
    type: KMConfigDeviceType = betterproto.enum_field(2)
    """The device type for KM KC"""
    username: str = betterproto.string_field(20)
    """The username for KM KC"""
    email: str = betterproto.string_field(21)
    """The email for KM KC"""
    account_id: int = betterproto.uint32_field(22)
    """The account ID for KM KC"""
    device_id: int = betterproto.uint32_field(23)
    """The device ID for KM KC"""


@dataclass
class KMConfigMobile(KMConfigBase):
    """
    Represents the config file for KM KC mobile
    """

    user_id: str = betterproto.string_field(3)
    """The user ID for KM KC"""
    user_secret: str = betterproto.string_field(4)
    """The user secret for KM KC"""


class _KMConfigWebKVTypes(TypedDict):
    value: str
    expires: str


@dataclass
class KMConfigWebKV(betterproto.Message):
    """
    The key-value cookies pair for KM KC web
    """

    value: str = betterproto.string_field(1)
    """The value of the cookie"""
    expires: int = betterproto.uint64_field(2)
    """The expiry of the cookie"""

    @classmethod
    def from_cookie_dict(cls: Type["KMConfigWebKV"], cookie_dict: _KMConfigWebKVTypes):
        return cls(value=cookie_dict["value"], expires=int(cookie_dict["expires"]))


@dataclass
class KMConfigWeb(KMConfigBase):
    """
    Represents the config file for KM KC web
    """

    uwt: str = betterproto.string_field(3)
    """:class:`str`: The auth token for KM KC"""
    birthday: KMConfigWebKV = betterproto.message_field(4)
    """:class:`KMConfigWebKV`: Account birthday information"""
    tos_adult: KMConfigWebKV = betterproto.message_field(5)
    """:class:`KMConfigWebKV`: Account adult information"""
    privacy: KMConfigWebKV = betterproto.message_field(6)
    """:class:`KMConfigWebKV`: Account privacy policy information"""

    @classmethod
    def from_cookies(cls: Type["KMConfigWeb"], cookies: RequestsCookieJar):
        uwt_cookie = cookies.get("uwt")
        assert uwt_cookie is not None, "`uwt` cookie is not found"
        birthday_cookie = json.loads(cookies.get("birthday"))
        tos_adult_cookie = json.loads(cookies.get("terms_of_service_adult"))
        privacy_cookie = json.loads(cookies.get("privacy_policy"))

        return cls(
            id=str(uuid4()),
            type=KMConfigDeviceType.WEB,
            username="",
            email="temp@kmkc.xyz",
            account_id=0,
            device_id=0,
            uwt=uwt_cookie,
            birthday=KMConfigWebKV.from_cookie_dict(birthday_cookie),
            tos_adult=KMConfigWebKV.from_cookie_dict(tos_adult_cookie),
            privacy=KMConfigWebKV.from_cookie_dict(privacy_cookie),
        )

    def apply_cookies(self, cookies: RequestsCookieJar):
        uwt_cookie = cookies.get("uwt")
        if uwt_cookie is not None:
            self.uwt = uwt_cookie
        bdy_cookie = cookies.get("birthday")
        if bdy_cookie is not None:
            self.birthday = KMConfigWebKV.from_cookie_dict(json.loads(bdy_cookie))
        tos_adult_cookie = cookies.get("terms_of_service_adult")
        if tos_adult_cookie is not None:
            self.tos_adult = KMConfigWebKV.from_cookie_dict(json.loads(tos_adult_cookie))
        privacy_cookie = cookies.get("privacy_policy")
        if privacy_cookie is not None:
            self.privacy = KMConfigWebKV.from_cookie_dict(json.loads(privacy_cookie))


def get_config(hex_mode: str) -> KMConfigWeb | KMConfigMobile | None:
    USER_PATH.mkdir(parents=True, exist_ok=True)

    CONFIG_PATH = USER_PATH / f"kmkc.{hex_mode}.tmconf"

    if not CONFIG_PATH.exists():
        return None

    conf_bita = CONFIG_PATH.read_bytes()
    conf_temp = KMConfigBase.FromString(conf_bita)
    if conf_temp.type is KMConfigDeviceType.WEB:
        return KMConfigWeb.FromString(conf_bita)
    elif conf_temp.type is KMConfigDeviceType.MOBILE:
        return KMConfigMobile.FromString(conf_bita)
    raise ValueError(f"Unknown device type {conf_temp.type}")


def get_all_config() -> list[KMConfigWeb | KMConfigMobile]:
    USER_PATH.mkdir(parents=True, exist_ok=True)

    CONFIG_GLOB = USER_PATH.glob("kmkc.*.tmconf")
    parsed_conf: list[KMConfigWeb | KMConfigMobile] = []
    for conf in CONFIG_GLOB:
        conf_bita = conf.read_bytes()
        conf_temp = KMConfigBase.FromString(conf_bita)
        if conf_temp.type is KMConfigDeviceType.WEB:
            parsed_conf.append(KMConfigWeb.FromString(conf_bita))
        elif conf_temp.type is KMConfigDeviceType.MOBILE:
            parsed_conf.append(KMConfigMobile.FromString(conf_bita))
        raise ValueError(f"Unknown device type {conf_temp.type}")
    return parsed_conf


def save_config(config: KMConfigWeb | KMConfigMobile):
    USER_PATH.mkdir(parents=True, exist_ok=True)

    CONFIG_PATH = USER_PATH / f"musq.{config.id}.tmconf"

    CONFIG_PATH.write_bytes(bytes(config))
