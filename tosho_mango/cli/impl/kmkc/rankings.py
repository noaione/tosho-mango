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

from tosho_mango import term
from tosho_mango.cli.base import ToshoMangoCommandHandler
from tosho_mango.sources.kmkc.config import KMConfigWeb
from tosho_mango.sources.kmkc.errors import KMAPIError
from tosho_mango.sources.kmkc.models import RankingTabs

from .. import options
from .common import do_print_search_information, make_web_client, select_single_account

__all__ = ("kmkc_home_rankings",)
console = term.get_console()

_TabToRankingsMap = {rank.tab: rank for rank in RankingTabs}


@click.command(
    name="rankings",
    help="Get current home rankings",
    cls=ToshoMangoCommandHandler,
)
@click.option(
    "-r",
    "--ranking",
    "ranking_tab",
    type=click.Choice(list(_TabToRankingsMap.keys())),
    default="all",
    show_default=True,
    help="Which ranking tab to show",
)
@click.option(
    "-l",
    "--limit",
    "show_limit",
    type=click.IntRange(1, 100),
    default=25,
    show_default=True,
    help="How many results to show",
)
@options.account_id
def kmkc_home_rankings(ranking_tab: str = "all", show_limit: int = 25, account_id: str | None = None):
    account = select_single_account(account_id)
    if account is None:
        console.warning("Aborted")
        return
    if not isinstance(account, KMConfigWeb):
        console.error("Only web account is supported for now!")
        return

    client = make_web_client(account=account)
    rank_info = _TabToRankingsMap.get(ranking_tab)
    if rank_info is None:
        console.error("Invalid ranking tab")
        return

    try:
        results = client.get_all_rankings(rank_info.id, limit=show_limit)
    except KMAPIError as exc:
        console.error(f"Failed to get ranking {rank_info.name} wallet: {exc}")
        return

    if not results.ranking_title_list:
        console.warning("No results found")
        return

    console.info(
        f"Fetching [highlight]{len(results.ranking_title_list)}[/highlight] results "
        f"from [highlight]{rank_info.name}[/highlight]"
    )
    try:
        all_titles = client.get_title_list([title.id for title in results.ranking_title_list])
    except KMAPIError as exc:
        console.error(f"Failed to get title list: {exc}")
        return

    if not all_titles:
        console.warning("No results found")
        return

    console.info(f"Ranking [highlight]{rank_info.name}[/highlight] ([bold]{len(all_titles)}[/bold] results)")
    do_print_search_information(all_titles, numbering=True)
