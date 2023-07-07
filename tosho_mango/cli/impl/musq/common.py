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

from tosho_mango import term
from tosho_mango.sources.musq.client import MUClient
from tosho_mango.sources.musq.config import MUConfig, MUConfigDevice, get_all_config, get_config
from tosho_mango.sources.musq.constants import DEVICE_CONSTANTS
from tosho_mango.sources.musq.proto import BadgeManga, MangaListNode

__all__ = (
    "select_single_account",
    "make_client",
    "do_print_search_information",
)
console = term.get_console()


def select_single_account(account_id: str | None = None) -> MUConfig | None:
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

    all_choices = [term.ConsoleChoice(acc.id, f"{acc.id} [{MUConfigDevice(acc.type).name}]") for acc in all_configs]
    all_choices.append(term.ConsoleChoice("_cancelrino", "Cancel"))
    select = console.choice(
        "Select an account",
        all_choices,
    )

    if select.name == "_cancelrino":
        return None

    for acc in all_configs:
        if select.name == acc.id:
            return acc
    raise RuntimeError("This should never happen!")


def make_client(account: MUConfig):
    return MUClient(account.session, DEVICE_CONSTANTS[account.type])


def do_print_search_information(results: list[MangaListNode]):
    for result in results:
        badge = BadgeManga(result.badge)
        manga_url = f"https://global.manga-up.com/manga/{result.id}"
        text_data = f"[bold][link={manga_url}]{result.name}[/link][/bold] ({result.id})"
        if badge is badge.NEW:
            text_data = f"{text_data} [bcyan][highr][NEW][/highr][/bcyan]"
        elif badge is BadgeManga.UNREAD:
            text_data = f"{text_data} ([blue][highr]●[/highr][/blue])"
        elif badge is BadgeManga.UPDATE:
            text_data = f"{text_data} ([success][highr]UP[/highr][/success])"
        elif badge is BadgeManga.UPDATE_THIS_WEEK:
            text_data = f"{text_data} ([warning][highr]UP (Week)[/highr][/warning])"
        console.info(f"  {text_data}")
        console.info(f"   {manga_url}")
