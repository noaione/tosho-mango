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

from tosho_mango.sources.musq.models import ConsumeCoin, WeeklyCode


def test_weekly_code():
    monday = WeeklyCode.MONDAY
    assert monday == "mon"
    assert WeeklyCode.today() in WeeklyCode
    assert monday.indexed == 0


class TestConsumeCoin:
    def test_free(self):
        cc = ConsumeCoin()
        assert cc.is_free is True

    def test_possible(self):
        cc = ConsumeCoin(20, 0, 0, 10)
        assert cc.is_possible() is True

    def test_not_possible(self):
        cc = ConsumeCoin(10, 5, 2, 20)
        assert cc.is_possible() is False
