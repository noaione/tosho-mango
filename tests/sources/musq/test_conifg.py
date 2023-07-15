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
