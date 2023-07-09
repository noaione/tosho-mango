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
from tosho_mango.sources.musq.models import WeeklyCode
from tosho_mango.sources.musq.proto import Tag

from .. import options
from .common import do_print_search_information, make_client, select_single_account

__all__ = (
    "musq_search_title",
    "musq_search_weekly",
    "musq_title_info",
)
console = term.get_console()


@click.command(
    name="search",
    help="Search for a title",
    cls=ToshoMangoCommandHandler,
)
@click.argument("title", type=str, metavar="TITLE", required=True)
@options.account_id
def musq_search_title(title: str, account_id: str | None = None):
    account = select_single_account(account_id)
    if account is None:
        console.warning("Aborted")
        return

    console.info(f"Searching for [highlight]{title!r}[/highlight]...")
    client = make_client(account)

    try:
        results = client.search_manga(title)
    except HTTPError as e:
        console.error(f"Unable to connect to MU!: {e}")
        return

    if not results.titles:
        console.warning("No results found.")
        return

    console.info(f"Seach results ([highlight]{len(results.titles)}[/highlight] results):")
    do_print_search_information(results.titles)


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
def musq_search_weekly(weekday: WeeklyCode | None = None, account_id: str | None = None):
    account = select_single_account(account_id)
    if account is None:
        console.warning("Aborted")
        return

    current_day = weekday or WeeklyCode.today()
    console.info(f"Getting weekly manga for weekday [highlight]{current_day.name}[/highlight]...")
    client = make_client(account)

    try:
        results = client.get_weekly_titles(current_day)
    except HTTPError as e:
        console.error(f"Unable to connect to MU!: {e}")
        return

    if not results.titles:
        console.warning("No results found.")
        return

    console.info(
        f"Weekday [bold]{current_day.name}[/bold] results ([highlight]{len(results.titles)}[/highlight] results):",
    )
    do_print_search_information(results.titles)


def _fmt_tags(tags_data: list[Tag]) -> str:
    joined_tags: list[str] = []
    for tag in tags_data:
        genre_url = f"https://global.manga-up.com/genre/{tag.id}"
        text_d = f"[gray][highr][link={genre_url}]{tag.name}[/link][/highr][/gray]"
        joined_tags.append(text_d)
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
def musq_title_info(title_id: int, account_id: str | None = None, show_chapters: bool = False):
    account = select_single_account(account_id)
    if account is None:
        console.warning("Aborted")
        return

    console.info(f"Searching for ID [highlight]{title_id}[/highlight]...")
    client = make_client(account)

    try:
        result = client.get_manga(title_id)
    except HTTPError as e:
        console.error(f"Unable to connect to MU!: {e}")
        return

    manga_url = f"https://global.manga-up.com/manga/{title_id}"
    console.info(f"Title information for [highlight][link={manga_url}]{result.title}[/link][/highlight]")

    console.info(f"  [bold]Author[/bold]: {result.authors}")
    console.info(f"  [bold]Genre/Tags[/bold]: {_fmt_tags(result.tags)}")
    console.info("   [bold]Summary[/bold]")
    split_desc = result.description.split("\n")
    for desc in split_desc:
        console.info(f"    {desc}")

    if result.warning:
        console.info(f"  [bold]Warning[/bold]: {result.warning}")

    console.enter()
    console.info(f"  [bold]Chapters[/bold]: {len(result.chapters)} chapters")
    if show_chapters:
        for chapter in result.chapters:
            console.info(f"    [bold]{chapter.name}[/bold] ({chapter.id})")
            if chapter.subtitle:
                console.info(f"     [bold]{chapter.subtitle}[/bold]")
            console.info(f"      [bold]Price[/bold]: {chapter.price}c")
        console.enter()
    if result.next_update:
        console.info(f"  [bold]Next Update[/bold]: {result.next_update}")
    console.info(f"  [bold]Copyright[/bold]: {result.copyright}")
