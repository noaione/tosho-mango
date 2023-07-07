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

from base64 import b64decode
from typing import Generator, Optional
from urllib.parse import urlparse

import requests

from .constants import API_HOST, IMAGE_HOST, QUALITY_FORMAT, WEEKLY_CODE, ClientConstants
from .models import ConsumeCoin, Quality, WeeklyCode
from .proto import (
    AccountView,
    Chapter,
    ChapterViewer,
    ConsumptionType,
    HomeView,
    MangaDetail,
    MangaList,
    MyPageView,
    PointShopHistory,
    PointShopView,
    SettingView,
    Status,
    UserPoint,
)

__all__ = ("MUClient",)


class MUClient:
    BASE_IMG = b64decode("aHR0cHM6Ly9nbG9iYWwtaW1nLm1hbmdhLXVwLmNvbQ==").decode("utf-8")
    BASE_API = b64decode("aHR0cHM6Ly9nbG9iYWwtYXBpLm1hbmdhLXVwLmNvbS9hcGk=").decode("utf-8")

    def __init__(self, secret: str, client: ClientConstants, *, session: requests.Session | None = None) -> None:
        self._session = session or requests.Session()
        self._client = client
        self._session.headers.update(
            {
                "Host": API_HOST,
                "User-Agent": self._client["_API_UA"],
            }
        )
        self._secret = secret

    # --> Helper Methods

    def close(self):
        self._session.close()

    def _build_param(self, base_param: dict):
        # Important params
        base_param["secret"] = self._secret
        for key, value in self._client.items():
            if key.startswith("_"):
                continue
            base_param[key.lower()] = value
        base_param["lang"] = "en"
        return base_param

    def request(self, method: str, url: str, **kwargs):
        return self._session.request(method, url, **kwargs)

    def _build_coin(
        self,
        need_coin: int,
        free_coin: int,
        event_coin: Optional[int] = None,
        paid_coin: Optional[int] = None,
    ) -> ConsumeCoin:
        event_coin = free_coin if event_coin is None else event_coin
        paid_coin = free_coin if paid_coin is None else paid_coin
        return ConsumeCoin(free=free_coin, event=event_coin, paid=paid_coin, need=need_coin)

    # <-- Helper Methods

    def calculate_coin(self, user_point: UserPoint, chapter: Chapter) -> ConsumeCoin:
        """
        Calculate how many coins you need to get this chapter.

        After using this, I recommend subtracting your current User Point value
        or getting it when you call any other endpoint to update your value.

        Call this before you call :func:`get_chapter_images`.
        Then call :meth:`.is_possible()` method to check if you can get this chapter.

        Parameters
        ----------
        user_point: :class:`UserPoint`
            Your current user point value, you can get it by calling :meth:`get_user_point`.
        chapter: :class:`Chapter`
            The chapter you want to check with.

        Returns
        -------
        :class:`ConsumeCoin`
            The coin dataclass, tell you how much coins you need to get this chapter.
            It will be divided into free, event, and paid coins.
        """
        if chapter.is_free:
            return self._build_coin(0, 0)
        match chapter.consumption:
            case ConsumptionType.ANY_ITEMS:
                # Prioritization: Free > Event > Paid
                free = user_point.free
                event = user_point.event
                paid = user_point.paid
                cprice = max(chapter.price - free, 0)
                if cprice <= 0:
                    return self._build_coin(chapter.price, chapter.price, 0, 0)
                cprice = max(cprice - event, 0)
                if cprice <= 0:
                    event_diff = chapter.price - free
                    return self._build_coin(chapter.price, free, event_diff, 0)
                cprice = max(cprice - paid, 0)
                paid_diff = max(chapter.price - free - event, 0)
                if cprice > 0:
                    # Even after paid, still not enough
                    paid_diff = paid
                return self._build_coin(chapter.price, free, event, paid_diff)
            case ConsumptionType.EVENT_OR_PAID:
                event = user_point.event
                paid = user_point.paid
                cprice = max(chapter.price - event, 0)
                if cprice <= 0:
                    return self._build_coin(chapter.price, 0, event, 0)
                cprice = max(cprice - paid, 0)
                paid_diff = max(chapter.price - event, 0)
                if cprice > 0:
                    # Even after paid, still not enough
                    paid_diff = paid
                return self._build_coin(chapter.price, 0, event, paid_diff)
            case ConsumptionType.PAID_ONLY:
                paid_left = user_point.paid - chapter.price
                if paid_left < 0:
                    return self._build_coin(chapter.price, 0, 0, 0)
                return self._build_coin(chapter.price, 0, 0, chapter.price)
            case _:
                raise ValueError(f"Invalid consumption type: {chapter.consumption}")

    # <-- Helper Methods

    # --> PointEndpoints.kt

    def get_user_point(self) -> UserPoint:
        """
        Get your current user point.

        Returns
        -------
        :class:`UserPoint`
            Your current user point.

        Raises
        ------
        :exc:`requests.HTTPError`
            If the request failed.
        """
        params = self._build_param({})
        r = self.request("GET", f"{self.BASE_API}/point/shop", params=params)
        r.raise_for_status()
        psv = PointShopView.FromString(r.content)
        return psv.user_point

    def get_point_history(self) -> PointShopHistory:
        """
        Get your point acquisition history.

        Returns
        -------
        :class:`PointShopHistory`
            Your point acquisition history.

        Raises
        ------
        :exc:`requests.HTTPError`
            If the request failed.
        """
        params = self._build_param({})
        r = self.request("GET", f"{self.BASE_API}/point/history", params=params)
        r.raise_for_status()
        psv = PointShopHistory.FromString(r.content)
        return psv

    # <-- PointEndpoints.kt

    # --> MangaEndpoints.kt

    def get_manga(self, manga_id: int):
        """
        Get manga detail information.

        Parameters
        ----------
        manga_id: :class:`int`
            The manga ID.

        Returns
        -------
        :class:`MangaDetail`
            The manga detail information.

        Raises
        ------
        :exc:`RuntimeError`
            If the server returns error.
        :exc:`requests.HTTPError`
            If the request failed.
        """
        params = self._build_param(
            {
                "title_id": str(manga_id),
                "ui_lang": "en",
            }
        )

        r = self.request("GET", f"{self.BASE_API}/manga/detail", params=params)
        r.raise_for_status()
        manga = MangaDetail.FromString(r.content)
        if manga.status != Status.SUCCESS:
            raise RuntimeError(f"Failed to get manga {manga_id}: {Status(manga.status).name}")
        return manga

    def get_weekly_titles(self, week: WeeklyCode) -> MangaList:
        """
        Get weekly manga list.

        Parameters
        ----------
        week: :class:`WeeklyCode`
            The weekly code, can be ``"mon"`` for Monday, ``"tue"`` for Tuesday, and so on.

        Returns
        -------
        :class:`MangaList`
            The manga list.

        Raises
        ------
        :exc:`ValueError`
            If the weekly code is invalid.
        :exc:`requests.HTTPError`
            If the request failed.
        """
        if week.value not in WEEKLY_CODE:
            valid_types: str = ", ".join(WEEKLY_CODE)
            raise ValueError(f"Invalid weekly code: {week}, valid types: {valid_types}")

        params = self._build_param(
            {
                "code": week.value,
            }
        )

        r = self.request("GET", f"{self.BASE_API}/manga/weekly", params=params)
        r.raise_for_status()
        manga_list = MangaList.FromString(r.content)
        return manga_list

    def search_manga_by_tag(self, tag_id: int) -> MangaList:
        """
        Search manga by tag.

        Parameters
        ----------
        tag_id: :class:`int`
            The tag ID.

        Returns
        -------
        :class:`MangaList`
            The manga list.

        Raises
        :exc:`requests.HTTPError`
            If the request failed.
        """
        params = self._build_param(
            {
                "tag_id": tag_id,
            }
        )

        r = self.request("GET", f"{self.BASE_API}/manga/tag", params=params)
        r.raise_for_status()
        manga_list = MangaList.FromString(r.content)
        return manga_list

    def search_manga(self, query: str) -> MangaList:
        """
        Search manga by query.

        Parameters
        ----------
        query: :class:`str`
            The search query.

        Returns
        -------
        :class:`MangaList`
            The matching manga in list.

        Raises
        :exc:`requests.HTTPError`
            If the request failed.
        """
        params = self._build_param(
            {
                "word": query,
            }
        )

        r = self.request("GET", f"{self.BASE_API}/manga/search", params=params)
        r.raise_for_status()
        manga_list = MangaList.FromString(r.content)
        return manga_list

    def get_chapter_images(
        self,
        chapter_id: int,
        *,
        coins: ConsumeCoin = ConsumeCoin(),
        quality: Quality = Quality.HIGH,
    ) -> ChapterViewer:
        """
        Get chapter viewer that contains images.

        Parameters
        ----------
        chapter_id: :class:`int`
            The chapter ID.
        coins: :class:`ConsumeCoin`
            The coin dataclass, tell you how much coins you need to use to get this chapter.
            Use :meth:`calculate_coin` to get this.
        quality: :class:`Quality`
            The image quality, can be ``high`` or ``medium``.

        Returns
        -------
        :class:`ChapterViewer`
            The chapter viewer that contains the images.

        Raises
        ------
        :exc:`ValueError`
            If the quality is invalid.
        :exc:`ValueError`
            If you don't have enough coins.
        :exc:`RuntimeError`
            If the server returns error.
        """
        if quality.value not in QUALITY_FORMAT:
            raise ValueError(f"Invalid quality format: {quality}")
        if not coins.is_possible():
            raise ValueError(f"Insufficient coins: {coins}")
        params = self._build_param(
            {
                "chapter_id": str(chapter_id),
                "free_point": str(coins.free),
                "event_point": str(coins.event),
                "paid_point": str(coins.paid),
                "quality": quality.value,
            }
        )

        r = self.request("POST", f"{self.BASE_API}/manga/viewer", params=params)
        r.raise_for_status()
        ch_view = ChapterViewer.FromString(r.content)
        if ch_view.status != Status.SUCCESS:
            raise RuntimeError(f"Failed to get chapter view {chapter_id}: {Status(ch_view.status).name}")
        return ch_view

    # <-- MangaEndpoints.kt

    # --> AccountEndpoints.kt

    def get_account(self) -> AccountView:
        """
        Get your account information.

        Returns
        -------
        :class:`AccountView`
            Your account information.

        Raises
        ------
        :exc:`requests.HTTPError`
            If the request failed.
        """
        params = self._build_param({})
        r = self.request("GET", f"{self.BASE_API}/account/account", params=params)
        r.raise_for_status()
        acc_view = AccountView.FromString(r.content)
        return acc_view

    def get_setting(self) -> SettingView:
        """
        Get your account setting.

        Returns
        -------
        :class:`SettingView`
            Your account setting.

        Raises
        ------
        :exc:`requests.HTTPError`
            If the request failed.
        """
        params = self._build_param({})
        r = self.request("GET", f"{self.BASE_API}/setting/setting", params=params)
        r.raise_for_status()
        setting_view = SettingView.FromString(r.content)
        return setting_view

    # --> Api.kt (Personalized)

    def get_my_manga(self) -> MyPageView:
        """
        Get your profile list of manga.

        Including History, and Bookmarks/Favorites.

        Returns
        -------
        :class:`MyPageView`
            Your personalized manga list.

        Raises
        :exc:`requests.HTTPError`
            If the request failed.
        """
        params = self._build_param({})
        r = self.request("GET", f"{self.BASE_API}/my_page", params=params)
        r.raise_for_status()
        my_page_view = MyPageView.FromString(r.content)
        return my_page_view

    def get_my_home(self) -> HomeView:
        """
        Get your personalized home view.

        Same result as the one when you click the ``Home`` section in the app.

        Returns
        -------
        :class:`HomeView`
            Your personalized home view.

        Raises
        :exc:`requests.HTTPError`
            If the request failed.
        """
        params = self._build_param(
            {
                "ui_lang": "en",
            }
        )
        r = self.request("GET", f"{self.BASE_API}/home_v2", params=params)
        r.raise_for_status()
        my_home_view = HomeView.FromString(r.content)
        return my_home_view

    # <-- Api.kt (Personalized)

    # --> Downloader

    def _replace_image_host(self, url: str) -> str:
        if not url.startswith("http"):
            return self.BASE_IMG + url
        # We want to replace the host, but keep the path and everything else
        parse_url = urlparse(url)
        base_img_parse = urlparse(self.BASE_IMG)
        parse_url = parse_url._replace(netloc=base_img_parse.netloc, scheme=base_img_parse.scheme)
        return parse_url.geturl()

    def stream_download(self, url: str, *, chunk_size: int = 1024) -> Generator[bytes, None, None]:
        """
        Stream download the image.

        The `url` is what you get from :meth:`get_chapter_images`.

        Parameters
        ----------
        url: :class:`str`
            The image URL.
        chunk_size: :class:`int`
            The chunk size. Default is 1024.

        Yields
        ------
        :class:`bytes`
            The image chunk.
        """
        join_url = self._replace_image_host(url)
        r = self.request(
            "GET",
            join_url,
            headers={
                "User-Agent": self._client["_IMAGE_UA"],
                "Cache-Control": "no-cache",
                "Host": IMAGE_HOST,
            },
            stream=True,
        )
        for chunk in r.iter_content(chunk_size=chunk_size):
            yield chunk

    # <-- Downloader
