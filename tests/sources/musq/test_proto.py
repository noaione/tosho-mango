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
from typing import Type, TypeVar

from betterproto import Message

from tests.fixtures.helper import Fixtureable
from tosho_mango.sources.musq.proto import (
    ChapterViewer,
    ChapterViewerV2,
    HomeView,
    MangaDetail,
    MangaDetailV2,
    MyPageView,
    PointShopHistory,
    PointShopView,
)

ProtoT = TypeVar("ProtoT", bound="Message")


def _proto_read(source: Path, cls: Type[ProtoT]) -> ProtoT:
    hex_bytes = source.read_bytes()
    hexed = bytes.fromhex(hex_bytes.decode("utf-8"))
    return cls.FromString(hexed)


class TestChapterView(Fixtureable[ChapterViewer]):
    fixture_name = "musq_chapterview"

    def process(self, source: Path):
        return _proto_read(source, ChapterViewer)

    def assertion_test(self, result: ChapterViewer):
        assert result.status == 0

        assert result.user_point.free == 0
        assert result.user_point.event == 0
        assert result.user_point.paid == 480
        assert result.user_point.total_point == 480
        assert len(result.images) == 11
        assert result.previous_chapter is None
        assert result.next_chapter is not None

        image = result.images[0]
        assert image.filename == "1.avif"
        assert image.stem == "1"
        assert image.extension == "avif"

        image2 = result.images[1]
        image2.url = "/data/1/noextension"
        assert image2.extension == ""


class TestChapterViewV2(Fixtureable[ChapterViewerV2]):
    fixture_name = "musq_chapterviewv2"

    def process(self, source: Path):
        return _proto_read(source, ChapterViewerV2)

    def assertion_test(self, result: ChapterViewerV2):
        assert result.status == 0

        assert result.user_point.free == 40
        assert result.user_point.event == 0
        assert result.user_point.paid == 370
        assert result.user_point.total_point == 410
        assert len(result.blocks) == 1

        assert result.next_chapter is not None

        block = result.blocks[0]
        assert block.title == "Chapter 10.1"
        image = block.images[0]
        assert image.filename == "1.avif"
        assert image.stem == "1"
        assert image.extension == "avif"

        image2 = block.images[1]
        image2.url = "/data/1/noextension"
        assert image2.extension == ""


class TestPointShopHistory(Fixtureable[PointShopHistory]):
    fixture_name = "musq_coinhistory"

    def process(self, source: Path) -> PointShopHistory:
        return _proto_read(source, PointShopHistory)

    def assertion_test(self, result: PointShopHistory):
        assert result.user_point.free == 0
        assert result.user_point.event == 0
        assert result.user_point.paid == 280

        assert len(result.logs) > 0


class TestHomeViewV2(Fixtureable[HomeView]):
    fixture_name = "musq_homev2"

    def process(self, source: Path) -> HomeView:
        return _proto_read(source, HomeView)

    def assertion_test(self, result: HomeView):
        assert len(result.top_banners) > 0
        assert len(result.top_sub_banners) > 0
        assert result.tutorial_banners is None
        assert result.updated_section_name == "Updates for you"
        assert len(result.updated_titles) > 0
        assert len(result.tags) == 8
        assert result.featured is not None
        assert result.new_titles_section_name == "New Series"
        assert len(result.new_titles) > 0
        assert result.ranking_section_name == "Ranking"
        assert len(result.rankings) == 4
        assert result.ranking_description != ""
        assert result.recommended_banner_image_url != ""


class TestMangaDetail(Fixtureable[MangaDetail]):
    fixture_name = "musq_mangadetail"

    def process(self, source: Path) -> MangaDetail:
        return _proto_read(source, MangaDetail)

    def assertion_test(self, result: MangaDetail):
        assert result.status == 0
        assert result.title == "The Diary of a Middle-Aged Teacher's Carefree Life in Another World"
        assert result.authors == "Kotobuki Yasukiyo, Maneki, Ryu Nishin, Johndee"
        assert result.copyright != ""
        assert result.next_update is None
        assert result.warning is None
        assert result.description != ""
        assert result.display_description is False
        assert len(result.tags) == 1
        assert result.video_url is None

        assert len(result.chapters) > 0
        # Test chapter
        last_chapter = result.chapters[0]
        first_chapter = result.chapters[-1]
        assert first_chapter.name == "Chapter 1.1"
        assert first_chapter.is_free is True
        assert last_chapter.name == "Chapter 44.2"
        assert last_chapter.is_free is False
        assert first_chapter.chapter_title == "Chapter 1.1"
        last_chapter.subtitle = "Test"
        assert last_chapter.chapter_title == "Chapter 44.2 — Test"


class TestMangaDetailV2(Fixtureable[MangaDetailV2]):
    fixture_name = "musq_mangadetailv2"

    def process(self, source: Path) -> MangaDetailV2:
        return _proto_read(source, MangaDetailV2)

    def assertion_test(self, result: MangaDetailV2):
        assert result.status == 0
        assert result.title == "The Angel Next Door Spoils Me Rotten"
        assert result.authors == "Saekisan, Hanekoto, Wan Shibata, Suzu Yuki"
        assert result.copyright != ""
        assert result.next_update is None
        assert result.warning is None
        assert result.description != ""
        assert result.display_description is False
        assert len(result.tags) == 3
        assert result.video_url is None

        assert len(result.chapters) > 0
        # Test chapter
        last_chapter = result.chapters[0]
        first_chapter = result.chapters[-1]
        assert first_chapter.name == "Chapter 1.1"
        assert first_chapter.is_free is True
        assert last_chapter.name == "Chapter 10.3"
        assert last_chapter.is_free is True
        assert first_chapter.chapter_title == "Chapter 1.1"
        last_chapter.subtitle = "Test"
        assert last_chapter.chapter_title == "Chapter 10.3 — Test"

        assert result.hidden_chapters is not None


class TestMyPageView(Fixtureable[MyPageView]):
    fixture_name = "musq_mypage"

    def process(self, source: Path) -> MyPageView:
        return _proto_read(source, MyPageView)

    def assertion_test(self, result: MyPageView):
        assert len(result.favorites) > 0
        assert len(result.history) > 0


class TestPointShopView(Fixtureable[PointShopView]):
    fixture_name = "musq_pointshop"

    def process(self, source: Path) -> PointShopView:
        return _proto_read(source, PointShopView)

    def assertion_test(self, result: PointShopView):
        assert result.user_point.free == 0
        assert result.user_point.event == 0
        assert result.user_point.paid == 480

        assert result.point_limit.free == 40
        assert result.point_limit.event == 100000
        assert result.point_limit.paid == 100000
        assert result.next_recovery == 1674604800
        assert len(result.subscriptions) == 0
        assert len(result.billings) == 9
        assert result.default_select == 0

        billing = result.billings[8]
        assert billing.event_point == 8040
        assert billing.paid_point == 26800
        assert billing.total_point == 34840
