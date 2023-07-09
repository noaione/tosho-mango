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

import re
from datetime import datetime, timezone
from enum import Enum
from typing import List, Optional

from msgspec import Struct, field

from .errors import KMAPIError

__all__ = (
    "EpisodeBadge",
    "DevicePlatform",
    "GenderType",
    "PublishCategory",
    "FavoriteStatus",
    "SupportStatus",
    "SimpleId",
    "UserPoint",
    "UserTicket",
    "UserFavoriteList",
    "TitleList",
    "EpisodeEntry",
    "PageList",
    "PremiumTicketInfo",
    "TitleTicketInfo",
    "TicketInfo",
    "TitleTicketListEntry",
    "UserAccountDevice",
    "UserAccount",
    "WeeklyListContent",
    "MagazineCategoryInfo",
    "GenreNode",
    "StatusResponse",
    "UserAccountPointResponse",
    "TitleListResponse",
    "UserFavoriteResponse",
    "EpisodesListResponse",
    "ChapterViewerResponse",
    "WebChapterViewerResponse",
    "TitleTicketListResponse",
    "EpisodePurchaseResponse",
    "BulkEpisodePurchaseResponse",
    "AccountResponse",
    "SearchResponse",
    "WeeklyListResponse",
    "MagazineCategoryResponse",
    "GenreSearchResponse",
    "RankingListResponse",
)


def parse_datetime(data: str):
    # YYYY-MM-DD HH:MM:SS
    # Assume UTC
    return datetime.strptime(data, "%Y-%m-%d %H:%M:%S").replace(tzinfo=timezone.utc)


class EpisodeBadge(int, Enum):
    """
    The purchase status of an episode.
    """

    PURCHASEABLE = 1
    """Episode need to be purchased by point or ticket (if possible)"""
    FREE = 2
    """Episode is free to view"""
    PURCHASED = 3
    """Episode is purchased"""
    RENTAL = 4
    """Episode is on rental"""


class DevicePlatform(int, Enum):
    """
    The device platform type
    """

    ANDROID = 2
    """Is android"""
    WEB = 3
    """Is website"""


class GenderType(int, Enum):
    MALE = 1
    FEMALE = 2
    OTHER = 3


class PublishCategory(int, Enum):
    """
    The publication category type
    """

    SERIALIZATION = 1
    """Series is being serialized"""
    COMPLETION = 2
    """Series is completed"""
    READING_OUT = 3
    """Series ????"""


class MagazineCategory(int, Enum):
    """
    The magazine category type
    """

    Undefined = 0
    """Unknown magazine"""
    Original = 1
    """KM Original series"""
    WeeklyShounenMagazine = 2
    """Weekly Shounen Magazine"""
    BessatsuShounenMagazine = 3
    """Bessatsu Shounen Magazine, a spin-off WSM (Monthly)"""
    Gravure = 4
    """Gravure magazine"""
    Misc = 5
    """Misc comic/book"""
    MiscMagazine = 6
    """Misc magazine"""
    OtherCompany = 7
    """Magazine from other company"""
    MonthlyShounenSirius = 8
    """Monthly Shounen Sirius"""
    SuiyobiShounenSirius = 9
    """Suiyobi no Sirius, a spin-off MSS, released every Wednesday"""
    MonthlyShounenMagazine = 10
    """Monthly Shounen Magazine"""
    ShounenMagazineR = 11
    """
    Shounen Magazine R, a supplement magazine for WSM. (Discontinued)

    Originally a bi-monthly magazine, but now it's a monthly digital-only magazine.
    """
    BekkanGetsumaga = 12
    """Bekkan Getsumaga"""
    WeeklyYoungMagazine = 13
    """Weekly Young Magazine, Seinen focused magazine"""
    WeeklyYoungMagazineThe3rd = 14
    """Weekly Young Magazine the 3rd, a supplementary for WYM"""
    MonthlyYoungMagazine = 15
    """Monthly Young Magazine, a sister magazine of WYM"""
    ShounenMagazineEdge = 16
    """Shounen Magazine Edge, a monthly magazine"""
    Morning = 17
    """Morning magazine, a weekly seinen magazine"""
    MorningTwo = 18
    """Morning Two, a montlhy version of Morning"""
    Afternoon = 19
    """Afternoon magazine, a monthly seinen magazine"""
    GoodAfternoon = 20
    """Good! Afternoon, sister magazine of Afternoon, a monthly seinen magazine"""
    Evening = 21
    """Evening magazine, a bi-weekly seinen magazine, discontinued and moved some to Comic Days"""
    ComicBombom = 22
    """Comic BomBom, a monthly kids magazine"""
    eYoungMagazine = 23
    """e-Young Magazine, a digital-only of Young magazine which focused on user-submitted serialized content"""
    DMorning = 24
    """Digital Morning, a digital-only version of Morning magazine"""
    ComicDays = 25
    """Comic Days, a digital-only seinen magazine"""
    Palcy = 26
    """Palcy, a digital-only magazine collaboration with Pixiv"""
    Cycomi = 27
    """Cycomi, a digital-only magazine collaboration with Cygames"""
    MangaBox = 28
    """Manga Box, a mobile app for manga from Kodansha, Shogakukan, and other publishers"""
    Nakayoshi = 29
    """Nakayoshi/Good Friend, a monthly shoujo magazine"""
    BessatsuFriend = 30
    """Bessatsu Friend, a monthly shoujo magazine"""
    Dessert = 31
    """Dessert, a monthly shoujo/josei magazine"""
    Kiss = 32
    """Kiss, a monthly josei magazine"""
    HatsuKiss = 33
    """Hatsu Kiss, a monthly josei magazine (originally bi-monthly until 2018)"""
    BeLove = 34
    """Be Love, a monthly josei magazine"""
    HoneyMilk = 35
    """Honey Milk. a web magazine focused on Boys Love"""
    AneFriend = 36
    """Ane Friend, a web shoujo/josei magazine"""
    comic_tint = 37
    """Comic Tint, a web shoujo/josei magazine"""
    GakujutsuBunko = 38
    """Kodansha Gakujutsu Bunko or Kodansha Academic Paperback Library"""
    Seikaisha = 39
    """Seikaisha or Star Seas Company, a subsidiary of Kodansha"""
    BabyMofu = 40
    """Baby Mofu, a website focused on books and manga related to childcare"""
    Ichijinsha = 41
    """
    Ichijinsha, a subsidiary of Kodansha

    Owns the following:
    - Febri
    - Comic Rex (4-koma)
    - Monthly Comic Zero Sum (Josei focused)
    - Comic Yuri Hime (Girls Love)
    - gateau
    - IDOLM@STER Million Live Magazine Plus+
    """
    ComicCreate = 42
    """Comic Create, a web comic magazine"""
    ComicBull = 43
    """Comic Bull, a magazine by Sports Bull"""
    YoungMagazineWeb = 44
    """Young Magazine Web, a digital-only magazine for Young Magazine"""
    WhiteHeart = 45
    """White Heart, a digital-only magazine for josei manga"""
    MonthlyMagazineBase = 46
    """Monthly Magazine Base, a digital-only magazine for shounen manga, replacement for Shounen Magazine R"""

    @property
    def pretty(self):
        # Convert to space separated string
        split_space = re.sub(r"((?<=[a-z])[A-Z]|(?<!\A)[A-Z](?=[a-z])|([\d])+)", r" \1", self.name).strip().split()
        # Special case
        join_back = " ".join(split_space)
        for word in ["The", "A", "An"]:
            if join_back.startswith(word):
                continue
            join_back = join_back.replace(word, word.lower())
        if len(split_space) > 1:
            if split_space[0] == "e":
                return join_back.replace("e ", "e-", 1)
            elif split_space[0] == "D":
                return join_back.replace("D ", "Digital ", 1)
        if "_" in join_back:
            return join_back.replace("_", " ")
        return join_back


class FavoriteStatus(int, Enum):
    NONE = 1
    FAVORITE = 2


class SupportStatus(int, Enum):
    NOT_ALLOWED = 0
    ALLOWED = 1
    APPLIED = 2


class SimpleId(Struct):
    id: int


class StatusResponse(Struct):
    """
    The base response for all API calls.
    """

    status: str
    """The status of the response, usually "success" or "fail"."""
    response_code: int
    """The response code of the response, usually 200 for success."""
    error_message: str
    """The error message of the response, usually empty if success."""

    def raise_for_status(self):
        if self.status != "success":
            raise KMAPIError(self.response_code, self.error_message)


class UserPoint(Struct):
    """
    The user point information.
    """

    paid_point: int
    """The paid/purchased point that the user have."""
    free_point: int
    """The free point that the user have."""
    point_sale_text: str | None = None
    """Unknown"""
    point_sale_finish_datetime: str | None = None
    """Unknown"""

    @property
    def point_sale_finish(self) -> Optional[datetime]:
        # Parse the datetime string to datetime object
        if self.point_sale_finish_datetime is not None:
            return parse_datetime(self.point_sale_finish_datetime)

    @property
    def total_point(self) -> int:
        return self.paid_point + self.free_point

    def can_purchase(self, price: int) -> bool:
        total_point = self.paid_point + self.free_point
        return total_point >= price

    def subtract(self, price: int):
        if not self.can_purchase(price):
            return  # silently fail

        # Subtract from free point first
        fp_min = min(self.free_point, price)
        self.free_point -= fp_min

        pp_min = min(self.paid_point, price - fp_min)
        self.paid_point -= pp_min

    def add(self, bonus: int):
        self.free_point += bonus


class UserTicket(Struct):
    """
    The premium ticket information.
    """

    total_num: int
    """Total ticket the user have."""


class UserAccountPointResponse(StatusResponse):
    """
    Represents an user account point response.

    A subclass of :class:`StatusResponse`.
    """

    point: UserPoint
    """The user point information."""
    ticket: UserTicket
    """The user premium ticket information."""


class UserFavoriteList(Struct):
    """
    Manga that the user favorited.
    """

    free_episode_updated: str
    """The last updated time of the free episode."""
    paid_episode_updated: str
    """The last updated time of the paid episode."""
    is_unread_free_episode: int
    """Is there any unread free episode."""
    purchase_status: EpisodeBadge
    """Purchase status of the manga."""
    ticket_recover_time: str
    """The title ticket recover time."""
    title_id: int
    """The title ID."""


class TitleList(Struct):
    """
    A node of a title or manga.
    """

    title_id: int
    """The manga ID."""
    title_name: str
    """The manga name."""
    banner_image_url: str
    """The banner image URL."""
    thumbnail_image_url: str
    """The thumbnail image URL."""
    thumbnail_rect_image_url: str
    """The thumbnail (square) image URL."""
    feature_image_url: str
    """The feature image URL."""
    campaign_text: str
    """The current active campaign text."""
    notice_text: str
    """The current notice for the manga."""
    next_updated_text: Optional[str]
    """The next update for the manga."""
    author_text: str
    """The author of the manga."""
    author_list: List[str]
    """The author list of the manga."""
    introduction_text: str
    """The description of the manga."""
    short_introduction_text: str
    """The short description of the manga."""
    free_episode_update_cycle_text: str
    """When will a free episode will be added."""
    new_episode_update_cycle_text: str
    """When will a new episode will be added."""
    episode_order: int
    """The order of the episode."""
    first_episode_id: int
    """The first episode ID."""
    episode_id_list: List[int]
    """The list of episode IDs."""
    latest_paid_episode_id: List[int]
    """The latest paid episode ID."""
    latest_free_episode_id: int
    """The latest free episode ID."""
    genre_id_list: List[int]
    favorite_status: FavoriteStatus
    support_status: SupportStatus
    publish_category: PublishCategory
    magazine_category: MagazineCategory = field(default=MagazineCategory.Undefined)


class TitleListResponse(StatusResponse):
    """
    Represents a title list response.

    A subclass of :class:`StatusResponse`.
    """

    title_list: List[TitleList]
    """The list of titles."""


class UserFavoriteResponse(StatusResponse):
    """
    Represents a user favorite response.

    A subclass of :class:`StatusResponse`.
    """

    favorite_num: int
    """The number of favorited manga."""
    favorite_title_list: List[UserFavoriteList]
    """The list of favorited manga."""
    max_favorite_num: int
    """The maximum number of favorited manga."""
    title_list: List[TitleList] = field(default_factory=list)
    """The list of manga."""


class EpisodeEntry(Struct):
    """
    An episode entry of a manga.
    """

    badge: EpisodeBadge
    """The badge of the episode."""
    episode_id: int
    """The episode ID."""
    episode_name: str
    """The episode name."""
    index: int
    """Th episode index."""
    point: int
    """The episode purchase point."""
    bonus_point: int
    """The episode bonus point (if read)"""
    use_status: int
    """The episode use status."""
    ticket_rental_enabled: int
    """The episode ticket rental status."""
    title_id: int
    """The title ID."""
    start_time: str
    """The episode start time or release time."""
    rental_rest_time: Optional[str]

    def start_time_datetime(self) -> datetime:
        """:class:`datetime.datetime`: The episode start time or release time as datetime object."""
        return parse_datetime(self.start_time)

    def ticketable(self) -> bool:
        """:class:`bool`: Whether the episode is ticketable or not."""
        return self.ticket_rental_enabled == 1

    def available(self) -> bool:
        """:class:`bool`: Whether the episode is available to view or not."""
        return self.badge != 1

    def set_available(self) -> None:
        """:class:`None`: Set the episode to available."""
        self.badge = EpisodeBadge.PURCHASED


class EpisodesListResponse(StatusResponse):
    """
    Represents an episode list response.

    A subclass of :class:`StatusResponse`.
    """

    episode_list: List[EpisodeEntry]
    """The list of episodes."""


class PageList(Struct):
    """The page list of a chapter viewer."""

    index: int
    """The page index."""
    image_url: str
    """The page image URL."""

    @property
    def filename(self) -> str:
        """:class:`str`: The filename of the image."""
        return self.image_url.split("/")[-1].split("?")[0]

    @property
    def stem(self) -> str:
        """:class:`str`: The stem of the image, without the extension."""
        return self.filename.rsplit(".", 1)[0]

    @property
    def extension(self) -> str:
        """:class:`str`: The extension of the image, without the leading dot."""
        try:
            _, ext = self.filename.rsplit(".", 1)
            return ext
        except ValueError:
            return ""


class ChapterViewerResponse(StatusResponse):
    """
    Represents a chapter viewer response. (Mobile app)
    Will be available when you try to view a chapter/episode.

    A subclass of :class:`StatusResponse`.
    """

    episode_id: int
    """The episode ID."""
    page_list: List[PageList]
    """The list of pages."""
    episode_list: List[EpisodeEntry]
    """The list of episodes for the title."""
    prev_episode_id: Optional[int] = None
    """The previous episode ID."""
    next_episode_id: Optional[int] = None
    """The next episode ID."""


class WebChapterViewerResponse(StatusResponse):
    """
    Represents a chapter viewer response. (Web)
    Will be available when you try to view a chapter/episode.

    A subclass of :class:`StatusResponse`.
    """

    bonus_point: int
    """The bonus point of the episode."""
    episode_id: int
    """The episode ID."""
    scramble_seed: int
    """The scramble seed of the episode."""
    title_id: int
    """The title ID."""
    page_list: List[str]
    """The list of pages."""


class PremiumTicketInfo(Struct):
    """
    The premium ticket info of a manga.
    """

    own_ticket_num: int
    """The number of owned premium tickets."""
    rental_second: int
    """The rental time of the premium ticket."""
    ticket_type: int
    """The ticket type of the premium ticket."""


class TitleTicketInfo(Struct):
    """
    The title ticket info of a manga.
    """

    own_ticket_num: int
    """The number of owned title tickets."""
    rental_second: int
    """The rental time of the title ticket."""
    ticket_type: int
    """The ticket type of the title ticket."""
    ticket_version: int
    """The ticket version of the title ticket."""
    max_ticket_num: int
    """The maximum number of title tickets you can own."""
    recover_second: int
    """The recover time left of the title ticket."""
    finish_time: Optional[int] = None
    """The finish time of the title ticket."""
    next_ticket_recover_second: Optional[int] = None
    """The next ticket recover time left of the title ticket."""


class TicketInfo(Struct):
    """
    The ticket info of a manga.
    """

    premium_ticket_info: PremiumTicketInfo
    """The premium ticket info."""
    title_ticket_info: TitleTicketInfo
    """The title ticket info."""
    target_episode_id_list: List[int]
    """The list of applicable episode IDs."""


class TitleTicketListEntry(Struct):
    """
    The title ticket list entry of a manga.
    """

    title_id: int
    """The title ID."""
    ticket_info: TicketInfo
    """The ticket info."""

    def title_available(self) -> bool:
        """:class:`bool`: Whether the title ticket is available or not."""
        title_ticket = self.ticket_info.title_ticket_info.own_ticket_num > 0
        return title_ticket

    def premium_available(self) -> bool:
        """:class:`bool`: Whether the premium ticket is available or not."""
        premium_ticket = self.ticket_info.premium_ticket_info.own_ticket_num > 0
        return premium_ticket

    def subtract_title(self) -> None:
        """:class:`None`: Subtract the title ticket."""
        self.ticket_info.title_ticket_info.own_ticket_num -= 1

    def subtract_premium(self) -> None:
        """:class:`None`: Subtract the premium ticket."""
        self.ticket_info.premium_ticket_info.own_ticket_num -= 1


class TitleTicketListResponse(StatusResponse):
    """
    Represents a title ticket list response.

    A subclass of :class:`StatusResponse`.
    """

    title_ticket_list: List[TitleTicketListEntry]
    """The list of title ticket entries."""


class EpisodePurchaseResponse(StatusResponse):
    """
    Represents an episode purchase response.

    A subclass of :class:`StatusResponse`.
    """

    account_point: int
    """The point left on the account"""
    paid_point: int
    """The point paid for the episode"""


class BulkEpisodePurchaseResponse(EpisodePurchaseResponse):
    """
    Represents an episode purchase response.

    A subclass of :class:`EpisodePurchaseResponse`.
    """

    earned_point_back: int
    """The point earned back from the purchase"""


class UserAccountDevice(Struct):
    """
    The device info of a user account.
    """

    user_id: int
    """:class:`int`: The user ID or device ID."""
    device_name: str
    """:class:`str`: The device name."""
    platform: DevicePlatform
    """:class:`DevicePlatform`: The device platform."""


class UserAccount(Struct):
    """
    The user account info.
    """

    account_id: int
    """:class:`int`: The account ID."""
    is_registerd: int
    """:class:`int`: Whether the account is registered or not."""
    user_id: int
    """:class:`int`: The user/device ID."""
    nickname: str
    """:class:`str`: User nickname."""
    email: str
    """:class:`str`: User email."""
    gender: int
    """:class:`int`: User gender"""
    birthyear: int
    """:class:`int`: User birthday year"""
    device_list: list[UserAccountDevice]
    """:class:`list[UserAccountDevice]`: The list of registered devices."""
    days_since_created: int
    """:class:`int`: The number of days since the account was created."""


class AccountResponse(StatusResponse):
    """
    Represents an account response.

    A subclass of :class:`StatusResponse`.
    """

    account: UserAccount
    """:class:`UserAccount`: The user account."""


class SearchResponse(StatusResponse):
    """
    Represents a search response.

    A subclass of :class:`StatusResponse`.
    """

    title_list: List[TitleList]
    """The list of titles."""
    title_id_list: List[int]
    """The list of title IDs."""


class WeeklyListContent(Struct):
    """
    The weekly list content.
    """

    title_id_list: List[int]
    """The list of title IDs."""
    # 1: Monday -> 7: Sunday
    weekday_index: int
    """The weekday index. (1: Monday -> 7: Sunday)"""
    feature_title_id: int
    """The featured title ID."""
    bonus_point_title_id: List[int]
    """The list of title with bonus point"""
    popular_title_id_list: List[int]
    """The list of popular title IDs."""
    new_title_id_list: List[int]
    """The list of new title IDs."""


class WeeklyListResponse(StatusResponse):
    """
    Represents a weekly list response.

    A subclass of :class:`StatusResponse`.
    """

    weekly_list: List[WeeklyListContent]
    """The list of weekly list contents."""
    title_list: List[TitleList] = field(default_factory=list)
    """The list of titles."""


class MagazineCategoryInfo(Struct):
    is_purchase: int
    is_search: int
    is_subscription: int
    category_id: int = field(name="magazine_category_id")
    category_name: str = field(name="magazine_category_name_text")
    image_url: Optional[str] = field(name="subscription_image_url")


class MagazineCategoryResponse(StatusResponse):
    magazine_category_list: List[MagazineCategoryInfo]


class GenreNode(Struct):
    genre_id: int
    genre_name: str
    image_url: str


class GenreSearchResponse(StatusResponse):
    genre_list: list[GenreNode]


class RankingListResponse(StatusResponse):
    ranking_title_list: list[SimpleId]
