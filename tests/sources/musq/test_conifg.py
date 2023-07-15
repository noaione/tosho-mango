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

from pathlib import Path

import pytest

from tosho_mango.sources.musq.config import MUConfig, MUConfigDevice, get_all_config, get_config, save_config

mocked_path = Path(__file__).absolute().parent.parent.parent.parent / ".config-test"


@pytest.mark.usefixtures("config_id")
def test_load_config(config_id: str):
    config = get_config(config_id, path=mocked_path)
    assert config is not None
    assert config.session == "test"
    assert config.type == MUConfigDevice.ANDROID


@pytest.mark.usefixtures("config_id")
def test_load_all_config(config_id: str):
    configs = get_all_config(mocked_path)
    assert configs is not None
    assert len(configs) == 1
    assert config_id == configs[0].id


@pytest.fixture(name="config_id", scope="module")
def test_config_maker():
    config = MUConfig.from_auth("test", MUConfigDevice.ANDROID)
    assert config.session == "test"
    assert config.type == MUConfigDevice.ANDROID

    save_config(config, path=mocked_path)
    return config.id


def test_missing_config():
    config = get_config("missing", path=mocked_path)
    assert config is None


def setup_module():
    mocked_path.mkdir(exist_ok=True)
    # Empty folder
    for file in mocked_path.iterdir():
        file.unlink(missing_ok=True)


def teardown_module():
    # Empty folder
    for file in mocked_path.iterdir():
        file.unlink(missing_ok=True)
    mocked_path.rmdir()
