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
from PIL import Image

from tests.fixtures.helper import Fixtureable
from tosho_mango.sources.kmkc.imaging import (
    bytes_to_image,
    calc_block_size,
    descramble_target,
    generate_copy_targets,
    seed_generator,
    uint32,
)


class TestImageDescramble(Fixtureable):
    fixture_name = "image_descramble"

    def process(self, source: Path) -> bytes:
        img_bytes = bytes_to_image(source.read_bytes())
        results = descramble_target(img_bytes, 4, 749191485)
        target_res = source.parent / "intermediate"
        results.save(target_res, "PNG", optimize=True)
        return target_res.read_bytes()

    def test_bytes_to_image(self):
        source, _ = self._get_fixture()
        img = bytes_to_image(source.read_bytes())
        assert img.height == 1378

    def test_descramble_too_small(self):
        with pytest.raises(ValueError):
            im_test = Image.new("RGB", (1, 1))
            descramble_target(im_test, 4, 749191485)


def test_calc_block_size():
    width = 960
    height = 1378

    block_size = calc_block_size(width, height, 4)
    assert (240, 344) == block_size


def test_calc_block_size_small():
    assert (None, None) == calc_block_size(1, 1, 4)


def test_uint32_clamp():
    assert 4294897876 == uint32(-69420)
    assert 10 == uint32(10)


def test_seed_generator():
    rectbox = 4
    seeder = seed_generator(749191485)
    seeds_gen = []
    for _ in range(rectbox):
        seeds_gen.append(next(seeder))
    assert [1149330653, 1799678672, 902605375, 2984793402] == seeds_gen


def test_generate_copy_targets():
    all_copy_targets = list(generate_copy_targets(4, 749191485))
    expect_targets = [
        ((1, 1), (0, 0)),
        ((2, 0), (1, 0)),
        ((3, 1), (2, 0)),
        ((0, 0), (3, 0)),
        ((3, 2), (0, 1)),
        ((0, 2), (1, 1)),
        ((1, 3), (2, 1)),
        ((1, 0), (3, 1)),
        ((0, 3), (0, 2)),
        ((3, 0), (1, 2)),
        ((2, 1), (2, 2)),
        ((0, 1), (3, 2)),
        ((3, 3), (0, 3)),
        ((2, 3), (1, 3)),
        ((2, 2), (2, 3)),
        ((1, 2), (3, 3)),
    ]

    assert expect_targets == all_copy_targets
