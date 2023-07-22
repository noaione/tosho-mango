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

from pathlib import Path

import click
import msgspec
from requests import HTTPError

from tosho_mango import models, term
from tosho_mango.cli.base import ToshoMangoCommandHandler
from tosho_mango.sources.musq.models import ConsumeCoin, Quality
from tosho_mango.sources.musq.proto import ChapterV2, MangaDetailV2

from .. import options
from .common import make_client, parse_published, select_single_account

__all__ = (
    "musq_manga_download",
    "musq_manga_auto_download",
)
console = term.get_console()


def create_chapters_info(manga_detail: MangaDetailV2) -> bytes:
    chapters: list[models.ChapterDetailDump] = []
    for chapter in manga_detail.chapters:
        ch_dump = models.ChapterDetailDump(id=chapter.id, main_name=chapter.name)
        if (pub_time := parse_published(chapter.published)) is not None:
            ch_dump.timestamp = int(pub_time.timestamp())
        if chapter.subtitle is not None:
            ch_dump.sub_name = chapter.subtitle
        chapters.append(ch_dump)
    return msgspec.json.format(
        msgspec.json.encode(models.MangaDetailDump(manga_detail.title, manga_detail.authors, chapters)),
        indent=4,
    )


def get_output_directory(
    output_dir: Path, title_id: int | str, chapter_id: int | str | None = None, *, skip_create: bool = False
):
    pathing = output_dir / str(title_id)
    if not skip_create:
        pathing.mkdir(parents=True, exist_ok=True)
    if chapter_id is not None:
        pathing = pathing / str(chapter_id)
        if not skip_create:
            pathing.mkdir(parents=True, exist_ok=True)
    return pathing


@click.command(
    name="download",
    help="Download manga chapter(s) for a title",
    cls=ToshoMangoCommandHandler,
)
@click.argument("title_id", type=int, metavar="TITLE_ID", required=True)
@options.output_dir
@click.option(
    "-c",
    "--chapter",
    "chapter_ids",
    type=options.COMMA_SEPARATED_NUMBER,
    help="Specify the chapter ID to purchase",
    default=None,
    required=False,
)
@click.option(
    "--show-all",
    "show_all",
    help="Show all chapters (including unavailable)",
    is_flag=True,
    default=False,
)
@click.option(
    "-ap",
    "--auto-purchase",
    "auto_purchase",
    help="Automatically purchase chapters, if needed",
    is_flag=True,
    default=False,
)
@click.option(
    "-q",
    "--quality",
    "image_quality",
    type=click.Choice(Quality),  # type: ignore
    help="Specify the image quality to download",
    default=Quality.HIGH,
    show_default=True,
)
@options.account_id
def musq_manga_download(
    title_id: int,
    output_dir: Path,
    chapter_ids: list[int] | None = None,
    show_all: bool = False,
    auto_purchase: bool = False,
    image_quality: Quality = Quality.HIGH,
    account_id: str | None = None,
):
    # Remove chapters dupe
    if chapter_ids is not None:
        chapter_ids = list(dict.fromkeys(chapter_ids))

    account = select_single_account(account_id)
    if account is None:
        console.warning("Aborted")
        return

    console.info(f"Getting manga ID [highlight]{title_id}[/highlight]...")
    client = make_client(account)

    try:
        result = client.get_manga(title_id)
    except HTTPError as e:
        console.error(f"Unable to connect to MU!: {e}")
        return

    coin_purse = result.user_point
    manga_chapters = result.chapters
    manga_chapters.sort(key=lambda x: x.id)

    if chapter_ids is None:
        select_choices = [
            term.ConsoleChoice(str(chapter.id), chapter.chapter_title)
            for chapter in manga_chapters
            if chapter.is_free and not show_all
        ]
        selected = console.select("Select chapter to download", select_choices)
        if not selected:
            console.warning("No chapter selected, aborting")
            return

        chapter_ids = list(map(lambda x: int(x.name), selected))

    console.info(f"Downloading {len(chapter_ids)} chapters...")
    # Precalculate chapters consumption
    ids_lists = [chapter.id for chapter in manga_chapters]
    actual_chapters: list[ChapterV2] = []
    for ch_id in chapter_ids:
        if ch_id not in ids_lists:
            console.warning(f"Chapter ID [bold]{ch_id}[/bold] is not available, skipping")
            continue

        id_index = ids_lists.index(ch_id)
        chapter = manga_chapters[id_index]
        if chapter.is_free:
            actual_chapters.append(chapter)
            continue
        consume = client.calculate_coin(coin_purse, chapter)
        if not consume.is_possible():
            console.warning(
                f"Chapter [highlight]{chapter.chapter_title}[/highlight] ([bold]{chapter.id}[/bold]) is not available "
                "for purchase, skipping",
            )
            console.warning(f"Need {consume.free} free coin, {consume.event} coin, and {consume.paid} paid coin")
            continue

        if consume.is_free:
            actual_chapters.append(chapter)
            continue

        should_purchase = auto_purchase
        if not auto_purchase:
            should_purchase = console.confirm(
                f"Chapter [highlight]{chapter.chapter_title}[/highlight] ({chapter.id}) need to be purchased "
                f"for {consume!r}, continue?",
            )

        if should_purchase:
            console.info(
                f"Purchasing chapter [highlight]{chapter.chapter_title}[/highlight] ({chapter.id}) "
                f"with consumption [bold]{consume!r}[/bold]...",
            )
            _t = client.get_chapter_images(chapter.id, coins=consume)
            if not _t.blocks:
                console.error(f"Unable to purchase chapter {chapter.chapter_title} ({chapter.id}), skipping")
                continue

            actual_chapters.append(chapter)
            coin_purse.paid -= consume.paid
            coin_purse.event -= consume.event
            coin_purse.free -= consume.free

    if not actual_chapters:
        console.warning("No chapters to download after filtering, aborting")
        return

    title_dir = get_output_directory(output_dir, title_id)
    (title_dir / "_info.json").write_bytes(create_chapters_info(result))

    for chapter in actual_chapters:
        console.info(f"  Downloading chapter [highlight]{chapter.chapter_title}[/highlight] ({chapter.id})...")
        ch_images = client.get_chapter_images(
            chapter.id,
            coins=ConsumeCoin(0, 0, 0, 0),
            quality=image_quality,
        )
        if not ch_images.blocks:
            console.error(f"   Unable to download chapter {chapter.chapter_title} ({chapter.id}), skipping [no blocks]")
            continue

        CH_IMGS = get_output_directory(output_dir, title_id, chapter.id, skip_create=True)
        if len(ch_images.blocks) > 1:
            console.warning(
                f"   Chapter [bold]{chapter.chapter_title}[/bold] ({chapter.id}) has more than 1 block, "
                "please report this to the developer!"
            )
            continue

        images_blocks = ch_images.blocks[0].images
        if not images_blocks:
            console.error(f"   Unable to download chapter {chapter.chapter_title} ({chapter.id}), skipping [no images]")
            continue

        if CH_IMGS.exists() and len(list(CH_IMGS.glob("*.avif"))) >= len(images_blocks):
            console.warning(
                f"   Chapter [bold]{chapter.chapter_title}[/bold] ({chapter.id}) already downloaded, skipping",
            )
            continue

        CH_IMGS.mkdir(parents=True, exist_ok=True)
        for image in images_blocks:
            img_dl_path = CH_IMGS / f"p{int(image.stem):03d}.{image.extension}"
            console.info(f"   Downloading image [bold]{image.filename}[/bold] to [bold]{img_dl_path.name}[/bold]...")
            with img_dl_path.open("wb") as fpw:
                for img_bita in client.stream_download(image.url):
                    if img_bita:
                        fpw.write(img_bita)


@click.command(
    name="autodownload",
    help="Automatically/batch download manga chapter(s) for a title",
    cls=ToshoMangoCommandHandler,
)
@click.argument("title_id", type=int, metavar="TITLE_ID", required=True)
@options.output_dir
@click.option(
    "-q",
    "--quality",
    "image_quality",
    type=click.Choice(Quality),  # type: ignore
    help="Specify the image quality to download",
    default=Quality.HIGH,
    show_default=True,
)
@click.option(
    "-ff",
    "--force-free",
    "opt_force_free",
    is_flag=True,
    default=False,
    help="Force to only download free/purchased chapters",
)
@click.option(
    "-np",
    "--no-purchase",
    "opt_no_purchase",
    is_flag=True,
    default=False,
    help="Do not purchase chapter if possible",
)
@click.option(
    "-nxp",
    "--no-xp",
    "opt_no_use_xp",
    is_flag=True,
    default=False,
    help="Do not use XP to purchase chapter if possible",
)
@click.option(
    "-sf",
    "--start-from",
    "start_from",
    type=click.IntRange(min=1),
    default=None,
    help="Start downloading from chapter ID",
)
@click.option(
    "-ef",
    "--end-until",
    "end_until",
    type=click.IntRange(min=1),
    default=None,
    help="Stop downloading until chapter ID",
)
@options.account_id
def musq_manga_auto_download(
    title_id: int,
    output_dir: Path,
    image_quality: Quality = Quality.HIGH,
    opt_force_free: bool = False,
    opt_no_purchase: bool = False,
    opt_no_use_xp: bool = False,
    start_from: int | None = None,
    end_until: int | None = None,
    account_id: str | None = None,
):
    if start_from is not None and end_until is not None and start_from > end_until:
        raise click.BadParameter("Start chapter ID cannot be greater than end chapter ID")

    account = select_single_account(account_id)
    if account is None:
        console.warning("Aborted")
        return

    console.info(f"Getting manga ID [highlight]{title_id}[/highlight]...")
    client = make_client(account)

    try:
        result = client.get_manga(title_id)
    except HTTPError as e:
        console.error(f"Unable to connect to MU!: {e}")
        return

    coin_purse = result.user_point
    manga_chapters = result.chapters
    manga_chapters.sort(key=lambda x: x.id)

    if opt_force_free:
        coin_purse.paid = 0
        coin_purse.event = 0
    if opt_no_purchase:
        coin_purse.paid = 0
        coin_purse.event = 0
        coin_purse.free = 0
    if opt_no_use_xp:
        coin_purse.event = 0

    consumptions_list: list[ConsumeCoin] = []
    consume_chapters: list[ChapterV2] = []
    # Precalculate coin consumption
    for chapter in manga_chapters:
        if start_from is not None and chapter.id < start_from:
            console.warning(f" Skipping chapter {chapter.chapter_title} ({chapter.id}) due to start from option")
            continue
        if end_until is not None and chapter.id > end_until:
            break
        consume = client.calculate_coin(coin_purse, chapter)
        consumptions_list.append(consume)
        consume_chapters.append(chapter)
    consumpted_total = sum(x.need for x in consumptions_list)
    console.info(f" Total chapters: [bold]{len(consume_chapters)}[/bold]")
    console.info(f" Expected coins needed: [bold]{consumpted_total}[/bold]")

    title_dir = get_output_directory(output_dir, title_id)
    (title_dir / "_info.json").write_bytes(create_chapters_info(result))

    for chapter in consume_chapters:
        console.info(f"  Downloading chapter [highlight]{chapter.chapter_title}[/highlight] ({chapter.id})...")

        CH_IMGS = get_output_directory(output_dir, title_id, chapter.id, skip_create=True)
        if CH_IMGS.exists():
            console.warning(
                f"   Chapter [bold]{chapter.chapter_title}[/bold] ({chapter.id}) already downloaded, skipping",
            )
            continue

        consume = client.calculate_coin(coin_purse, chapter)
        if not consume.is_possible():
            console.warning(
                f"   Chapter [highlight]{chapter.chapter_title}[/highlight] ([bold]{chapter.id}[/bold]) is not "
                "available for purchase, skipping",
            )
            console.warning(f"    Need {consume.free} free coin, {consume.event} coin, and {consume.paid} paid coin")
            continue

        coin_purse.free -= consume.free
        coin_purse.event -= consume.event
        coin_purse.paid -= consume.paid
        ch_images = client.get_chapter_images(
            chapter.id,
            coins=consume,
            quality=image_quality,
        )
        if not ch_images.blocks:
            console.error(f"   Unable to download chapter {chapter.chapter_title} ({chapter.id}), skipping [no blocks]")
            continue

        if len(ch_images.blocks) > 1:
            console.warning(
                f"   Chapter [bold]{chapter.chapter_title}[/bold] ({chapter.id}) has more than 1 block, "
                "please report this to the developer!"
            )
            continue

        images_blocks = ch_images.blocks[0].images
        if not images_blocks:
            console.error(f"   Unable to download chapter {chapter.chapter_title} ({chapter.id}), skipping [no images]")
            continue

        CH_IMGS.mkdir(parents=True, exist_ok=True)
        for image in images_blocks:
            img_dl_path = CH_IMGS / f"p{int(image.stem):03d}.{image.extension}"
            console.info(f"   Downloading image [bold]{image.filename}[/bold] to [bold]{img_dl_path.name}[/bold]...")
            with img_dl_path.open("wb") as fpw:
                for img_bita in client.stream_download(image.url):
                    if img_bita:
                        fpw.write(img_bita)
