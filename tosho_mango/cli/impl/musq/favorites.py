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
from requests import HTTPError

from tosho_mango import term
from tosho_mango.cli.base import ToshoMangoCommandHandler

from .. import options
from .common import do_print_search_information, make_client, select_single_account

__all__ = (
    "musq_my_favorites",
    "musq_my_read_history",
)
console = term.get_console()


@click.command(
    name="favorites",
    help="Get my favorites list",
    cls=ToshoMangoCommandHandler,
)
@options.account_id
def musq_my_favorites(account_id: str | None = None):
    account = select_single_account(account_id)

    console.info(f"Getting favorites list for [highlight]{account.id}[/highlight]...")
    client = make_client(account)

    try:
        results = client.get_my_manga()
    except HTTPError as e:
        console.error(f"Unable to connect to MU!: {e}")
        return

    if not results.favorites:
        console.warning("No favorites found.")
        return

    console.info(f"Your favorites list ([highlight]{len(results.favorites)}[/highlight] results):")
    do_print_search_information(results.favorites)


@click.command(
    name="history",
    help="Get my read history",
    cls=ToshoMangoCommandHandler,
)
@options.account_id
def musq_my_read_history(account_id: str | None = None):
    account = select_single_account(account_id)

    console.info(f"Getting read history for [highlight]{account.id}[/highlight]...")
    client = make_client(account)

    try:
        results = client.get_my_manga()
    except HTTPError as e:
        console.error(f"Unable to connect to MU!: {e}")
        return

    if not results.history:
        console.warning("No favorites found.")
        return

    console.info(f"Your read history ([highlight]{len(results.history)}[/highlight] results):")
    do_print_search_information(results.history)
