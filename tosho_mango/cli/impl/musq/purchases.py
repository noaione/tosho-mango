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

import time
from pathlib import Path

import click
from requests import HTTPError

from tosho_mango import term
from tosho_mango.cli.base import ToshoMangoCommandHandler
from tosho_mango.sources.musq.proto import ChapterV2

from .. import options
from .common import make_client, select_single_account

__all__ = ("musq_manga_purchase",)
console = term.get_console()


@click.command(
    name="purchase",
    help="Purchase a manga chapter for a title",
    cls=ToshoMangoCommandHandler,
)
@click.argument("title_id", type=int, metavar="TITLE_ID", required=True)
@options.account_id
def musq_manga_purchase(title_id: int, account_id: str | None = None):
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

    point_bal = result.user_point
    console.info("Your current point balance:")
    console.info("  - [bold]Total[/bold]: {0:,}".format(point_bal.total_point))
    console.info("  - [bold]Paid point[/bold]: {0:,}c".format(point_bal.paid))
    console.info("  - [bold]Event/XP point[/bold]: {0:,}c".format(point_bal.event))
    console.info("  - [bold]Free point[/bold]: {0:,}c".format(point_bal.free))

    console.info("Title information:")
    console.info(f"  - [bold]ID[/bold]: {title_id}")
    console.info(f"  - [bold]Title[/bold]: {result.title}")
    console.info(f"  - [bold]Chapters[/bold]: {len(result.chapters)} chapters")

    select_choices = [
        term.ConsoleChoice(str(chapter.id), f"{chapter.chapter_title} ({chapter.price}c)")
        for chapter in result.chapters
    ]
    selected = console.select("Select chapter to purchase", select_choices)
    if not selected:
        console.warning("No chapter selected, aborting")
        return

    selected_ch_ids = list(map(lambda x: int(x.name), selected))

    ids_lists = [chapter.id for chapter in result.chapters]
    console.status(f"Purchasing chapter(s)... (1/{len(selected_ch_ids)})")
    claimed_total = 0
    failed_chapters: list[tuple[ChapterV2, str]] = []
    for idx, ch_id in enumerate(selected_ch_ids):
        console.status(f"Purchasing chapter(s)... ({idx + 1}/{len(selected_ch_ids)})")
        id_index = ids_lists.index(ch_id)
        chapter = result.chapters[id_index]

        consume = client.calculate_coin(point_bal, chapter)
        if not consume.is_possible():
            console.warning(
                f"Unable to purchase chapter [highlight]{chapter.chapter_title}[/highlight] (ID: {chapter.id}),"
                "insufficient point balance",
            )
            failed_chapters.append((chapter, "Insufficient point balance"))
            continue
        point_bal.free -= consume.free
        point_bal.paid -= consume.paid
        point_bal.event -= consume.event
        img_chapter = client.get_chapter_images(chapter.id, coins=consume)
        Path(f"{chapter.id}").write_text(img_chapter.SerializeToString().hex())
        if not img_chapter.blocks:
            console.warning(
                f"Unable to purchase chapter [highlight]{chapter.chapter_title}[/highlight] (ID: {chapter.id}),"
                "no images available",
            )
            failed_chapters.append((chapter, "Failed when claiming"))
            continue
        time.sleep(0.5)
        claimed_total += 1
    console.stop_status(f"Purchased {claimed_total} chapters!")
    if len(failed_chapters) > 0:
        console.warning(f"We failed to purchase {len(failed_chapters)} chapters, you might want to retry")
        for chapter, error_msg in failed_chapters:
            console.warning(f"  - [bold]{chapter.chapter_title}[/bold] (ID: {chapter.id}): [error]{error_msg}[/error]")
