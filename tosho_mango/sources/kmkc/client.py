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

from base64 import b64decode
from hashlib import md5, sha256, sha512
from http.cookiejar import Cookie, CookieJar
from typing import Any, Generator, TypeVar
from urllib.parse import quote

import msgspec
import requests

from tosho_mango.sources.kmkc.config import KMConfigMobile, KMConfigWeb, save_config

from .constants import (
    API_HOST,
    API_MOBILE_UA,
    API_UA,
    BASE_HOST,
    DEVICE_PLATFORM,
    DEVICE_VERSION,
    HASH_HEADER,
    HASH_MOBILE_HEADER,
)
from .dto import (
    AccountResponse,
    BulkEpisodePurchaseResponse,
    EpisodeEntry,
    EpisodePurchaseResponse,
    EpisodesListResponse,
    GenreSearchResponse,
    MagazineCategoryResponse,
    PremiumTicketInfo,
    RankingListResponse,
    SearchResponse,
    StatusResponse,
    TitleListResponse,
    TitlePurchaseResponse,
    TitleTicketInfo,
    TitleTicketListEntry,
    TitleTicketListResponse,
    UserAccountPointResponse,
    UserPoint,
    WebChapterViewerResponse,
    WeeklyListResponse,
)
from .errors import KMNotEnoughPointError

__all__ = ("KMClientWeb",)
DtoT = TypeVar("DtoT", bound="StatusResponse")


def hash_kv(key: str, value: str):
    key_b = key.encode("utf-8")
    val_b = value.encode("utf-8")

    key_hash = sha256(key_b).hexdigest()
    val_hash = sha512(val_b).hexdigest()
    return f"{key_hash}_{val_hash}"


class KMClientBase:
    """The base client for interacting with KC KM Web API."""

    API_HOST = b64decode("aHR0cHM6Ly9hcGkua21hbmdhLmtvZGFuc2hhLmNvbQ==").decode("utf-8")
    CDN_HOST = b64decode("aHR0cHM6Ly9jZG4ua21hbmdhLmtvZGFuc2hhLmNvbQ==").decode("utf-8")
    _config: KMConfigWeb | KMConfigMobile  # type: ignore

    def __init__(self, config: KMConfigWeb | KMConfigMobile) -> None:
        self._config = config
        self._client = requests.Session()
        self._client.headers.update(
            {
                "User-Agent": API_UA if isinstance(config, KMConfigWeb) else API_MOBILE_UA,
                "Host": API_HOST,
                "accept": "application/json",
            },
        )

    @property
    def client(self):
        """:class:`requests.Session`: The underlying HTTP client."""
        return self._client

    def _create_request_hash(self, query_params: dict[str, str]) -> str:
        """Create the request hash for the given query parameters.

        Parameters
        ----------
        query_params: :class:`dict[str, str]`
            The query/body parameters to create the hash for.

        Returns
        -------
        :class:`str`
            The request hash.
        """
        if isinstance(self._config, KMConfigMobile):
            # Thanks to neckothy for the help
            sha_256 = sha256()
            for vals in sorted({"hash_key": self._config.user_secret, **query_params}.values()):
                sha_256.update(md5(vals.encode("utf-8")).hexdigest().encode("utf-8"))  # noqa: S324
            return sha_256.hexdigest()

        birthday = self._config.birthday.value
        expires = str(self._config.birthday.expires)

        keys = list(query_params.keys())
        keys.sort()
        qi_s = []
        for key in keys:
            qi_s.append(hash_kv(key, query_params[key]))

        qi_s_hashed = sha256(",".join(qi_s).encode()).hexdigest()
        birth_expire_hash = hash_kv(birthday, expires)

        merged_hash = sha512(f"{qi_s_hashed}{birth_expire_hash}".encode("utf-8")).hexdigest()
        return merged_hash

    def _format_request(self, query: dict[str, str] | None = None, headers: dict[str, str] | None = None):
        """Format the request with the required headers and query parameters.

        Parameters
        ----------
        query: :class:`dict[str, str]` | ``None``
            The query or body you want to add, by default None
        headers: :class:`dict[str, str]` | ``None``
            The headers you want to add, by default None

        Returns
        -------
        :class:`tuple[dict[str, str], dict[str, str]]`
            A tuple containing the formatted query and headers.
        """
        extend_query = {
            "platform": DEVICE_PLATFORM,
            "version": DEVICE_VERSION,
            **(query or {}),
        }
        extend_headers = {
            **(headers or {}),
        }
        req_hash = self._create_request_hash(extend_query)
        if isinstance(self._config, KMConfigMobile):
            extend_headers[HASH_MOBILE_HEADER] = req_hash
        else:
            extend_headers[HASH_HEADER] = req_hash
        return extend_query, extend_headers

    def request(self, method: str, url: str, **kwargs):
        """Make a request to the API.

        This request will also automatically apply and save the cookies to the config.

        Parameters
        ----------
        method: :class:`str`
            The HTTP method to use.
        url: :class:`str`
            The URL to make the request to.
        kwargs: :class:`dict[str, Any]`
            The keyword arguments to pass to the request.

        Returns
        -------
        :class:`requests.Response`
            The response from the API.
        """
        data = kwargs.pop("data", None)
        headers = kwargs.pop("headers", None)
        params = kwargs.pop("params", None)

        fmt_data, fmt_headers = self._format_request(data, headers)
        fmt_params, fmt_param_headers = self._format_request(params)

        key_param = "data"
        if data is None and params is None:
            # Assume params
            fmt_data = fmt_params
            fmt_headers = fmt_param_headers
            key_param = "params"
        elif data is None and params is not None:
            # Assume params
            fmt_data = fmt_params
            fmt_headers = fmt_param_headers
            key_param = "params"
        elif data is not None:
            # Assume data
            fmt_data = fmt_data
            key_param = "data"
            fmt_headers["Content-Type"] = "application/x-www-form-urlencoded"

        new_kwargs = {
            **kwargs,
            key_param: fmt_data,
            "headers": fmt_headers,
        }
        requested = self._client.request(method, url, **new_kwargs)
        if isinstance(self._config, KMConfigWeb):
            self._config.apply_cookies(requested.cookies)
            save_config(self._config)
        return requested

    def _make_response(self, response: requests.Response, *, type: type[DtoT]) -> DtoT:
        """Create a entity response from the given HTTP response.

        Parameters
        ----------
        response: :class:`requests.Response`
            The HTTP response to create the response from.
        type: :class:`type[DtoT]`
            The type of response to create.

        Returns
        -------
        DtoT
            The created response.

        Raises
        ------
        :exc:`.exceptions.KMAPIError`
            If the response is not successful.
        """
        _temp = msgspec.json.decode(response.content, type=StatusResponse)
        _temp.raise_for_status()
        parsed = msgspec.json.decode(response.content, type=type)
        return parsed

    def chunk_episodes(self, episode_ids: list[int], *, chunk_size: int = 50) -> Generator[list[int], Any, None]:
        """Chunk episode ids into a list of lists with the specified chunk size.

        Parameters
        ----------
        episode_ids: :class:`list[int]`
            The episode IDs to chunk.
        chunk_size: :class:`int`
            The size of each chunk.

        Returns
        -------
        :class:`Generator[list[int], Any, None]`
            A generator that yields a list of episode IDs.
        """

        for i in range(0, len(episode_ids), chunk_size):
            yield episode_ids[i : i + chunk_size]


class KMClientWeb(KMClientBase):
    """The main client for interacting with KC KM Web API.

    Usage
    -----
    ```py
    from tosho_mango.sources.kmkc import KMClientWeb

    ...

    client = KMClientWeb(config)
    results = client.get_title_list([10041])
    print(results)
    ```
    """

    API_HOST = b64decode("aHR0cHM6Ly9hcGkua21hbmdhLmtvZGFuc2hhLmNvbQ==").decode("utf-8")
    CDN_HOST = b64decode("aHR0cHM6Ly9jZG4ua21hbmdhLmtvZGFuc2hhLmNvbQ==").decode("utf-8")
    _config: KMConfigWeb  # type: ignore

    def __init__(self, config: KMConfigWeb) -> None:
        super().__init__(config)

        self._client = requests.Session()
        self._client.headers.update(
            {
                "User-Agent": API_UA,
                "Host": API_HOST,
                "accept": "application/json",
            },
        )
        self._client.cookies.update(self._create_cookiejar())

    def _create_cookiejar(self) -> CookieJar:
        cookie_jar = CookieJar()
        birthday = self._config.birthday
        birthday_value = quote(msgspec.json.encode(birthday).decode("utf-8"))

        cookie_jar.set_cookie(
            Cookie(
                version=0,
                name="birthday",
                value=birthday_value,
                port=None,
                port_specified=False,
                domain=f".{BASE_HOST}",
                domain_specified=True,
                domain_initial_dot=True,
                path="/",
                path_specified=False,
                secure=True,
                expires=birthday.expires,
                discard=False,
                comment=None,
                comment_url=None,
                rest={},
            ),
        )

        tos_adult = self._config.tos_adult
        tos_adult_value = quote(msgspec.json.encode(tos_adult).decode("utf-8"))

        cookie_jar.set_cookie(
            Cookie(
                version=0,
                name="terms_of_service_adult",
                value=tos_adult_value,
                port=None,
                port_specified=False,
                domain=f".{BASE_HOST}",
                domain_specified=True,
                domain_initial_dot=True,
                path="/",
                path_specified=False,
                secure=True,
                expires=tos_adult.expires,
                discard=False,
                comment=None,
                comment_url=None,
                rest={},
            ),
        )

        pri_pol = self._config.privacy
        pri_pol_value = quote(msgspec.json.encode(pri_pol).decode("utf-8"))

        cookie_jar.set_cookie(
            Cookie(
                version=0,
                name="privacy_policy",
                value=pri_pol_value,
                port=None,
                port_specified=False,
                domain=f".{BASE_HOST}",
                domain_specified=True,
                domain_initial_dot=True,
                path="/",
                path_specified=False,
                secure=True,
                expires=pri_pol.expires,
                discard=False,
                comment=None,
                comment_url=None,
                rest={},
            ),
        )

        cookie_jar.set_cookie(
            Cookie(
                version=0,
                name="uwt",
                value=quote(self._config.uwt),
                port=None,
                port_specified=False,
                domain=f".{BASE_HOST}",
                domain_specified=True,
                domain_initial_dot=True,
                path="/",
                path_specified=False,
                secure=True,
                expires=birthday.expires,
                discard=False,
                comment=None,
                comment_url=None,
                rest={"HTTPOnly": ""},
            ),
        )

        return cookie_jar

    def get_episode_list(self, episodes: list[int]):
        """Get episode list from episode ids.

        Parameters
        ----------
        episodes: :class:`list[int]`
            The episode IDs to get the list for.

        Returns
        -------
        :class:`list[Episode]`
            The list of episodes.

        Raises
        ------
        :exc:`.exceptions.KMAPIError`
            If the response is not successful.
        """

        response = self.request(
            "POST",
            f"{self.API_HOST}/episode/list",
            data={"episode_id_list": ",".join(map(str, episodes))},
        )

        parsed = self._make_response(response, type=EpisodesListResponse)
        return parsed.episode_list

    def get_title_list(self, titles: list[int]):
        """Get title list from title IDs.

        Parameters
        ----------
        titles: :class:`list[int]`
            The titles IDs to get the list for.

        Returns
        -------
        :class:`list[TitleNode]`
            The list of titles.

        Raises
        ------
        :exc:`.exceptions.KMAPIError`
            If the response is not successful.
        """

        response = self.request(
            "GET",
            f"{self.API_HOST}/title/list",
            params={"title_id_list": ",".join(map(str, titles))},
        )

        parsed = self._make_response(response, type=TitleListResponse)
        return parsed.title_list

    def get_chapter_viewer(self, episode_id: int):
        """Get chapter viewer from episode ID.

        Parameters
        ----------
        episode_id: :class:`int`
            The episode ID to get the viewer for.

        Returns
        -------
        :class:`WebChapterViewerResponse`
            The chapter viewer response.

        Raises
        ------
        :exc:`.exceptions.KMAPIError`
            If the response is not successful.
        """

        response = self.request(
            "GET",
            f"{self.API_HOST}/web/episode/viewer",
            params={"episode_id": str(episode_id)},
        )

        return self._make_response(response, type=WebChapterViewerResponse)

    def get_title_ticket(self, title_id: int) -> TitleTicketListEntry:
        """Get title ticket from title ID.

        Parameters
        ----------
        title_id: :class:`int`
            The title ID to get the ticket for.

        Returns
        -------
        :class:`TitleTicketListEntry`
            The title ticket.

        Raises
        ------
        :exc:`.exceptions.KMAPIError`
            If the response is not successful.
        """

        response = self.request(
            "GET",
            f"{self.API_HOST}/title/ticket/list",
            params={"title_id_list": str(title_id)},
        )

        parsed = self._make_response(response, type=TitleTicketListResponse)
        return parsed.title_ticket_list[0]

    def claim_episode_with_ticket(self, episode_id: int, ticket: TitleTicketInfo | PremiumTicketInfo):
        """Claim or purchase an episode with a ticket.

        Parameters
        ----------
        episode_id: :class:`int`
            The episode ID to claim.
        ticket: :class:`TitleTicketInfo` | :class:`PremiumTicketInfo`
            The ticket to use.

        Returns
        -------
        :class:`tuple[StatusResponse, bool]`
            The status response and whether the ticket is a title ticket.

        Raises
        ------
        :exc:`.exceptions.KMAPIError`
            If the response is not successful.
        """
        form_data = {
            "episode_id": str(episode_id),
            "ticket_type": str(ticket.ticket_type),
        }
        is_title = hasattr(ticket, "ticket_version")
        if isinstance(ticket, TitleTicketInfo):
            form_data["ticket_version"] = str(ticket.ticket_version)
        else:
            # Premium Ticket
            form_data["ticket_version"] = "1"
            form_data["ticket_type"] = "99"

        response = self.request(
            "POST",
            f"{self.API_HOST}/episode/rental/ticket",
            data=form_data,
        )

        return self._make_response(response, type=StatusResponse), is_title

    def claim_episode_with_point(self, episode: EpisodeEntry, wallet: UserPoint):
        """Claim or purchase an episode with a user's point.

        Parameters
        ----------
        episode: :class:`EpisodeEntry`
            The episode to claim.
        wallet: :class:`UserPoint`
            The user's point wallet.

        Returns
        -------
        :class:`UserPoint`
            The updated user's point wallet.

        Raises
        ------
        :exc:`.exceptions.KMAPIError`
            If the response is not successful.
        :exc:`.exceptions.KMNotEnoughPointError`
            If the user does not have enough points.
        """
        if not wallet.can_purchase(episode.point):
            raise KMNotEnoughPointError

        form_data = {
            "episode_id": str(episode.episode_id),
            "check_point": str(episode.point),
        }
        response = self.request(
            "POST",
            f"{self.API_HOST}/episode/paid",
            data=form_data,
        )

        parsed = self._make_response(response, type=EpisodePurchaseResponse)
        wallet.subtract(parsed.paid_point)
        wallet.add(episode.bonus_point)
        return wallet

    def claim_bulk_episode(self, episodes: list[EpisodeEntry], wallet: UserPoint):
        """Claim or purchase multiple episodes with a user's point.

        Parameters
        ----------
        episodes: :class:`list[EpisodeEntry]`
            The episodes to claim.
        wallet: :class:`UserPoint`
            The user's point wallet.

        Returns
        -------
        :class:`UserPoint`
            The updated user's point wallet.

        Raises
        ------
        :exc:`.exceptions.KMAPIError`
            If the response is not successful.
        :exc:`.exceptions.KMNotEnoughPointError`
            If the user does not have enough points.
        """

        # Test if all episodes can be purchased
        wallet_copy = msgspec.json.decode(msgspec.json.encode(wallet), type=UserPoint)
        paid_point = 0
        bonus_point = 0
        for episode in episodes:
            if not wallet_copy.can_purchase(episode.point):
                raise KMNotEnoughPointError
            wallet_copy.subtract(episode.point)
            paid_point += episode.point
            wallet_copy.add(episode.bonus_point)
            bonus_point += episode.bonus_point

        form_data = {
            "episode_id_list": ",".join(map(lambda x: str(x.episode_id), episodes)),
            "paid_point": str(paid_point),
            "point_back": str(bonus_point),
        }
        response = self.request(
            "POST",
            f"{self.API_HOST}/episode/paid/bulk",
            data=form_data,
        )

        parsed = self._make_response(response, type=BulkEpisodePurchaseResponse)
        wallet.subtract(parsed.paid_point)
        wallet.add(parsed.earned_point_back)
        return wallet

    def get_user_point(self):
        """Get the user's point wallet.

        Returns
        -------
        :class:`UserPointPointResponse`
            The user's point responses entity.

        Raises
        ------
        :exc:`.exceptions.KMAPIError`
            If the response is not successful.
        """

        response = self.request(
            "GET",
            f"{self.API_HOST}/account/point",
        )

        return self._make_response(response, type=UserAccountPointResponse)

    def search(self, keyword: str, limit: int = 99999):
        """Search for manga titles.

        Parameters
        ----------
        keyword: :class:`str`
            The keyword to search for.
        limit: :class:`int`
            Limit the response, by default ``99999``

        Returns
        -------
        :class:`list[TitleNode]`
            The list of manga titles.

        Raises
        ------
        :exc:`.exceptions.KMAPIError`
            If the response is not successful.
        """

        response = self.request(
            "GET",
            f"{self.API_HOST}/search/title",
            params={"keyword": keyword, "limit": str(limit)},
        )

        parsed = self._make_response(response, type=SearchResponse)
        return parsed.title_list

    def get_weekly(self):
        """Get the weekly list.

        Returns
        -------
        :class:`WeeklyListResponse`
            The weekly manga responses entity.

        Raises
        ------
        :exc:`.exceptions.KMAPIError`
            If the response is not successful.
        """

        response = self.request(
            "GET",
            f"{self.API_HOST}/title/weekly",
        )

        return self._make_response(response, type=WeeklyListResponse)

    def get_account(self):
        """Get the current user's account information.

        Returns
        -------
        :class:`AccountResponse`
            The user account information.

        Raises
        ------
        :exc:`.exceptions.KMAPIError`
            If the response is not successful.
        """

        response = self.request(
            "GET",
            f"{self.API_HOST}/account",
        )

        return self._make_response(response, type=AccountResponse)

    def get_purchased(self):
        """Get the user's purchased titles.

        Returns
        -------
        :class:`TitleListResponse`
            The user's purchased titles responses entity.

        Raises
        ------
        :exc:`.exceptions.KMAPIError`
            If the response is not successful.
        """

        response = self.request(
            "GET",
            f"{self.API_HOST}/web/title/purchased",
        )

        return self._make_response(response, type=TitlePurchaseResponse)

    def get_magazines(self):
        """Get the magazines list.

        Returns
        -------
        :class:`MagazineCategoryResponse`
            The magazines list responses entity.

        Raises
        ------
        :exc:`.exceptions.KMAPIError`
            If the response is not successful.
        """

        response = self.request(
            "GET",
            f"{self.API_HOST}/magazine/category/list",
            params={"limit": "99999", "offset": "0"},
        )

        return self._make_response(response, type=MagazineCategoryResponse)

    def get_genre_list(self):
        """Get the genre list.

        Returns
        -------
        :class:`GenreSearchResponse`
            The genre list responses entity.

        Raises
        ------
        :exc:`.exceptions.KMAPIError`
            If the response is not successful.
        """

        response = self.request(
            "GET",
            f"{self.API_HOST}/genre/search/list",
        )

        return self._make_response(response, type=GenreSearchResponse)

    def get_all_rankings(self, ranking_id: int, *, limit: int = 101, offset: int = 0):
        """Get manga rankings for a specific ranking ID.

        Parameters
        ----------
        ranking_id: :class:`int`
            The ranking ID.
        limit: :class:`int`
            Limit the response, by default ``101``
        offset: :class:`int`
            Offset the response, basically the pagination, by default ``0``

        Returns
        -------
        :class:`RankingListResponse`
            The ranking list responses entity.

        Raises
        ------
        :exc:`.exceptions.KMAPIError`
            If the response is not successful.
        """

        response = self.request(
            "GET",
            f"{self.API_HOST}/ranking/all",
            params={"ranking_id": str(ranking_id), "offset": str(offset), "limit": str(limit)},
        )

        return self._make_response(response, type=RankingListResponse)
