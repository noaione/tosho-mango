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

from tosho_mango import term
from tosho_mango.sources.kmkc.client import KMClientWeb
from tosho_mango.sources.kmkc.config import KMConfigDeviceType, KMConfigWeb, get_all_config, get_config
from tosho_mango.sources.kmkc.constants import BASE_HOST
from tosho_mango.sources.kmkc.dto import TitleNode

__all__ = (
    "select_single_account",
    "make_web_client",
)
console = term.get_console()


def select_single_account(account_id: str | None = None):
    if account_id is not None:
        config = get_config(account_id)
        if config is not None:
            return config
        console.warning(f"Account ID {account_id!r} not found!")

    all_configs = get_all_config()
    if not all_configs:
        raise RuntimeError("No authenticated account found! Register with `tosho mu auth`")

    if len(all_configs) == 1:
        return all_configs[0]

    select = console.choice(
        "Select an account",
        [term.ConsoleChoice(acc.id, f"{acc.id} [{KMConfigDeviceType(acc.type).name}]") for acc in all_configs],
    )

    for acc in all_configs:
        if select.name == acc.id:
            return acc
    raise RuntimeError("This should never happen!")


def make_web_client(account: KMConfigWeb):
    return KMClientWeb(account)


def do_print_search_information(results: list[TitleNode], *, numbering: bool = False):
    for idx, result in enumerate(results, 1):
        manga_url = f"https://{BASE_HOST}/title/{result.title_id}"
        manga_text = f"[bold][link={manga_url}]{result.title_name}[/link][/bold] ({result.title_id})"
        if result.next_updated_text:
            manga_text += f" [[orange]{result.next_updated_text}[/orange]]"
        if result.new_episode_update_cycle_text:
            manga_text += f" [[gold]{result.new_episode_update_cycle_text}[/gold]]"
        if numbering:
            manga_text = f"[{idx:02d}] {manga_text}"
        console.info(f"  {manga_text}")
        console.info(f"   {manga_url}")
