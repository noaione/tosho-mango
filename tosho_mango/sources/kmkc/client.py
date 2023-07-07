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
from hashlib import sha256, sha512
from http.cookiejar import Cookie, CookieJar
from urllib.parse import quote

import msgspec
import requests

from tosho_mango.sources.kmkc.config import KMConfigWeb, save_config

from .constants import API_HOST, API_UA, BASE_HOST, DEVICE_PLATFORM, DEVICE_VERSION, HASH_HEADER
from .dto import (
    AccountResponse,
    BulkEpisodePurchaseResponse,
    EpisodeEntry,
    EpisodePurchaseResponse,
    EpisodesListResponse,
    PremiumTicketInfo,
    SearchResponse,
    StatusResponse,
    TitleListResponse,
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


def hash_kv(key: str, value: str):
    key_b = key.encode("utf-8")
    val_b = value.encode("utf-8")

    key_hash = sha256(key_b).hexdigest()
    val_hash = sha512(val_b).hexdigest()
    return f"{key_hash}_{val_hash}"


class KMClientWeb:
    API_HOST = b64decode("aHR0cHM6Ly9hcGkua21hbmdhLmtvZGFuc2hhLmNvbQ==").decode("utf-8")
    CDN_HOST = b64decode("aHR0cHM6Ly9jZG4ua21hbmdhLmtvZGFuc2hhLmNvbQ==").decode("utf-8")

    def __init__(self, config: KMConfigWeb) -> None:
        self._config = config

        self._client = requests.Session()
        self._client.headers.update(
            {
                "User-Agent": API_UA,
                "Host": API_HOST,
                "accept": "application/json",
            }
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
            )
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
            )
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
            )
        )

        cookie_jar.set_cookie(
            Cookie(
                version=0,
                name="uwt",
                value=self._config.uwt,
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
            )
        )

        return cookie_jar

    @property
    def client(self):
        return self._client

    def _create_request_hash(self, query_params: dict[str, str]):
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
        extend_query = {
            "platform": DEVICE_PLATFORM,
            "version": DEVICE_VERSION,
            **(query or {}),
        }
        extend_headers = {
            **(headers or {}),
            HASH_HEADER: self._create_request_hash(extend_query),
        }
        return extend_query, extend_headers

    def request(self, method: str, url: str, **kwargs):
        data = kwargs.pop("data", None)
        headers = kwargs.pop("headers", None)
        params = kwargs.pop("params", None)

        fmt_data, fmt_headers = self._format_request(data, headers)
        fmt_params, _ = self._format_request(params)

        key_param = "data"
        if data is None and params is None:
            # Assume params
            fmt_data = fmt_params
            key_param = "params"
        elif data is None and params is not None:
            # Assume params
            fmt_data = fmt_params
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
        self._config.apply_cookies(requested.cookies)
        save_config(self._config)
        return requested

    def get_episode_list(self, episodes: list[int]):
        """
        Get episode list from episode ids
        """

        response = self.request(
            "POST",
            f"{self.API_HOST}/episode/list",
            data={"episode_id_list": ",".join(map(str, episodes))},
        )

        parsed = msgspec.json.decode(response.content, type=EpisodesListResponse)
        parsed.raise_for_status()

        return parsed.episode_list

    def get_title_list(self, titles: list[int]):
        """
        Get title list from title IDs.
        """

        response = self.request(
            "GET",
            f"{self.API_HOST}/title/list",
            params={"title_id_list": ",".join(map(str, titles))},
        )

        parsed = msgspec.json.decode(response.content, type=TitleListResponse)
        parsed.raise_for_status()

        return parsed.title_list

    def get_chapter_viewer(self, episode_id: int):
        """
        Get chapter viewer from episode ID.
        """

        response = self.request(
            "GET",
            f"{self.API_HOST}/web/episode/viewer",
            params={"episode_id": str(episode_id)},
        )

        parsed = msgspec.json.decode(response.content, type=WebChapterViewerResponse)
        parsed.raise_for_status()

        return parsed

    def get_title_ticket(self, title_id: int) -> TitleTicketListEntry:
        response = self.request(
            "GET",
            f"{self.API_HOST}/title/ticket/list",
            params={"title_id_list": str(title_id)},
        )

        parsed = msgspec.json.decode(response.content, type=TitleTicketListResponse)
        parsed.raise_for_status()

        return parsed.title_ticket_list[0]

    def claim_episode_with_ticket(self, episode_id: int, ticket: TitleTicketInfo | PremiumTicketInfo):
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

        parsed = msgspec.json.decode(response.content, type=StatusResponse)
        parsed.raise_for_status()

        return parsed, is_title

    def claim_episode_with_point(self, episode: EpisodeEntry, wallet: UserPoint):
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

        parsed = msgspec.json.decode(response.content, type=EpisodePurchaseResponse)
        parsed.raise_for_status()

        wallet.subtract(parsed.paid_point)
        wallet.add(episode.bonus_point)
        return wallet

    def claim_bulk_episode(self, episodes: list[EpisodeEntry], wallet: UserPoint):
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

        parsed = msgspec.json.decode(response.content, type=BulkEpisodePurchaseResponse)
        parsed.raise_for_status()
        wallet.subtract(parsed.paid_point)
        wallet.add(parsed.earned_point_back)
        return wallet

    def get_user_point(self):
        response = self.request(
            "GET",
            f"{self.API_HOST}/account/point",
        )

        parsed = msgspec.json.decode(response.content, type=UserAccountPointResponse)
        parsed.raise_for_status()

        return parsed

    def search(self, keyword: str, limit: int = 99999):
        response = self.request(
            "GET",
            f"{self.API_HOST}/search/title",
            params={"keyword": keyword, "limit": str(limit)},
        )

        parsed = msgspec.json.decode(response.content, type=SearchResponse)
        parsed.raise_for_status()

        return parsed.title_list

    def get_weekly(self):
        response = self.request(
            "GET",
            f"{self.API_HOST}/title/weekly",
        )

        parsed = msgspec.json.decode(response.content, type=WeeklyListResponse)
        parsed.raise_for_status()

        return parsed

    def get_account(self):
        response = self.request(
            "GET",
            f"{self.API_HOST}/account",
        )

        parsed = msgspec.json.decode(response.content, type=AccountResponse)
        parsed.raise_for_status()

        return parsed

    def get_purchased(self):
        response = self.request(
            "GET",
            f"{self.API_HOST}/web/title/purchased",
        )

        parsed = msgspec.json.decode(response.content, type=TitleListResponse)
        parsed.raise_for_status()

        return parsed
