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

import click

from tosho_mango import term
from tosho_mango.cli.base import ToshoMangoCommandHandler
from tosho_mango.sources.kmkc.config import KMConfigWeb
from tosho_mango.sources.kmkc.errors import KMAPIError

from .. import options
from .common import make_web_client, select_single_account

__all__ = ("kmkc_balance",)
console = term.get_console()


@click.command(
    name="balance",
    help="Get account balance/point information",
    cls=ToshoMangoCommandHandler,
)
@options.account_id
def kmkc_balance(account_id: str | None = None):
    account = select_single_account(account_id)
    if account is None:
        console.warning("Aborted")
        return
    if not isinstance(account, KMConfigWeb):
        console.error("Only web account is supported for now!")
        return

    console.info(f"Checking account balance for [highlight]{account.id}[/highlight]...")
    client = make_web_client(account=account)

    try:
        point_bal = client.get_user_point()
    except KMAPIError as exc:
        console.error(f"Failed to get your account: {exc}")
        return

    console.info("Your current point balance:")
    console.info("  - [bold]Total[/bold]: [bcyan][highr]{0:,}[/highr]c[/bcyan]".format(point_bal.point.total_point))
    console.info(
        "  - [bold]Paid point[/bold]: [success][highr]{0:,}[/highr]c[/success]".format(point_bal.point.paid_point)
    )
    console.info("  - [bold]Free point[/bold]: [info][highr]{0:,}[/highr]c[/info]".format(point_bal.point.free_point))
    console.info(
        "  - [bold]Premium ticket[/bold]: [orange][highr]{0:,}[/highr] ticket[/orange]".format(
            point_bal.ticket.total_num
        )
    )
