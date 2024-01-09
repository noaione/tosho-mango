## The Rust™ Rewrite™

There is an [ongoing process](https://github.com/noaione/tosho-mango/pull/5) of me rewritting everything into Rust because it's fun \:)

So, there will be no more update to the Python code.<br />
The Python code will be hosted on separate branch here: [`legacy-snek`](https://github.com/noaione/tosho-mango/tree/legacy-snek).

The new Rust version will also introduce some more capabilities like actual login system for some sources. (Legacy "login" will also still be supported because honestly it's way easier to implement and use)

# tosho-mango

[![CI](https://github.com/noaione/tosho-mango/actions/workflows/ci.yml/badge.svg)](https://github.com/noaione/tosho-mango/actions/workflows/ci.yml) ![Codecov](https://img.shields.io/codecov/c/github/noaione/tosho-mango) [![Code style: black](https://img.shields.io/badge/code%20style-black-000000.svg)](https://github.com/psf/black) [![Ruff](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/charliermarsh/ruff/main/assets/badge/v2.json)](https://github.com/astral-sh/ruff) [![License: MIT](https://img.shields.io/github/license/noaione/tosho-mango)](https://github.com/noaione/tosho-mango/blob/master/LICENSE)

A simple downloader for some official mango.

While I did say this is a "downloader", it can also be said it is an almost full-blown replacement of the app/web version
as a CLI application.

All of the implementations here are originally a separate script that I made myself, I decide to open source it because
I'm kinda burned out doing it myself so someone else can do it.

Currently, the authentication system is limited to HTTP Intercepting or downloading cookies.

## Installation

**Requirements**
- Python 3.10+
- [Poetry](https://python-poetry.org/)

1. Clone this repository
2. Go to the directory
3. Type `poetry install`
4. Run `poetry run tosho` to see all available commands

Each sources can be authenticated, to see how each sources can be authenticated, please see the [`sources`](https://github.com/noaione/tosho-mango/tree/master/tosho_mango/sources) folder.

## License

[MIT License](LICENSE)
