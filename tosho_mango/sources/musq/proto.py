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

from dataclasses import dataclass
from typing import List, Optional

import betterproto

__all__ = (
    "Status",
    "Badge",
    "BadgeManga",
    "ConsumptionType",
    "SubscriptionStatus",
    "UserPoint",
    "Tag",
    "Chapter",
    "ChapterV2",
    "ViewButton",
    "ChaptersRange",
    "MangaDetail",
    "MangaDetailV2",
    "ChapterPage",
    "ChapterViewer",
    "SNSInfo",
    "PageBlock",
    "ChapterViewerV2",
    "Subscription",
    "Billing",
    "PointShopView",
    "PointHistory",
    "PointShopHistory",
    "MangaListNode",
    "MangaList",
    "MyPageView",
    "HomeBanner",
    "HomeFeatured",
    "HomeView",
)


class Status(betterproto.Enum):
    """Ths status of each request."""

    SUCCESS = 0
    """Success"""
    CONTENT_NOT_FOUND = 1
    """Content not found or error"""
    UNRECOGNIZED = -1
    """An error has occurred"""


class Badge(betterproto.Enum):
    """The attached badge of the chapter."""

    NONE = 0
    """No badge marking this chapter"""
    UPDATE = 1
    """New chapter update"""
    ADVANCE = 2
    """Advance chapter"""
    UNRECOGNIZED = -1
    """An error has occurred"""


class BadgeManga(betterproto.Enum):
    """The attached badge of the manga."""

    NONE = 0
    """No badge marking this manga"""
    NEW = 1
    """New manga"""
    UPDATE = 2
    """New chapter/update (filled UP badge)"""
    UPDATE_THIS_WEEK = 3
    """New chapter/update this week (outlined UP badge)"""
    UNREAD = 4
    """Manga with unread chapters"""
    UNRECOGNIZED = -1
    """An error has occurred"""


class ConsumptionType(betterproto.Enum):
    """The type of coin used to read the chapter."""

    ANY_ITEMS = 0
    """Any coin type can be used to read this chapter"""
    EVENT_OR_PAID = 1
    """Only event or paid coins can be used to read this chapter"""
    PAID_ONLY = 2
    """Only paid coins can be used to read this chapter"""
    UNRECOGNIZED = -1
    """An error has occurred"""


class SubscriptionStatus(betterproto.Enum):
    """Subscription status of the user."""

    NOT_SUBSCRIBED = 0
    """Not subscribed"""
    SUBSCRIBED_MONTHLY = 1
    """Subscribed monthly"""
    SUBSCRIBED_YEARLY = 2
    """Subscribed yearly"""
    SUBSCRIBED_SEASONALLY = 3
    """Subscribed seasonally (tri-annual)"""
    SUBSCRIBED_HALF_YEARLY = 4
    """Subscribed half-yearly"""
    UNRECOGNIZED = -1
    """An error has occurred"""


@dataclass
class UserPoint(betterproto.Message):
    """User point information."""

    free: int = betterproto.uint64_field(1)
    """:class:`int`: Free/daily coins that you have."""
    event: int = betterproto.uint64_field(2)
    """:class:`int`: Event/XP coins that you have."""
    paid: int = betterproto.uint64_field(3)
    """:class:`int`: Paid coins that you have."""

    @property
    def total_point(self) -> int:
        """:class:`int`: Total coins that you have."""
        return self.free + self.event + self.paid


@dataclass
class Tag(betterproto.Message):
    """Tag or genre information."""

    id: int = betterproto.uint64_field(1)
    """:class:`int`: The tag ID."""
    name: str = betterproto.string_field(2)
    """:class:`str`: The tag name."""
    image_url: Optional[str] = betterproto.string_field(3)
    """:class:`str`: The tag image URL."""


@dataclass
class Chapter(betterproto.Message):
    """Represents a single chapter."""

    id: int = betterproto.uint64_field(1)
    """:class:`int`: The chapter ID."""
    name: str = betterproto.string_field(2)
    """:class:`str`: The chapter name."""
    subtitle: Optional[str] = betterproto.string_field(3)
    """:class:`str`: The chapter subtitle (usually the actual chapter title)."""
    thumbnail_url: str = betterproto.string_field(4)
    """:class:`str`: The chapter thumbnail URL."""
    consumption: ConsumptionType = betterproto.enum_field(5)
    """:class:`ConsumptionType`: The chapter consumption type."""
    price: int = betterproto.uint64_field(6)
    """:class:`int`: The chapter price in coins, also check with consumption type."""
    end_of_rental_period: Optional[int] = betterproto.uint64_field(7)
    """:class:`Optional[int]`: How much chapter rental period left in seconds.

    If ``0``, rental period is not available anymore.
    If ``None``, rental period is NOT YET activated.
    """
    comments: Optional[int] = betterproto.uint64_field(8)
    """:class:`Optional[int]`: How many comments this chapter has."""
    published: Optional[str] = betterproto.string_field(9)
    """:class:`Optional[str]`: When this chapter was published."""
    badge: Badge = betterproto.enum_field(10)
    """:class:`Badge`: The chapter badge marking."""
    first_page_url: str = betterproto.string_field(11)
    """:class:`str`: The first page URL of the chapter."""

    @property
    def is_free(self):
        """:class:`bool`: Whether the chapter is free or not."""
        return self.price == 0

    @property
    def chapter_title(self):
        """:class:`str`: A combined chapter title."""
        ch_title = self.name
        if self.subtitle:
            ch_title = f"{ch_title} — {self.subtitle}"
        return ch_title


@dataclass
class ChapterV2(betterproto.Message):
    """Represents a single chapter."""

    id: int = betterproto.uint64_field(1)
    """:class:`int`: The chapter ID."""
    name: str = betterproto.string_field(2)
    """:class:`str`: The chapter name."""
    subtitle: Optional[str] = betterproto.string_field(3)
    """:class:`str`: The chapter subtitle (usually the actual chapter title)."""
    thumbnail_url: str = betterproto.string_field(4)
    """:class:`str`: The chapter thumbnail URL."""
    consumption: ConsumptionType = betterproto.enum_field(5)
    """:class:`ConsumptionType`: The chapter consumption type."""
    price: int = betterproto.uint64_field(6)
    """:class:`int`: The chapter price in coins, also check with consumption type."""
    end_of_rental_period: Optional[int] = betterproto.uint64_field(7)
    """:class:`Optional[int]`: How much chapter rental period left in seconds.

    If ``0``, rental period is not available anymore.
    If ``None``, rental period is NOT YET activated.
    """
    comments: Optional[int] = betterproto.uint64_field(8)
    """:class:`Optional[int]`: How many comments this chapter has."""
    published: Optional[str] = betterproto.string_field(9)
    """:class:`Optional[str]`: When this chapter was published."""
    badge: Badge = betterproto.enum_field(10)
    """:class:`Badge`: The chapter badge marking."""
    first_page_url: str = betterproto.string_field(11)
    """:class:`str`: The first page URL of the chapter."""
    final_chapter: bool = betterproto.bool_field(12)
    """:class:`bool`: Whether this is the final chapter or not."""
    page_count: int = betterproto.uint64_field(13)
    """:class:`int`: How many pages this chapter has."""
    read_count: int = betterproto.uint64_field(14)
    """:class:`int`: How many times this chapter has been read."""

    @property
    def is_free(self):
        """:class:`bool`: Whether the chapter is free or not."""
        return self.price == 0

    @property
    def chapter_title(self):
        """:class:`str`: A combined chapter title."""
        ch_title = self.name
        if self.subtitle:
            ch_title = f"{ch_title} — {self.subtitle}"
        return ch_title


@dataclass
class ViewButton(betterproto.Message):
    """The button that will be shown in the manga detail page."""

    chapter: Chapter = betterproto.message_field(1)
    """:class:`Chapter`: The chapter that will be accessed if user click this button."""
    title: str = betterproto.string_field(2)
    """:class:`str`: The button title."""


@dataclass
class MangaDetail(betterproto.Message):
    """Manga information response.

    When you click a manga, this is the response you will get.
    """

    status: Status = betterproto.enum_field(1)
    """:class:`Status`: The status of the request."""
    user_point: UserPoint = betterproto.message_field(2)
    """:class:`UserPoint`: The user purse or point."""
    title: str = betterproto.string_field(3)
    """:class:`str`: The manga title."""
    authors: str = betterproto.string_field(4)
    """:class:`str`: The manga authors, separated by comma."""
    copyright: str = betterproto.string_field(5)
    """:class:`str`: The manga copyright.""" ""
    next_update: Optional[str] = betterproto.string_field(6)
    """:class:`Optional[str]`: The next chapter update time."""
    warning: Optional[str] = betterproto.string_field(7)
    """:class:`Optional[str]`: The manga warning."""
    description: str = betterproto.string_field(8)
    """:class:`str`: The manga description."""
    display_description: bool = betterproto.bool_field(9)
    """:class:`bool`: Whether the description is displayed or not."""
    tags: List[Tag] = betterproto.message_field(10)
    """:class:`List[Tag]`: The manga tags/generes."""
    thumbnail_url: str = betterproto.string_field(11)
    """:class:`str`: The manga thumbnail URL."""
    video_url: Optional[str] = betterproto.string_field(12)
    """:class:`Optional[str]`: The manga video thumbnail URL."""
    chapters: List[Chapter] = betterproto.message_field(13)
    """:class:`List[Chapter]`: The manga chapters."""
    is_favorite: bool = betterproto.bool_field(14)
    """:class:`bool`: Whether the manga is favorited or not."""
    view_button: Optional[ViewButton] = betterproto.message_field(15)
    """:class:`Optional[ViewButton]`: The view button, if any.""" ""
    is_comment_enabled: bool = betterproto.bool_field(16)
    """:class:`bool`: Whether the manga comment is enabled or not."""
    related_manga: List["MangaDetail"] = betterproto.message_field(17)
    """:class:`List[MangaDetail]`: Any related manga."""


@dataclass
class ChaptersRange(betterproto.Message):
    """The hidden chapters range."""

    start_id: int = betterproto.uint64_field(1)
    """:class:`int`: The start chapter ID."""
    end_id: int = betterproto.uint64_field(2)
    """:class:`int`: The end chapter ID."""


@dataclass
class MangaDetailV2(betterproto.Message):
    """Manga information response version 2.

    When you click a manga, this is the response you will get.

    This is version 2 of the response, which is used in the latest version of the app.
    """

    status: Status = betterproto.enum_field(1)
    """:class:`Status`: The status of the request."""
    user_point: UserPoint = betterproto.message_field(2)
    """:class:`UserPoint`: The user purse or point."""
    title: str = betterproto.string_field(3)
    """:class:`str`: The manga title."""
    authors: str = betterproto.string_field(4)
    """:class:`str`: The manga authors, separated by comma."""
    copyright: str = betterproto.string_field(5)
    """:class:`str`: The manga copyright.""" ""
    next_update: Optional[str] = betterproto.string_field(6)
    """:class:`Optional[str]`: The next chapter update time."""
    warning: Optional[str] = betterproto.string_field(7)
    """:class:`Optional[str]`: The manga warning."""
    description: str = betterproto.string_field(8)
    """:class:`str`: The manga description."""
    display_description: bool = betterproto.bool_field(9)
    """:class:`bool`: Whether the description is displayed or not."""
    tags: List[Tag] = betterproto.message_field(10)
    """:class:`List[Tag]`: The manga tags/generes."""
    thumbnail_url: str = betterproto.string_field(11)
    """:class:`str`: The manga thumbnail URL."""
    video_url: Optional[str] = betterproto.string_field(12)
    """:class:`Optional[str]`: The manga video thumbnail URL."""
    chapters: List[ChapterV2] = betterproto.message_field(13)
    """:class:`List[Chapter]`: The manga chapters."""
    is_favorite: bool = betterproto.bool_field(14)
    """:class:`bool`: Whether the manga is favorited or not."""
    view_button: Optional[ViewButton] = betterproto.message_field(15)
    """:class:`Optional[ViewButton]`: The view button, if any.""" ""
    is_comment_enabled: bool = betterproto.bool_field(16)
    """:class:`bool`: Whether the manga comment is enabled or not."""
    related_manga: List["MangaDetail"] = betterproto.message_field(17)
    """:class:`List[MangaDetail]`: Any related manga."""
    hidden_chapters: ChaptersRange = betterproto.message_field(18)
    """:class:`ChaptersRange`: The hidden chapters range."""


@dataclass
class ChapterPage(betterproto.Message):
    """Represents a manga/chapter page."""

    url: str = betterproto.string_field(1)
    """:class:`str`: The image URL."""
    video_url: Optional[str] = betterproto.string_field(2)
    """:class:`Optional[str]`: The video HLS URL."""
    intent_url: str = betterproto.string_field(3)
    """:class:`str`: The chapter page URL intents."""
    extra_id: Optional[int] = betterproto.uint64_field(4)
    """:class:`Optional[int]`: The extra ID, if any."""

    @property
    def filename(self) -> str:
        """:class:`str`: The filename of the image."""
        return self.url.split("/")[-1].split("?")[0]

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


@dataclass
class ChapterViewer(betterproto.Message):
    """Represents a chapter viewer response.

    When you click a chapter, this is the response you will get.
    """

    status: Status = betterproto.enum_field(1)
    """:class:`Status`: The status of the request."""
    user_point: UserPoint = betterproto.message_field(2)
    """:class:`UserPoint`: The user purse or point."""
    images: List[ChapterPage] = betterproto.message_field(3)
    """:class:`List[ChapterPage]`: The chapter images list."""
    next_chapter: Optional[Chapter] = betterproto.message_field(4)
    """:class:`Optional[Chapter]`: The next chapter, if any."""
    previous_chapter: Optional[Chapter] = betterproto.message_field(5)
    """:class:`Optional[Chapter]`: The previous chapter, if any."""
    page_start: int = betterproto.uint64_field(6)
    """:class:`int`: The chapter page start."""
    # event:  # this was something useless, so me ignore.
    is_comment_enabled: bool = betterproto.bool_field(8)
    """:class:`bool`: Whether the chapter comment is enabled or not."""


@dataclass
class SNSInfo(betterproto.Message):
    """TODO"""

    body: str = betterproto.string_field(1)
    """:class:`str`: The SNS body."""
    url: str = betterproto.string_field(2)
    """:class:`str`: The SNS URL."""


@dataclass
class PageBlock(betterproto.Message):
    """TODO"""

    id: int = betterproto.uint64_field(1)
    """:class:`int`: The chapter ID."""
    title: str = betterproto.string_field(2)
    """:class:`str`: The chapter view title."""
    images: List[ChapterPage] = betterproto.message_field(3)
    """:class:`List[ChapterPage]`: The chapter images list."""
    last_page: bool = betterproto.bool_field(4)
    """:class:`bool`: Whether this is the last page or not."""
    start_page: int = betterproto.uint64_field(5)
    """:class:`int`: The chapter page start."""
    sns: SNSInfo = betterproto.message_field(6)
    """:class:`SNSInfo`: The SNS sharing information."""
    page_start: int = betterproto.uint64_field(7)
    """:class:`int`: The chapter page start."""
    page_end: int = betterproto.uint64_field(8)
    """:class:`int`: The chapter page end."""


@dataclass
class ChapterViewerV2(betterproto.Message):
    """Represents a chapter viewer response.

    When you click a chapter, this is the response you will get.
    """

    status: Status = betterproto.enum_field(1)
    """:class:`Status`: The status of the request."""
    user_point: UserPoint = betterproto.message_field(2)
    """:class:`UserPoint`: The user purse or point."""
    blocks: List[PageBlock] = betterproto.message_field(3)  # TODO
    """:class:`List[ChapterPage]`: The chapter images list."""
    next_chapter: Optional[Chapter] = betterproto.message_field(4)
    """:class:`Optional[Chapter]`: The next chapter, if any."""
    is_comment_enabled: bool = betterproto.bool_field(5)
    """:class:`bool`: Whether the chapter comment is enabled or not."""
    enable_guide: bool = betterproto.bool_field(6)
    """:class:`bool`: Whether the chapter view guide is enabled or not."""


@dataclass
class Subscription(betterproto.Message):
    """Subscription information."""

    monthly_id: str = betterproto.string_field(1)
    """:class:`str`: The monthly subscription ID."""
    yearly_id: str = betterproto.string_field(2)
    """:class:`str`: The yearly subscription ID."""
    status: SubscriptionStatus = betterproto.enum_field(3)
    """:class:`SubscriptionStatus`: The subscription status."""
    end_date: int = betterproto.uint64_field(4)
    """:class:`int`: The subscription end date."""
    event_point: int = betterproto.uint64_field(5)
    """:class:`int`: The event point that we will get from the subscription."""
    seasonally_id: Optional[str] = betterproto.string_field(6)
    """:class:`Optional[str]`: The seasonally (tri-annual) subscription ID."""
    half_yearly_id: Optional[str] = betterproto.string_field(7)
    """:class:`Optional[str]`: The half yearly subscription ID."""


@dataclass
class Billing(betterproto.Message):
    """Billing or whatever coin purchase information."""

    id: str = betterproto.string_field(1)
    """:class:`str`: The billing ID."""
    event_point: int = betterproto.uint64_field(2)
    """:class:`int`: The event point that we will get from the purchases."""
    paid_point: int = betterproto.uint64_field(3)
    """:class:`int`: The paid point that we will get from the purchases."""
    detail: str = betterproto.string_field(4)
    """:class:`str`: The billing detail."""

    @property
    def total_point(self):
        """:class:`int`: The total point that we will get from the purchases."""
        return self.event_point + self.paid_point


@dataclass
class PointShopView(betterproto.Message):
    """Represents point shop view response.

    The ``Shop`` section in the actual app.
    """

    user_point: UserPoint = betterproto.message_field(1)
    """:class:`UserPoint`: The user purse or point."""
    point_limit: UserPoint = betterproto.message_field(2)
    """:class:`UserPoint`: The user point limit."""
    next_recovery: int = betterproto.uint64_field(3)
    """:class:`int`: The next free point recovery time in seconds."""
    subscriptions: List[Subscription] = betterproto.message_field(4)
    """:class:`List[Subscription]`: The subscription list."""
    billings: List[Billing] = betterproto.message_field(5)
    """:class:`List[Billing]`: The billing or purchase list."""
    default_select: int = betterproto.uint64_field(6)
    """:class:`int`: The default selected billing index(?)."""


@dataclass
class PointHistory(betterproto.Message):
    """The node of each point purchase history."""

    displayed_text: str = betterproto.string_field(1)
    """:class:`str`: The displayed/title text."""
    free_point: int = betterproto.uint64_field(2)
    """:class:`int`: The free point that we use/get from the purchase."""
    event_point: int = betterproto.uint64_field(3)
    """:class:`int`: The event point that we use/get from the purchase."""
    paid_point: int = betterproto.uint64_field(4)
    """:class:`int`: The paid point that we use/get from the purchase."""
    created_at: int = betterproto.uint64_field(5)
    """:class:`int`: Unix timestamp of the acquisition time."""


@dataclass
class PointShopHistory(betterproto.Message):
    """Represents point shop history response.

    The ``Shop`` section in the actual app then the ``Acquisition History`` section.
    """

    user_point: UserPoint = betterproto.message_field(1)
    """:class:`UserPoint`: The user purse or point."""
    logs: List[PointHistory] = betterproto.message_field(2)
    """:class:`List[PointHistory]`: The point history list."""


@dataclass
class MangaListNode(betterproto.Message):
    """Simple manga information on the manga list or grouping."""

    id: int = betterproto.uint64_field(1)
    """:class:`int`: The manga ID."""
    name: str = betterproto.string_field(2)
    """:class:`str`: The manga title."""
    image_url: str = betterproto.string_field(3)
    """:class:`str`: The manga cover image URL."""
    video_url: Optional[str] = betterproto.string_field(4)
    """:class:`Optional[str]`: The manga video thumbnail URL."""
    short_description: str = betterproto.string_field(5)
    """:class:`str`: The manga short description."""
    campaign: Optional[str] = betterproto.string_field(6)
    """:class:`Optional[str]`: The manga campaign information."""
    favorites: int = betterproto.uint64_field(7)
    """:class:`int`: The manga bookmark/favorites count."""
    badge: BadgeManga = betterproto.enum_field(8)
    """:class:`BadgeManga`: The manga badge information."""
    last_updated: Optional[str] = betterproto.string_field(9)
    """:class:`Optional[str]`: The manga last updated date."""


@dataclass
class MangaList(betterproto.Message):
    """
    The manga list response.

    Contains a list of titles/manga.
    """

    titles: List[MangaListNode] = betterproto.message_field(1)
    """:class:`List[MangaListNode]`: The manga list."""


@dataclass
class MangaGroup(betterproto.Message):
    """Used for grouping manga by tag/genres."""

    name: str = betterproto.string_field(1)
    """:class:`str`: The tag/genre name."""
    titles: List[MangaListNode] = betterproto.message_field(2)
    """:class:`List[MangaListNode]`: The associated manga list."""
    tag_id: int = betterproto.uint64_field(3)
    """:class:`int`: The tag/genre ID."""


@dataclass
class MyPageView(betterproto.Message):
    """Your personalized profile page view response."""

    favorites: List[MangaListNode] = betterproto.message_field(1)
    """:class:`List[MangaListNode]`: The manga list that you bookmarked/favorited."""
    history: List[MangaListNode] = betterproto.message_field(2)
    """:class:`List[MangaListNode]`: The manga list that you read."""
    register_event_point: int = betterproto.uint64_field(3)  # what the fuck is this?
    """:class:`int`: The event point that we get from the registration."""


@dataclass
class HomeBanner(betterproto.Message):
    """The node of each banner on the home page."""

    id: int = betterproto.uint64_field(1)
    """:class:`int`: The manga ID."""
    image_url: str = betterproto.string_field(2)
    """:class:`str`: The manga thumbnail URL."""
    intent_url: str = betterproto.string_field(3)
    """:class:`str`: The manga intent URL."""


@dataclass
class HomeFeatured(betterproto.Message):
    """The currently featured manga on the home page."""

    id: int = betterproto.uint64_field(1)
    """:class:`int`: The manga ID.""" ""
    image_url: str = betterproto.string_field(2)
    """:class:`str`: The manga thumbnail URL."""
    video_url: Optional[str] = betterproto.string_field(3)
    """:class:`Optional[str]`: The video HLS URL."""
    short_description: str = betterproto.string_field(4)
    """:class:`str`: The manga short description."""
    intent_url: str = betterproto.string_field(5)
    """:class:`str`: The manga URL intents."""
    name: str = betterproto.string_field(6)
    """:class:`str`: The manga title."""


@dataclass
class HomeView(betterproto.Message):
    """The personalized home page view response."""

    user_point: UserPoint = betterproto.message_field(1)
    """:class:`UserPoint`: The user purse or point."""
    # popup: Optional[Popup] = betterproto.message_field(2) # implement this later
    top_banners: List[HomeBanner] = betterproto.message_field(3)
    """:class:`List[HomeBanner]`: The top most banner list. (the big single carousel)"""
    top_sub_banners: List[HomeBanner] = betterproto.message_field(4)
    """:class:`List[HomeBanner]`: The top sub banner list. (the smaller carousel)"""
    tutorial_banners: Optional[HomeBanner] = betterproto.message_field(5)
    """:class:`Optional[HomeBanner]`: The tutorial banner list, if any."""
    updated_section_name: str = betterproto.string_field(6)
    """:class:`str`: The updated manga section name. (ex: "Updated for You")"""
    updated_titles: List[MangaListNode] = betterproto.message_field(7)
    """:class:`List[MangaListNode]`: Your personalized updated manga list."""
    tags: List[Tag] = betterproto.message_field(8)
    """:class:`List[Tag]`: The tag/genre list."""
    featured: Optional[HomeFeatured] = betterproto.message_field(9)
    """:class:`Optional[HomeFeatured]`: The currently featured manga."""
    new_titles_section_name: str = betterproto.string_field(10)
    """:class:`str`: The new manga section name. (ex: "New Series")"""
    new_titles: List[MangaListNode] = betterproto.message_field(11)
    """:class:`List[MangaListNode]`: The new manga list."""
    ranking_section_name: str = betterproto.string_field(12)
    """:class:`str`: The ranking section name. (ex: "Ranking")"""
    rankings: List[MangaGroup] = betterproto.message_field(13)
    """:class:`List[MangaGroup]`: The manga ranking list."""
    ranking_description: str = betterproto.string_field(14)
    """:class:`str`: The ranking description."""
    recommended_banner_image_url: str = betterproto.string_field(15)
    """:class:`str`: The recommended banner image URL."""


@dataclass
class AccountDevice(betterproto.Message):
    id: int = betterproto.uint64_field(1)
    """:class:`int`: The device ID."""
    name: str = betterproto.string_field(2)
    """:class:`str`: The device name."""
    install_at: int = betterproto.uint64_field(3)
    """:class:`int`: The device install unix timestamp."""


@dataclass
class AccountView(betterproto.Message):
    """The account view response."""

    devices: List[AccountDevice] = betterproto.message_field(1)
    """:class:`List[AccountDevice]`: The list of devices that you have logged in."""
    registered: Optional[bool] = betterproto.bool_field(2)
    """:class:`bool`: Whether or not you have registered your account."""
    login_url: str = betterproto.string_field(3)
    """:class:`str`: The login URL to connect your account."""


@dataclass
class SettingView(betterproto.Message):
    """The setting view response."""

    tag_name: str = betterproto.string_field(1)
    """:class:`str`: The bridge tag name."""
    keyword: str = betterproto.string_field(2)
    """:class:`str`: The bridge keyword."""
