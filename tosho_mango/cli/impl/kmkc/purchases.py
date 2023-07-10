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
import msgspec

from tosho_mango import term
from tosho_mango.cli.base import ToshoMangoCommandHandler
from tosho_mango.sources.kmkc.config import KMConfigWeb
from tosho_mango.sources.kmkc.constants import BASE_HOST
from tosho_mango.sources.kmkc.dto import EpisodeEntry, PremiumTicketInfo, TitleTicketInfo, UserPoint
from tosho_mango.sources.kmkc.errors import KMAPIError, KMNotEnoughPointError

from .. import options
from .common import make_web_client, select_single_account

__all__ = (
    "kmkc_title_purchase",
    "kmkc_account_purchases",
)
console = term.get_console()


@click.command(
    name="purchase",
    help="Purchase a manga chapter for a title",
    cls=ToshoMangoCommandHandler,
)
@click.argument("title_id", type=int, metavar="TITLE_ID", required=True)
@options.account_id
def kmkc_title_purchase(title_id: int, account_id: str | None = None):
    account = select_single_account(account_id)
    if account is None:
        console.warning("Aborted")
        return
    if not isinstance(account, KMConfigWeb):
        console.error("Only web account is supported for now!")
        return

    client = make_web_client(account=account)

    console.info(f"Getting user point for [highlight]{account.username}[/highlight]...")
    try:
        user_wallet = client.get_user_point()
    except KMAPIError as exc:
        console.error(f"Failed to get user wallet: {exc}")
        return

    console.info(f"Getting title information for ID [highlight]{title_id}[/highlight]...")
    try:
        results = client.get_title_list([title_id])
    except KMAPIError as exc:
        console.error(f"Failed to get title information: {exc}")
        return

    if not results:
        console.error("Unable to find title information.")
        return

    result = results[0]

    console.info(f"Fetching [highlight]{result.title_name}[/highlight] title ticket...")
    try:
        ticket_entry = client.get_title_ticket(title_id)
    except KMAPIError as exc:
        console.error(f"Failed to get title ticket information: {exc}")
        return

    chapters_info: list[EpisodeEntry] = []
    console.info(f"Fetching [highlight]{len(result.episode_id_list)}[/highlight] chapters information...")
    for episode_ids in client.chunk_episodes(result.episode_id_list):
        try:
            chapters_info.extend(client.get_episode_list(episode_ids))
        except KMAPIError as exc:
            console.error(f"Failed to get chapter information: {exc}")
            return

    console.info("Your current point balance:")
    console.info("  - [bold]Total[/bold]: [bcyan][highr]{0:,}[/highr]c[/bcyan]".format(user_wallet.point.total_point))
    console.info(
        "  - [bold]Paid point[/bold]: [success][highr]{0:,}[/highr]c[/success]".format(user_wallet.point.paid_point),
    )
    console.info("  - [bold]Free point[/bold]: [info][highr]{0:,}[/highr]c[/info]".format(user_wallet.point.free_point))
    console.info(
        "  - [bold]Premium ticket[/bold]: [orange][highr]{0:,}[/highr] ticket[/orange]".format(
            user_wallet.ticket.total_num,
        ),
    )
    console.info(f"  - [bold]Title ticket?[/bold]: {ticket_entry.title_available()!r}")

    console.info("Title information:")
    console.info(f"  - [bold]ID[/bold]: {result.title_id}")
    console.info(f"  - [bold]Title[/bold]: {result.title_name}")
    console.info(f"  - [bold]Chapters[/bold]: {len(chapters_info)} chapters")

    # Only show unpurchased chapters
    chapters_info = [chapter for chapter in chapters_info if not chapter.available()]
    select_choices = [
        term.ConsoleChoice(
            str(chapter.episode_id),
            f"{chapter.episode_name} ({chapter.point}P)"
            if not chapter.ticketable()
            else f"{chapter.episode_name} ({chapter.point}P/Ticket)",
        )
        for chapter in chapters_info
    ]
    selected = console.select("Select chapters to purchase", select_choices)
    if not selected:
        console.warning("No chapter selected, aborting")
        return

    selected_ch_ids = list(map(lambda x: int(x.name), selected))
    ids_lists = [chapter.episode_id for chapter in chapters_info]
    point_claimant: list[EpisodeEntry] = []
    _wallet_copy = msgspec.json.decode(msgspec.json.encode(user_wallet.point), type=UserPoint)
    ticket_claimant: list[tuple[EpisodeEntry, TitleTicketInfo | PremiumTicketInfo]] = []
    for episode_id in selected_ch_ids:
        episode = chapters_info[ids_lists.index(episode_id)]
        if episode.available():
            console.warning(f"  Chapter [highlight]{episode.episode_name}[/highlight] is already purchased, skipping")
            continue

        if episode.ticketable() and ticket_entry.title_available():
            ticket_entry.subtract_title()
            ticket_claimant.append((episode, ticket_entry.ticket_info.title_ticket_info))
        elif episode.ticketable() and ticket_entry.premium_available():
            ticket_entry.subtract_premium()
            ticket_claimant.append((episode, ticket_entry.ticket_info.premium_ticket_info))
        else:
            if _wallet_copy.can_purchase(episode.point):
                _wallet_copy.subtract(episode.point)
                _wallet_copy.add(episode.bonus_point)
                point_claimant.append(episode)

    total_chs = len(point_claimant) + len(ticket_claimant)
    if not total_chs:
        console.warning("No chapters to purchase after precalculation, aborting")
        return

    console.info("Precalculated purchase information:")
    console.info(f"  - [bold]Total chapters[/bold]: {len(point_claimant)}")
    console.info(f"  - [bold]With ticket[/bold]: {ticket_claimant}T")

    current_dex = 1
    failure_count = 0
    console.status(f"Purchasing chapter(s).. ({current_dex}/{total_chs})")
    # Claim ticket first
    for episode, ticket_info in ticket_claimant:
        console.status(f"Purchasing chapter(s).. ({current_dex}/{total_chs})")
        try:
            client.claim_episode_with_ticket(episode.episode_id, ticket_info)
        except KMAPIError as exc:
            console.warning(f"Failed to purchase chapter, ignoring: {exc}")
            failure_count += 1
        current_dex += 1

    # Claim point
    console.status(f"Purchasing chapter(s).. ({current_dex}/{total_chs}) [point]")
    try:
        client.claim_bulk_episode(point_claimant, user_wallet.point)
        current_dex += len(point_claimant)
    except KMNotEnoughPointError as exc:
        console.error(f"Not enough point to purchase chapters: {exc}")
        failure_count += len(point_claimant)
    except KMAPIError as exc:
        console.error(f"Failed to purchase chapters: {exc}")
        failure_count += len(point_claimant)

    console.stop_status(f"Purchased [highlight]{current_dex}[/highlight] chapters")

    if failure_count:
        console.warning(f"  There is [highlight]{failure_count}[/highlight] chapters that failed to purchase")


@click.command(
    name="purchased",
    help="Get your purchased title information",
    cls=ToshoMangoCommandHandler,
)
@options.account_id
def kmkc_account_purchases(account_id: str | None = None):
    account = select_single_account(account_id)
    if account is None:
        console.warning("Aborted")
        return
    if not isinstance(account, KMConfigWeb):
        console.error("Only web account is supported for now!")
        return

    client = make_web_client(account=account)

    console.info(f"Getting user point for [highlight]{account.username}[/highlight]...")
    try:
        title_purchased = client.get_purchased()
    except KMAPIError as exc:
        console.error(f"Failed to get user purchases: {exc}")
        return

    console.info(f"Purchased title ([highlight]{len(title_purchased.title_list)}[/highlight] results):")
    for result in title_purchased.title_list:
        manga_url = f"https://{BASE_HOST}/title/{result.title_id}"
        manga_text = f"[bold][link={manga_url}]{result.title_name}[/link][/bold] ({result.title_id})"
        console.info(f"  {manga_text}")
        console.info(f"   {manga_url}")
