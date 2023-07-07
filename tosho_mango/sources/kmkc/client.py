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

from .constants import API_HOST, API_UA, BASE_HOST, DEVICE_PLATFORM, DEVICE_VERSION, HASH_HEADER
from .dto import (
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
from .errors import KMAPIError, KMNotEnoughPointError
from .models import KMConfig

__all__ = ("KMClient",)


def hash_kv(key: str, value: str):
    key_b = key.encode("utf-8")
    val_b = value.encode("utf-8")

    key_hash = sha256(key_b).hexdigest()
    val_hash = sha512(val_b).hexdigest()
    return f"{key_hash}_{val_hash}"


class KMClient:
    API_HOST = b64decode("aHR0cHM6Ly9hcGkua21hbmdhLmtvZGFuc2hhLmNvbQ==").decode("utf-8")
    CDN_HOST = b64decode("aHR0cHM6Ly9jZG4ua21hbmdhLmtvZGFuc2hhLmNvbQ==").decode("utf-8")

    def __init__(self, config: KMConfig) -> None:
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

    def get_episode_list(self, episodes: list[int]):
        """
        Get episode list from episode ids
        """

        data, headers = self._format_request({"episode_id_list": ",".join(map(str, episodes))})
        response = self._client.post(
            f"{self.API_HOST}/episode/list",
            data=data,
            headers=headers,
        )

        parsed = msgspec.json.decode(response.content, type=EpisodesListResponse)
        if parsed.status != "success":
            raise KMAPIError(parsed.response_code, parsed.error_message)

        return parsed.episode_list

    def get_title_list(self, titles: list[int]):
        """
        Get title list from title IDs.
        """

        data, headers = self._format_request({"title_id_list": ",".join(map(str, titles))})
        response = self._client.get(
            f"{self.API_HOST}/title/list",
            params=data,
            headers=headers,
        )

        parsed = msgspec.json.decode(response.content, type=TitleListResponse)
        if parsed.status != "success":
            raise KMAPIError(parsed.response_code, parsed.error_message)

        return parsed.title_list

    def get_web_chapter_viewer(self, episode_id: int):
        """
        Get web chapter viewer from episode ID.
        """

        params, headers = self._format_request({"episode_id": str(episode_id)})
        response = self._client.get(
            f"{self.API_HOST}/web/episode/viewer",
            params=params,
            headers=headers,
        )

        parsed = msgspec.json.decode(response.content, type=WebChapterViewerResponse)
        if parsed.status != "success":
            raise KMAPIError(parsed.response_code, parsed.error_message)

        return parsed

    def get_title_ticket(self, title_id: int) -> TitleTicketListEntry:
        params, headers = self._format_request({"title_id_list": str(title_id)})
        response = self._client.get(
            f"{self.API_HOST}/title/ticket/list",
            params=params,
            headers=headers,
        )

        parsed = msgspec.json.decode(response.content, type=TitleTicketListResponse)
        if parsed.status != "success":
            raise KMAPIError(parsed.response_code, parsed.error_message)

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

        data, headers = self._format_request(
            form_data,
            {
                "Content-Type": "application/x-www-form-urlencoded",
            },
        )
        response = self._client.post(
            f"{self.API_HOST}/episode/rental/ticket",
            data=data,
            headers=headers,
        )

        parsed = msgspec.json.decode(response.content, type=StatusResponse)
        if parsed.status != "success":
            raise KMAPIError(parsed.response_code, parsed.error_message)

        return parsed, is_title

    def claim_episode_with_point(self, episode: EpisodeEntry, wallet: UserPoint):
        if not wallet.can_purchase(episode.point):
            raise KMNotEnoughPointError

        form_data = {
            "episode_id": str(episode.episode_id),
            "check_point": str(episode.point),
        }
        data, headers = self._format_request(
            form_data,
            {
                "Content-Type": "application/x-www-form-urlencoded",
            },
        )
        response = self._client.post(
            f"{self.API_HOST}/episode/paid",
            data=data,
            headers=headers,
        )

        parsed = msgspec.json.decode(response.content, type=EpisodePurchaseResponse)
        if parsed.status != "success":
            raise KMAPIError(parsed.response_code, parsed.error_message)

        wallet.subtract(episode.point)
        wallet.paid_point += episode.bonus_point
        return wallet

    def get_user_point(self):
        params, headers = self._format_request()
        response = self._client.get(
            f"{self.API_HOST}/account/point",
            params=params,
            headers=headers,
        )

        parsed = msgspec.json.decode(response.content, type=UserAccountPointResponse)
        if parsed.status != "success":
            raise KMAPIError(parsed.response_code, parsed.error_message)

        return parsed

    def search(self, keyword: str, limit: int = 99999):
        params, headers = self._format_request({"keyword": keyword, "limit": str(limit)})
        response = self._client.get(
            f"{self.API_HOST}/search/title",
            params=params,
            headers=headers,
        )

        parsed = msgspec.json.decode(response.content, type=SearchResponse)
        if parsed.status != "success":
            raise KMAPIError(parsed.response_code, parsed.error_message)

        return parsed.title_list

    def get_weekly(self):
        params, headers = self._format_request()
        responses = self._client.get(
            f"{self.API_HOST}/title/weekly",
            params=params,
            headers=headers,
        )

        parsed = msgspec.json.decode(responses.content, type=WeeklyListResponse)
        if parsed.status != "success":
            raise KMAPIError(parsed.response_code, parsed.error_message)

        return parsed
