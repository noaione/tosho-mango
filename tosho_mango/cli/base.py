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

import sys
import traceback
from functools import partial
from typing import TYPE_CHECKING, List, Optional, Tuple, Union, cast

import click
from click.core import Context
from click.parser import Option as ParserOption
from click.parser import OptionParser

from .. import term

if TYPE_CHECKING:
    from click.parser import ParsingState

console = term.get_console()

__all__ = (
    "WithDeprecatedOption",
    "ToshoMangoCommandHandler",
    "UnrecoverableToshoMangoError",
)


class WithDeprecatedOption(click.Option):
    def __init__(self, *args, **kwargs):
        self.is_deprecated = bool(kwargs.pop("deprecated", False))

        preferred: Optional[Union[Tuple[str], List[str], str]] = kwargs.pop("preferred", None)
        preferred_list = []
        if preferred is not None:
            if isinstance(preferred, str):
                preferred_list = [preferred]
            elif isinstance(preferred_list, (tuple, list)):
                preferred_list: List[str] = []
                for pref in preferred:
                    if not isinstance(pref, str):
                        raise ValueError(f"The following prefered option is not a string! `{pref!r}`")
                    preferred_list.append(pref)
        self.preferred: List[str] = preferred_list
        super(WithDeprecatedOption, self).__init__(*args, **kwargs)

    def get_help_record(self, ctx: Context) -> Optional[Tuple[str, str]]:
        parent = super().get_help_record(ctx)
        if parent is None:
            return parent

        if self.is_deprecated:
            opts_thing, help = parent
            return (opts_thing, f"(DEPRECATED) {help}")
        return parent


class UnrecoverableToshoMangoError(click.ClickException):
    def __init__(self, message, exc_info):
        super().__init__(message)
        self.exc_info = exc_info

    def show(self):
        emoji = ""
        if console.is_advanced():
            emoji = "\u274C "
        console.error(f"*** {emoji}An unrecoverable error occured ***")
        console.error(self.message)
        # Make traceback
        traceback.print_exception(*self.exc_info)


def _fmt_pref_text(preferred: List[str]):
    return "`" + "` or `".join(preferred) + "`"


class ToshoMangoCommandHandler(click.Command):
    def make_parser(self, ctx: Context) -> OptionParser:
        """
        Hook the process of making parser to handle deprecated options.

        https://stackoverflow.com/a/50402799
        """
        parser = super(ToshoMangoCommandHandler, self).make_parser(ctx)

        options = set(parser._short_opt.values())
        options |= set(parser._long_opt.values())

        for option in options:
            if not isinstance(option.obj, WithDeprecatedOption):
                continue

            def make_process(opt: ParserOption):
                orig_process = option.process

                def _process_intercept(
                    value: str,
                    state: "ParsingState",
                    upper_opt: ParserOption,
                    original_func: callable,
                ):
                    is_deprecated = cast(bool, getattr(upper_opt.obj, "is_deprecated", False))
                    preferred = cast(List[str], getattr(upper_opt.obj, "preferred", []))

                    opt_short = upper_opt._short_opts
                    opt_long = upper_opt._long_opts

                    merged_short = "/".join(opt_short)
                    merged_long = "/".join(opt_long)

                    merged_opt = ""
                    if merged_short:
                        merged_opt += merged_short
                    if merged_long:
                        if merged_opt:
                            merged_opt += "/"
                        merged_opt += merged_long

                    if is_deprecated is not None:
                        warn_msg = f"Option `{merged_opt}` is deprecated!"
                        if len(preferred) > 0:
                            warn_msg += f" Use {_fmt_pref_text(preferred)} instead!"
                        else:
                            warn_msg += " This option will be removed in the future!"
                        console.warning(warn_msg)

                    return original_func(value, state)

                return partial(_process_intercept, upper_opt=opt, original_func=orig_process)

            option.process = make_process(option)

        return parser

    def invoke(self, ctx: Context):
        try:
            return super().invoke(ctx)
        except Exception as ex:
            # Invoke error handler
            raise UnrecoverableToshoMangoError(str(ex), sys.exc_info())
