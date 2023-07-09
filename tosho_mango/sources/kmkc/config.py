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
from typing import Type
from urllib.parse import unquote
from uuid import uuid4

import betterproto
from msgspec import Struct, json
from requests.cookies import RequestsCookieJar

from tosho_mango.constants import USER_PATH


class KMConfigDeviceType(betterproto.Enum):
    """Device type for KM KC session"""

    MOBILE = 1  # Not implemented yet though
    """Mobile device"""
    WEB = 2
    """Web app"""


@dataclass
class _KMConfigBase(betterproto.Message):
    """Represents a simple basic KM KC config"""

    id: str = betterproto.string_field(1)
    """:class:`str`: The ID for KM KC"""
    type: KMConfigDeviceType = betterproto.enum_field(2)
    """:class:`KMConfigDeviceType`: The device type for KM KC"""


@dataclass
class KMConfigMobile(betterproto.Message):
    """Represents the config file for KM KC mobile"""

    id: str = betterproto.string_field(1)
    """:class:`str`: The ID for KM KC"""
    type: KMConfigDeviceType = betterproto.enum_field(2)
    """:class:`KMConfigDeviceType`: The device type for KM KC"""
    username: str = betterproto.string_field(3)
    """:class:`str`: The username for KM KC"""
    email: str = betterproto.string_field(4)
    """:class:`str`: The email for KM KC"""
    account_id: int = betterproto.uint32_field(5)
    """:class:`int`: The account ID for KM KC"""
    device_id: int = betterproto.uint32_field(6)
    """:class:`int`: The device ID for KM KC"""
    user_id: str = betterproto.string_field(100)
    """:class:`str`: The user ID for KM KC"""
    user_secret: str = betterproto.string_field(101)
    """:class:`str`: The user secret for KM KC"""


class _KMConfigWebKVTypes(Struct):
    value: str
    expires: str


@dataclass
class KMConfigWebKV(betterproto.Message):
    """The key-value cookies pair for KM KC web"""

    value: str = betterproto.string_field(1)
    """:class:`str`: The value of the cookie"""
    expires: int = betterproto.uint64_field(2)
    """:class:`int`: The expiry of the cookie"""

    @classmethod
    def from_cookie_dict(cls: Type["KMConfigWebKV"], cookie_dict: _KMConfigWebKVTypes) -> "KMConfigWebKV":
        """Get the config from the cookie dict

        Parameters
        ----------
        cookie_dict: :class:`_KMConfigWebKVTypes`
            The parsed cookie dict

        Returns
        -------
        KMConfigWebKV
            The config from the cookie dict
        """
        return cls(value=str(cookie_dict.value), expires=int(cookie_dict.expires))


@dataclass
class KMConfigWeb(betterproto.Message):
    """Represents the config file for KM KC web"""

    id: str = betterproto.string_field(1)
    """:class:`str`: The ID for KM KC"""
    type: KMConfigDeviceType = betterproto.enum_field(2)
    """:class:`KMConfigDeviceType`: The device type for KM KC"""
    username: str = betterproto.string_field(3)
    """:class:`str`: The username for KM KC"""
    email: str = betterproto.string_field(4)
    """:class:`str`: The email for KM KC"""
    account_id: int = betterproto.uint32_field(5)
    """:class:`int`: The account ID for KM KC"""
    device_id: int = betterproto.uint32_field(6)
    """:class:`int`: The device ID for KM KC"""
    uwt: str = betterproto.string_field(100)
    """:class:`str`: The auth token for KM KC"""
    birthday: KMConfigWebKV = betterproto.message_field(101)
    """:class:`KMConfigWebKV`: Account birthday information"""
    tos_adult: KMConfigWebKV = betterproto.message_field(102)
    """:class:`KMConfigWebKV`: Account adult information"""
    privacy: KMConfigWebKV = betterproto.message_field(103)
    """:class:`KMConfigWebKV`: Account privacy policy information"""

    @classmethod
    def from_cookies(cls: Type["KMConfigWeb"], cookies: RequestsCookieJar) -> "KMConfigWeb":
        """Get the config from the cookies responses

        Parameters
        ----------
        cookies: :class:`requests.cookies.RequestsCookieJar`
            The cookies responses

        Returns
        -------
        KMConfigWeb
            The parsed configuration

        Raises
        ------
        ValueError
            If the `uwt` cookie is missing
        """
        uwt_cookie = cookies.get("uwt")
        if uwt_cookie is None:
            raise ValueError("`uwt` cookie is not found")
        birthday_cookie = json.decode(unquote(cookies.get("birthday")), type=_KMConfigWebKVTypes)
        tos_adult_cookie = json.decode(unquote(cookies.get("terms_of_service_adult")), type=_KMConfigWebKVTypes)
        privacy_cookie = json.decode(unquote(cookies.get("privacy_policy")), type=_KMConfigWebKVTypes)

        return cls(
            id=str(uuid4()),
            type=KMConfigDeviceType.WEB,
            username="",
            email="temp@kmkc.xyz",
            account_id=0,
            device_id=0,
            uwt=unquote(uwt_cookie),
            birthday=KMConfigWebKV.from_cookie_dict(birthday_cookie),
            tos_adult=KMConfigWebKV.from_cookie_dict(tos_adult_cookie),
            privacy=KMConfigWebKV.from_cookie_dict(privacy_cookie),
        )

    def apply_cookies(self, cookies: RequestsCookieJar):
        """Apply new cookies data to the config

        Parameters
        ----------
        cookies: :class:`requests.cookies.RequestsCookieJar`
            The cookies responses
        """
        uwt_cookie = cookies.get("uwt")
        if uwt_cookie is not None:
            self.uwt = unquote(uwt_cookie)
        bdy_cookie = cookies.get("birthday")
        if bdy_cookie is not None:
            self.birthday = KMConfigWebKV.from_cookie_dict(json.decode(unquote(bdy_cookie), type=_KMConfigWebKVTypes))
        tos_adult_cookie = cookies.get("terms_of_service_adult")
        if tos_adult_cookie is not None:
            self.tos_adult = KMConfigWebKV.from_cookie_dict(
                json.decode(unquote(tos_adult_cookie), type=_KMConfigWebKVTypes),
            )
        privacy_cookie = cookies.get("privacy_policy")
        if privacy_cookie is not None:
            self.privacy = KMConfigWebKV.from_cookie_dict(
                json.decode(unquote(privacy_cookie), type=_KMConfigWebKVTypes),
            )


def get_config(hex_mode: str) -> KMConfigWeb | KMConfigMobile | None:
    """Get a single config from the account ID.

    Parameters
    ----------
    account_id: :class:`str`
        The account ID to be fetched.

    Returns
    -------
    :class:`KMConfigWeb` | :class:`KMConfigMobile` | None
        The config if found, else None.
    """
    USER_PATH.mkdir(parents=True, exist_ok=True)

    CONFIG_PATH = USER_PATH / f"kmkc.{hex_mode}.tmconf"

    if not CONFIG_PATH.exists():
        return None

    conf_bita = CONFIG_PATH.read_bytes()
    conf_temp = _KMConfigBase.FromString(conf_bita)
    conf_type = KMConfigDeviceType(conf_temp.type)
    if conf_type is KMConfigDeviceType.WEB:
        return KMConfigWeb.FromString(conf_bita)
    elif conf_type is KMConfigDeviceType.MOBILE:
        return KMConfigMobile.FromString(conf_bita)
    raise ValueError(f"Unknown device type {conf_type!r}")


def get_all_config() -> list[KMConfigWeb | KMConfigMobile]:
    """Get all config from the user path.

    Returns
    -------
    :class:`list[KMConfigWeb | KMConfigMobile]`
        The list of config.
    """
    USER_PATH.mkdir(parents=True, exist_ok=True)

    CONFIG_GLOB = USER_PATH.glob("kmkc.*.tmconf")
    parsed_conf: list[KMConfigWeb | KMConfigMobile] = []
    for conf in CONFIG_GLOB:
        conf_bita = conf.read_bytes()
        conf_temp = _KMConfigBase.FromString(conf_bita)
        conf_type = KMConfigDeviceType(conf_temp.type)
        if conf_type == KMConfigDeviceType.WEB:
            parsed_conf.append(KMConfigWeb.FromString(conf_bita))
        elif conf_type == KMConfigDeviceType.MOBILE:
            parsed_conf.append(KMConfigMobile.FromString(conf_bita))
        else:
            raise ValueError(f"Unknown device type {conf_type!r}")
    return parsed_conf


def save_config(config: KMConfigWeb | KMConfigMobile):
    """Save the config to the user path.

    Parameters
    ----------
    config: :class:`KMConfigWeb` | :class:`KMConfigMobile`
        The config to be saved.
    """
    USER_PATH.mkdir(parents=True, exist_ok=True)

    CONFIG_PATH = USER_PATH / f"kmkc.{config.id}.tmconf"

    CONFIG_PATH.write_bytes(bytes(config))
