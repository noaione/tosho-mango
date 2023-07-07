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

from http.cookiejar import MozillaCookieJar
from pathlib import Path

import click
import requests

from tosho_mango import term
from tosho_mango.cli.base import ToshoMangoCommandHandler
from tosho_mango.cli.impl.kmkc.common import make_web_client
from tosho_mango.sources.kmkc.config import KMConfigDeviceType, KMConfigWeb, get_all_config, save_config
from tosho_mango.sources.kmkc.errors import KMAPIError

__all__ = (
    "kmkc_auth_session",
    "kmkc_accounts",
)
console = term.get_console()


@click.command(
    name="authweb",
    help="Authenticate your account (web mode)",
    cls=ToshoMangoCommandHandler,
)
@click.argument("cookies", metavar="COOKIE_PATH", required=True, type=Path)
def kmkc_auth_session(cookies: Path):
    console.info("Authenticating your account...")

    cookie_jar = MozillaCookieJar(cookies)
    cookie_jar.load()

    session = requests.Session()
    session.cookies.update(cookie_jar)

    config = KMConfigWeb.from_cookies(session.cookies)

    client = make_web_client(config)
    try:
        account_resp = client.get_account()
    except KMAPIError as exc:
        console.error(f"Failed to authenticate your account: {exc}")
        return

    account = account_resp.account
    console.info(f"Authenticated as [highlight]{account.nickname}[/highlight] ({account.account_id}/{account.user_id})")
    all_configs = get_all_config()
    old_config: KMConfigWeb | None = None
    for conf in all_configs:
        if not isinstance(conf, KMConfigWeb):
            continue
        if conf.account_id == account.account_id and conf.device_id == account.user_id:
            console.warning("Session ID already authenticated!")
            abort_it = console.confirm("Do you want to replace it?")
            if not abort_it:
                console.info("Aborting...")
                return
            old_config = conf

    if old_config is not None:
        config.id = old_config.id

    console.info("Authentication success! Saving config...")
    save_config(config)


@click.command(
    name="accounts",
    help="Get all authenticated accounts",
    cls=ToshoMangoCommandHandler,
)
def kmkc_accounts():
    all_configs = get_all_config()
    if not all_configs:
        console.error("No account found!")
        return

    console.info(f"Found {len(all_configs)} account(s)")
    for idx, account in enumerate(all_configs, 1):
        console.info(
            f"{idx:02d}. {account.id} — [bold]{account.username}[/bold] [{KMConfigDeviceType(account.type).name}]"
        )
