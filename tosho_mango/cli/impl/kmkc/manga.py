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
from tosho_mango.sources.kmkc.dto import EpisodeBadge, EpisodeEntry, GenreNode, MagazineCategory, TitleList
from tosho_mango.sources.kmkc.errors import KMAPIError
from tosho_mango.sources.musq.models import WeeklyCode

from .. import options
from .common import do_print_search_information, make_web_client, select_single_account

__all__ = (
    "kmkc_search_title",
    "kmkc_search_weekly",
    "kmkc_title_info",
    "kmkc_magazines_list",
)
console = term.get_console()


@click.command(
    name="search",
    help="Search for a title",
    cls=ToshoMangoCommandHandler,
)
@click.argument("title", type=str, metavar="TITLE", required=True)
@options.account_id
def kmkc_search_title(title: str, account_id: str | None = None):
    account = select_single_account(account_id)
    if account is None:
        console.warning("Aborted")
        return
    if not isinstance(account, KMConfigWeb):
        console.error("Only web account is supported for now!")
        return

    console.info(f"Searching for [highlight]{title}[/highlight]...")
    client = make_web_client(account=account)

    try:
        search_results = client.search(title)
    except KMAPIError as exc:
        console.error(f"Failed to get search results: {exc}")
        return

    if not search_results:
        console.error("No result found.")
        return

    console.info(f"Seach results ([highlight]{len(search_results)}[/highlight] results):")
    do_print_search_information(search_results)


@click.command(
    name="weekly",
    help="Get weekly manga",
    cls=ToshoMangoCommandHandler,
)
@click.option(
    "-d",
    "--day",
    "weekday",
    type=click.Choice(WeeklyCode),  # type: ignore
    help="The selected weekday to get, default to current day at JST",
    default=None,
    show_default=False,
)
@options.account_id
def kmkc_search_weekly(weekday: WeeklyCode | None = None, account_id: str | None = None):
    account = select_single_account(account_id)
    if account is None:
        console.warning("Aborted")
        return
    if not isinstance(account, KMConfigWeb):
        console.error("Only web account is supported for now!")
        return

    current_day = weekday or WeeklyCode.today()
    console.info(f"Getting weekly manga for weekday [highlight]{current_day.name}[/highlight]...")
    client = make_web_client(account=account)

    try:
        all_weekly_responses = client.get_weekly()
    except KMAPIError as exc:
        console.error(f"Failed to get weekly information: {exc}")
        return

    title_ids_list: list[int] = []
    for weekly_info in all_weekly_responses.weekly_list:
        if weekly_info.weekday_index == current_day.indexed + 1:
            title_ids_list = weekly_info.title_id_list
            break

    if not title_ids_list:
        console.error("Unknown weekday provided.")
        return

    title_list: list[TitleList] = []
    for title in all_weekly_responses.title_list:
        if title.title_id in title_ids_list:
            title_list.append(title)

    if not title_ids_list:
        console.error("No result found.")
        return

    console.info(f"Weekday [bold]{current_day.name}[/bold] results ([highlight]{len(title_list)}[/highlight] results):")
    do_print_search_information(title_list)


def _fmt_tags(genre_list: list[GenreNode], genre_ids: list[int]) -> str:
    joined_tags: list[str] = []
    added_ids: list[int] = []
    for genre in genre_list:
        if genre.genre_id in genre_ids:
            joined_tags.append(f"[gray][highr]{genre.genre_name}[/highr][/gray]")
            added_ids.append(genre.genre_id)
    # Check if there are any missing tags
    missing_ids = set(genre_ids) - set(added_ids)
    for missing in missing_ids:
        joined_tags.append(f"[red][highr]Unknown ({missing})[/highr][/red]")
    return ", ".join(joined_tags)


@click.command(
    name="info",
    help="Get title information",
    cls=ToshoMangoCommandHandler,
)
@click.argument("title_id", type=int, metavar="TITLE_ID", required=True)
@options.account_id
@click.option(
    "--chapters",
    "-c",
    "show_chapters",
    is_flag=True,
    help="Show chapters information",
    show_default=True,
)
def kmkc_title_info(title_id: int, account_id: str | None = None, show_chapters: bool = False):
    account = select_single_account(account_id)
    if account is None:
        console.warning("Aborted")
        return
    if not isinstance(account, KMConfigWeb):
        console.error("Only web account is supported for now!")
        return

    console.info(f"Searching for ID [highlight]{title_id}[/highlight]...")
    client = make_web_client(account=account)

    try:
        results = client.get_title_list([title_id])
    except KMAPIError as exc:
        console.error(f"Failed to get title information: {exc}")
        return

    if not results:
        console.error("Unable to find title information.")
        return

    result = results[0]
    genre_results: list[GenreNode] = []

    if result.genre_id_list:
        try:
            genre_results = client.get_genre_list().genre_list
        except KMAPIError as exc:
            console.error(f"Failed to get title information: {exc}")
            return

    chapters_info: list[EpisodeEntry] = []
    if show_chapters:
        console.info(f"Fetching [highlight]{len(result.episode_id_list)}[/highlight] chapters information...")
        for episode_ids in client.chunk_episodes(result.episode_id_list):
            try:
                chapters_info.extend(client.get_episode_list(episode_ids))
            except KMAPIError as exc:
                console.error(f"Failed to get chapter information: {exc}")
                return

    manga_url = f"https://kmanga.kodansha.com/title/{title_id}"
    console.info(f"Title information for [highlight][link={manga_url}]{result.title_name}[/link][/highlight]")

    console.info(f"  [bold]Author[/bold]: {result.author_text}")
    if result.genre_id_list:
        console.info(f"  [bold]Genre/Tags[/bold]: {_fmt_tags(genre_results, result.genre_id_list)}")
    if result.magazine_category and result.magazine_category is not MagazineCategory.Undefined:
        console.info(f"  [bold]Magazine[/bold]: {result.magazine_category.pretty}")
    console.info("   [bold]Summary[/bold]")
    split_desc = result.introduction_text.split("\n")
    for desc in split_desc:
        console.info(f"    {desc}")

    if result.notice_text:
        console.info(f"  [bold]Notice[/bold]: {result.notice_text}")

    console.enter()
    console.info(f"  [bold]Chapters[/bold]: {len(result.episode_id_list)} chapters")
    if show_chapters:
        for chapter in chapters_info:
            episode_url = f"{manga_url}/episode/{chapter.episode_id}"
            text_info = f"    [bold][link={episode_url}]{chapter.episode_name}[/link][/bold] ({chapter.episode_id})"
            if chapter.badge is EpisodeBadge.PURCHASEABLE:
                if chapter.ticket_rental_enabled == 1:
                    text_info += " [[orange]🎫 [highr]FREE[/highr][/orange]]"
                else:
                    text_info += f" [[success][highr]P{chapter.point}[/highr][/success]]"
            elif chapter.badge is EpisodeBadge.FREE:
                text_info += " [[darkb][highr]FREE[/highr][/darkb]]"
            elif chapter.badge is EpisodeBadge.PURCHASED:
                text_info += " [[success]Purchased[/success]]"
            elif chapter.badge is EpisodeBadge.RENTAL and chapter.rental_rest_time is not None:
                text_info += f" [[orange]Renting: {chapter.rental_rest_time}[/orange]]"
            console.info(text_info)
        console.enter()
    if result.next_updated_text:
        console.info(f"  [bold]Next Update[/bold]: {result.next_updated_text}")


@click.command(
    name="magazines",
    help="Get magazines list",
    cls=ToshoMangoCommandHandler,
)
@options.account_id
def kmkc_magazines_list(account_id: str | None = None):
    account = select_single_account(account_id)
    if account is None:
        console.warning("Aborted")
        return
    if not isinstance(account, KMConfigWeb):
        console.error("Only web account is supported for now!")
        return

    console.info("Getting magazines list...")
    client = make_web_client(account=account)

    try:
        search_results = client.get_magazines()
    except KMAPIError as exc:
        console.error(f"Failed to get search results: {exc}")
        return

    if not search_results.magazine_category_list:
        console.error("No result found.")
        return

    for magazine in search_results.magazine_category_list:
        console.info(f"[bold]{magazine.category_name}[/bold] ({magazine.category_id})")
