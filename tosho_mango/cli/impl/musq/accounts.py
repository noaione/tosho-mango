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

import click
from requests import HTTPError

from tosho_mango import term
from tosho_mango.cli.base import ToshoMangoCommandHandler
from tosho_mango.cli.impl import options
from tosho_mango.sources.musq.config import MUConfig, MUConfigDevice, get_all_config, save_config
from tosho_mango.utils import format_date, get_date_from_unix

from .common import make_client, select_single_account

__all__ = (
    "musq_auth_session",
    "musq_accounts",
    "musq_account_info",
)
console = term.get_console()


@click.command(
    name="auth",
    help="Authenticate your account",
    cls=ToshoMangoCommandHandler,
)
@click.argument("session_id", metavar="SESSION_ID", required=True, type=str)
@click.option(
    "-t",
    "--type",
    "device_type",
    help="Device type to use",
    default="android",
    show_default=True,
    type=click.Choice(["android", "ios"]),
)
def musq_auth_session(session_id: str, device_type: str):
    match device_type.lower():
        case "android":
            device_tt = MUConfigDevice.ANDROID
        case "ios":
            device_tt = MUConfigDevice.IOS
        case _:
            raise click.BadParameter("Invalid device type", param_hint="device_type")

    all_configs = get_all_config()
    old_config: MUConfig | None = None
    for conf in all_configs:
        if conf.session == session_id:
            console.warning("Session ID already authenticated!")
            abort_it = console.confirm("Do you want to replace it?")
            if not abort_it:
                console.info("Aborting...")
                return
            old_config = conf
    console.info(f"Authenticating with session ID {session_id!r} ({device_tt.name})...")
    config = MUConfig.from_auth(session_id, device_tt)

    client = make_client(config)
    try:
        client.get_account()
    except HTTPError as err:
        console.error(f"Failed to authenticate: {err}")
        return

    if old_config is not None:
        old_config.session = config.session
        config = old_config

    console.info("Authentication success! Saving config...")
    save_config(config)


@click.command(
    name="accounts",
    help="Get all authenticated accounts",
    cls=ToshoMangoCommandHandler,
)
def musq_accounts():
    all_configs = get_all_config()
    if not all_configs:
        console.error("No account found!")
        return

    console.info(f"Found {len(all_configs)} account(s)")
    for idx, account in enumerate(all_configs, 1):
        console.info(f"{idx:02d}. {account.id} [{MUConfigDevice(account.type).name}]")


@click.command(
    name="account",
    help="Get account information",
    cls=ToshoMangoCommandHandler,
)
@options.account_id
def musq_account_info(account_id: str | None = None):
    account = select_single_account(account_id)
    if account is None:
        console.warning("Aborted")
        return

    console.info(f"Fetching account info for [highlight]{account.id}[/highlight]...")
    client = make_client(account)

    try:
        account_view = client.get_account()
    except HTTPError as e:
        console.error(f"Failed to check balance: {e}")
        return

    console.info(f"Account info for [highlight]{account.id}[/highlight]:")
    console.info(f"  [bold]Session:[/bold] {account.session}")
    console.info(f"  [bold]Type:[/bold] {MUConfigDevice(account.type).name}")
    console.info(f"  [bold]Registered?[/bold] {account_view.registered!r}")
    if account_view.devices:
        console.info("  [bold]Devices:[/bold]")
        for device in account_view.devices:
            install_at = format_date(get_date_from_unix(device.install_at, 9))
            console.info(f"    - [bold]{device.name}[/bold] ({device.id}) [{install_at}]")
    console.info(f"  [bold]Login Flow:[/bold] {account_view.login_url}")
