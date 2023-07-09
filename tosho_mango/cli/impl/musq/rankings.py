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

from .. import options
from .common import do_print_search_information, make_client, select_single_account

__all__ = ("musq_home_rankings",)
console = term.get_console()


@click.command(
    name="rankings",
    help="Get current home rankings",
    cls=ToshoMangoCommandHandler,
)
@options.account_id
def musq_home_rankings(account_id: str | None = None):
    account = select_single_account(account_id)
    if account is None:
        console.warning("Aborted")
        return

    console.info(f"Getting favorites list for [highlight]{account.id}[/highlight]...")
    client = make_client(account)

    try:
        results = client.get_my_home()
    except HTTPError as e:
        console.error(f"Unable to connect to MU!: {e}")
        return

    if not results.rankings:
        console.error("There is no rankings available for some reason.")
        return

    ranking_name = [rank.name for rank in results.rankings]
    while True:
        cancel = term.ConsoleChoice("_cancel", "Cancel")
        rank_choice = [term.ConsoleChoice(rank, rank) for rank in ranking_name]
        rank_choice.append(cancel)

        select = console.choice("Select ranking you want to see", rank_choice, cancel)
        if select.name == cancel.name:
            break

        rank_idx = ranking_name.index(select.name)
        ranking = results.rankings[rank_idx]

        console.info(f"Ranking for [highlight]{ranking.name}[/highlight] ({len(ranking.titles)} titles):")
        do_print_search_information(ranking.titles, numbering=True)
        console.enter()
