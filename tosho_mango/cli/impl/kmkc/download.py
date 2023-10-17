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
from tosho_mango.sources.kmkc.config import KMConfigWeb
from tosho_mango.sources.kmkc.constants import CDN_HOST
from tosho_mango.sources.kmkc.dto import (
    EpisodeEntry,
    PremiumTicketInfo,
    TitleNode,
    TitleTicketInfo,
    UserPoint,
)
from tosho_mango.sources.kmkc.errors import KMAPIError, KMNotEnoughPointError
from tosho_mango.sources.kmkc.imaging import bytes_to_image, descramble_target

from .. import options
from .common import make_web_client, select_single_account

__all__ = (
    "kmkc_title_download",
    "kmkc_title_auto_download",
)
console = term.get_console()


def create_chapters_info(manga_detail: TitleNode, chapters: list[EpisodeEntry] | None = None) -> bytes:
    chapters_dump: list[models.ChapterDetailDump] = []
    if chapters:
        for chapter in chapters:
            ch_dump = models.ChapterDetailDump(id=chapter.episode_id, main_name=chapter.episode_name)
            ch_dump.timestamp = int(chapter.start_time_datetime().timestamp())
            chapters_dump.append(ch_dump)
    return msgspec.json.format(
        msgspec.json.encode(
            models.MangaDetailDump(manga_detail.title_name, manga_detail.author_text, chapters_dump),
        ),
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
@options.title_id
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
@options.account_id
def kmkc_title_download(
    title_id: int,
    output_dir: Path,
    chapter_ids: list[int] | None = None,
    show_all: bool = False,
    auto_purchase: bool = False,
    account_id: str | None = None,
):
    # Remove chapters dupe
    if chapter_ids is not None:
        chapter_ids = list(dict.fromkeys(chapter_ids))

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
        user_wallet = client.get_user_point().point
    except KMAPIError as exc:
        console.error(f"Failed to get user wallet: {exc}")
        return

    console.info(f"Getting title ID for [highlight]{title_id}[/highlight]...")
    try:
        result = client.get_title_list([title_id])[0]
    except KMAPIError as exc:
        console.error(f"Failed to get title ID: {exc}")
        return
    except IndexError:
        console.error(f"Failed to get title ID: Title ID {title_id} not found")
        return

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

    if not chapters_info:
        console.error("No chapters found")
        return

    chapters_info.sort(key=lambda x: x.episode_id)

    if chapter_ids is None:
        select_choices = [
            term.ConsoleChoice(str(chapter.episode_id), chapter.episode_name)
            for chapter in chapters_info
            if chapter.available() and not show_all
        ]
        selected = console.select("Select chapter to download", select_choices)
        if not selected:
            console.warning("No chapter selected, aborting")
            return

        chapter_ids = list(map(lambda x: int(x.name), selected))

    console.info(f"Downloading {len(chapter_ids)} chapters...")
    # Precalculate chapters consumption
    ids_lists = [chapter.episode_id for chapter in chapters_info]
    actual_chapters: list[EpisodeEntry] = []
    _wallet_copy = msgspec.json.decode(msgspec.json.encode(user_wallet), type=UserPoint)
    for ch_id in chapter_ids:
        if ch_id not in ids_lists:
            console.warning(f"Episode ID [bold]{ch_id}[/bold] is not available, skipping")
            continue

        id_index = ids_lists.index(ch_id)
        chapter = chapters_info[id_index]
        if chapter.available():
            actual_chapters.append(chapter)
            continue

        should_purchase = auto_purchase
        if not auto_purchase:
            should_purchase = console.confirm(
                f"Chapter [highlight]{chapter.episode_name}[/highlight] ({chapter.episode_id}) need to be purchased "
                f"for {chapter.point}P, continue?",
            )

        if should_purchase:
            if chapter.ticketable():
                ticket: PremiumTicketInfo | TitleTicketInfo | None = None
                if ticket_entry.premium_available():
                    ticket = ticket_entry.ticket_info.premium_ticket_info
                    ticket_entry.subtract_premium()
                elif ticket_entry.title_available():
                    ticket = ticket_entry.ticket_info.title_ticket_info
                    ticket_entry.subtract_title()
                if ticket is not None:
                    console.info(f"Using ticket to purchase: [highlight]{chapter.episode_name}[/highlight]")
                    try:
                        client.claim_episode_with_ticket(chapter.episode_id, ticket)
                        actual_chapters.append(chapter)
                    except KMAPIError as exc:
                        console.warning(f"Failed to purchase chapter, ignoring: {exc}")

            if not _wallet_copy.can_purchase(chapter.point):
                console.warning(
                    f"Chapter [highlight]{chapter.episode_name}[/highlight] ([bold]{chapter.episode_id}[/bold]) is "
                    "not available for purchase, skipping",
                )
                warn_info = f"Need {chapter.point} point"
                if chapter.ticketable():
                    warn_info += " or ticket"
                console.warning(warn_info)
                continue

            try:
                user_wallet = client.claim_episode_with_point(chapter, user_wallet)
                _wallet_copy.subtract(chapter.point)
                _wallet_copy.add(chapter.bonus_point)
                actual_chapters.append(chapter)
            except KMNotEnoughPointError:
                console.warning(f"Not enough point to purchase chapter: [bold]{chapter.episode_name}[/bold]")
            except KMAPIError as exc:
                console.warning(f"Failed to purchase chapter, ignoring: {exc}")

    if not actual_chapters:
        console.warning("No chapters to download after filtering, aborting")
        return

    title_dir = get_output_directory(output_dir, title_id)
    (title_dir / "_info.json").write_bytes(create_chapters_info(result, chapters_info))

    for chapter in actual_chapters:
        console.info(f"  Downloading chapter [highlight]{chapter.episode_name}[/highlight] ({chapter.episode_id})...")
        try:
            viewer_info = client.get_chapter_viewer(chapter.episode_id)
        except KMAPIError as exc:
            console.warning(f"Failed to get chapter viewer information, ignoring: {exc}")
            continue

        console.log(f"    Has: {len(viewer_info.page_list)} pages")
        console.log(f"    Seed: {viewer_info.scramble_seed}")

        CH_IMGS = get_output_directory(output_dir, title_id, chapter.episode_id, skip_create=True)
        if CH_IMGS.exists() and len(list(CH_IMGS.glob("*.png"))) >= len(viewer_info.page_list):
            console.warning(
                f"   Chapter [bold]{chapter.episode_name}[/bold] ({chapter.episode_id}) already downloaded, skipping",
            )
            continue

        CH_IMGS.mkdir(parents=True, exist_ok=True)
        for idx, image in enumerate(viewer_info.page_list):
            IMG_TARGET = CH_IMGS / f"p{idx:03d}.png"
            console.info(f"   Downloading image [bold]{IMG_TARGET.name}[/bold]...")

            try:
                img_req = client.client.get(
                    image,
                    headers={
                        "Host": CDN_HOST,
                    },
                )
                img_req.raise_for_status()
            except HTTPError as exc:
                console.warning(f"    Failed to download image, stopping: {exc}")
                break

            img = bytes_to_image(img_req.content)
            img_descram = descramble_target(img, 4, viewer_info.scramble_seed)
            img_descram.save(IMG_TARGET)


@click.command(
    name="autodownload",
    help="Automatically/batch download manga chapter(s) for a title",
    cls=ToshoMangoCommandHandler,
)
@click.argument("title_id", type=int, metavar="TITLE_ID", required=True)
@options.output_dir
@click.option(
    "-nt",
    "--no-ticket",
    "opt_no_ticket",
    is_flag=True,
    default=False,
    help="Force to only download using point",
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
def kmkc_title_auto_download(
    title_id: int,
    output_dir: Path,
    opt_no_ticket: bool = False,
    opt_no_purchase: bool = False,
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
    if not isinstance(account, KMConfigWeb):
        console.error("Only web account is supported for now!")
        return

    client = make_web_client(account=account)

    console.info(f"Getting user point for [highlight]{account.username}[/highlight]...")
    try:
        user_wallet = client.get_user_point().point
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

    actual_chapters: list[EpisodeEntry] = []
    _wallet_copy = msgspec.json.decode(msgspec.json.encode(user_wallet), type=UserPoint)
    console.info(f"Prepurchasing chapters for [highlight]{result.title_name}[/highlight]")
    for chapter in chapters_info:
        if chapter.available():
            actual_chapters.append(chapter)
            continue

        if start_from is not None and chapter.episode_id < start_from:
            console.warning(f" Skipping chapter {chapter.episode_name} ({chapter.episode_id}) due to start from option")
            continue

        if end_until is not None and chapter.episode_id > end_until:
            break

        if opt_no_purchase:
            continue

        if chapter.ticketable() and not opt_no_ticket:
            ticket: PremiumTicketInfo | TitleTicketInfo | None = None
            if ticket_entry.premium_available():
                ticket = ticket_entry.ticket_info.premium_ticket_info
                ticket_entry.subtract_premium()
            elif ticket_entry.title_available():
                ticket = ticket_entry.ticket_info.title_ticket_info
                ticket_entry.subtract_title()
            if ticket is not None:
                console.info(f"Using ticket to purchase: [highlight]{chapter.episode_name}[/highlight]")
                try:
                    client.claim_episode_with_ticket(chapter.episode_id, ticket)
                    actual_chapters.append(chapter)
                except KMAPIError as exc:
                    console.warning(f"Failed to purchase chapter, ignoring: {exc}")

        if not _wallet_copy.can_purchase(chapter.point):
            console.warning(
                f"Chapter [highlight]{chapter.episode_name}[/highlight] ([bold]{chapter.episode_id}[/bold]) is "
                "not available for purchase, skipping",
            )
            warn_info = f"Need {chapter.point} point"
            if chapter.ticketable():
                warn_info += " or ticket"
            console.warning(warn_info)
            continue

        try:
            user_wallet = client.claim_episode_with_point(chapter, user_wallet)
            _wallet_copy.subtract(chapter.point)
            _wallet_copy.add(chapter.bonus_point)
            actual_chapters.append(chapter)
        except KMNotEnoughPointError:
            console.warning(f"Not enough point to purchase chapter: [bold]{chapter.episode_name}[/bold]")
        except KMAPIError as exc:
            console.warning(f"Failed to purchase chapter, ignoring: {exc}")

    if not actual_chapters:
        console.warning("No chapters to download after filtering, aborting")
        return

    title_dir = get_output_directory(output_dir, title_id)
    (title_dir / "_info.json").write_bytes(create_chapters_info(result, chapters_info))

    for chapter in actual_chapters:
        console.info(f"  Downloading chapter [highlight]{chapter.episode_name}[/highlight] ({chapter.episode_id})...")
        try:
            viewer_info = client.get_chapter_viewer(chapter.episode_id)
        except KMAPIError as exc:
            console.warning(f"Failed to get chapter viewer information, ignoring: {exc}")
            continue

        console.log(f"    Has: {len(viewer_info.page_list)} pages")
        console.log(f"    Seed: {viewer_info.scramble_seed}")

        CH_IMGS = get_output_directory(output_dir, title_id, chapter.episode_id, skip_create=True)
        if CH_IMGS.exists():
            console.warning(
                f"   Chapter [bold]{chapter.episode_name}[/bold] ({chapter.episode_id}) already downloaded, skipping",
            )
            continue

        CH_IMGS.mkdir(parents=True, exist_ok=True)
        for idx, image in enumerate(viewer_info.page_list):
            IMG_TARGET = CH_IMGS / f"p{idx:03d}.png"
            console.info(f"   Downloading image [bold]{IMG_TARGET.name}[/bold]...")

            try:
                img_req = client.client.get(
                    image,
                    headers={
                        "Host": CDN_HOST,
                    },
                )
                img_req.raise_for_status()
            except HTTPError as exc:
                console.warning(f"    Failed to download image, stopping: {exc}")
                break

            img = bytes_to_image(img_req.content)
            img_descram = descramble_target(img, 4, viewer_info.scramble_seed)
            img_descram.save(IMG_TARGET)
