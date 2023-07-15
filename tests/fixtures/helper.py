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
from typing import Generic, TypeVar

CURRENT_DIR = Path(__file__).absolute().parent

SourceT = TypeVar("SourceT")


class Fixtureable(Generic[SourceT]):
    fixture_name: str | None = None

    def __init_subclass__(cls) -> None:
        if cls.test_fixture != Fixtureable.test_fixture:
            raise ValueError("You are not allowed to override test_fixture method")
        if cls._get_fixture != Fixtureable._get_fixture:
            raise ValueError("You are not allowed to override `_get_fixture` method")
        if cls._run_fixture != Fixtureable._run_fixture:
            raise ValueError("You are not allowed to override `_run_fixture` method")
        if cls._cleanup != Fixtureable._cleanup:
            raise ValueError("You are not allowed to override `_cleanup` method")

        if cls.process == Fixtureable.process:
            raise NotImplementedError("You must override `process` method")

    def _get_fixture(self):
        if self.fixture_name is None:
            raise ValueError("Fixture name is not defined")
        fixture_path = CURRENT_DIR / f"{self.fixture_name}"
        if not fixture_path.exists():
            raise FileNotFoundError(f"Fixture path {self.fixture_name} is not found")

        source = fixture_path / "source.tmfxture"
        expcets = fixture_path / "expects.tmfxture"
        if not source.exists():
            raise FileNotFoundError(f"Fixture source {source} is not found")

        return source, expcets

    def _run_fixture(self):
        source, expcets = self._get_fixture()
        proc_result = self.process(source)
        return proc_result, expcets

    def _cleanup(self):
        fixture_path = CURRENT_DIR / f"{self.fixture_name}"
        if not fixture_path.exists():
            raise FileNotFoundError(f"Fixture path {self.fixture_name} is not found")

        intermediate = fixture_path / "intermediate"
        intermediate_fx = fixture_path / "intermediate.tmfxture"
        intermediate.unlink(missing_ok=True)
        intermediate_fx.unlink(missing_ok=True)

    def process(self, source: Path) -> SourceT:
        raise NotImplementedError

    def assertion_test(self, result: SourceT):
        raise NotImplementedError

    def test_fixture(self):
        result, expect_path = self._run_fixture()
        self._cleanup()
        if expect_path.exists():
            expect_bytes = expect_path.read_bytes()
            assert result == expect_bytes
        else:
            try:
                self.assertion_test(result)
            except NotImplementedError as ner:
                raise FileNotFoundError(
                    f"Fixture expects {expect_path} is not found, and assertion_test is not overriden"
                ) from ner
