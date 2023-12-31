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

from types import ModuleType

from click import Group

from tosho_mango.cli.base import ToshoMangoCommandHandler


def auto_import_implementations(group: Group, modules: ModuleType):
    """Automatically import all ToshoMangoCommandHandler from a module.

    Parameters
    ----------
    group : Group
        The group to add the command to.
    modules : ModuleType
        The module to import from.

    Raises
    ------
    AttributeError
        If the module does not have __file__ attribute.
    ValueError
        If the module is not in cli/impl.
    """

    if modules.__file__ is None:
        raise AttributeError("Cannot import module without __file__ attribute")

    mod_path = modules.__file__.replace("\\", "/")
    if "cli/impl" not in mod_path:
        raise ValueError("Cannot import module that is not in cli/impl")

    for name in dir(modules):
        if name.startswith("__"):
            continue
        command_or = getattr(modules, name)
        if isinstance(command_or, ToshoMangoCommandHandler):
            group.add_command(command_or)
