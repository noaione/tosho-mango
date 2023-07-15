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
from pathlib import Path
from typing import Type
from uuid import uuid4

import betterproto

from tosho_mango.constants import USER_PATH


class MUConfigDevice(betterproto.Enum):
    """Device type for SQ MU session"""

    ANDROID = 1
    """Android device"""
    IOS = 2  # Not implemented yet though
    """iOS device"""


@dataclass
class MUConfig(betterproto.Message):
    """Represents the config file for SQ MU!"""

    id: str = betterproto.string_field(1)
    """:class:`str`: The ID for SQ MU!"""
    session: str = betterproto.string_field(2)
    """:class:`str`: The session ID for SQ MU!"""
    type: MUConfigDevice = betterproto.enum_field(3)
    """:class:`MUConfigDevice`: The device type for SQ MU!"""

    @classmethod
    def from_auth(cls: Type["MUConfig"], session: str, type: MUConfigDevice):
        """Create the config from the auth session.

        Parameters
        ----------
        session: :class:`str`
            The session ID for SQ MU!
        type: :class:`MUConfigDevice`
            The device type for SQ MU!

        Returns
        -------
        :class:`MUConfig`
            The config for SQ MU!
        """
        return cls(id=str(uuid4()), session=session, type=type)


def get_config(account_id: str, *, path: Path = USER_PATH) -> MUConfig | None:
    """Get a single config from the account ID.

    Parameters
    ----------
    account_id: :class:`str`
        The account ID to be fetched.

    Returns
    -------
    :class:`MUConfig` | None
        The config if found, else None.
    """

    path.mkdir(parents=True, exist_ok=True)

    CONFIG_PATH = path / f"musq.{account_id}.tmconf"

    if not CONFIG_PATH.exists():
        return None

    conf_data = MUConfig.FromString(CONFIG_PATH.read_bytes())
    return conf_data


def get_all_config(path: Path = USER_PATH) -> list[MUConfig]:
    """Get all config from the user path.

    Returns
    -------
    list[MUConfig]
        The list of config.
    """

    path.mkdir(parents=True, exist_ok=True)

    CONFIG_GLOB = path.glob("musq.*.tmconf")
    parsed_conf: list[MUConfig] = []
    for conf in CONFIG_GLOB:
        parsed_conf.append(MUConfig.FromString(conf.read_bytes()))
    return parsed_conf


def save_config(config: MUConfig, *, path: Path = USER_PATH):
    """Save the config to the user path.

    Parameters
    ----------
    config: :class:`MUConfig`
        The config to be saved.
    """
    path.mkdir(parents=True, exist_ok=True)

    CONFIG_PATH = path / f"musq.{config.id}.tmconf"

    CONFIG_PATH.write_bytes(bytes(config))
