//! A module containing information related to some others response models.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use serde::{Deserialize, Serialize};
use tosho_macros::AutoGetter;

/// Response for publishers list
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct PublishersList {
    /// Total pages available using the current page size
    #[serde(rename = "pages_count")]
    pages: u32,
    /// Total publishers available
    #[serde(rename = "total_count")]
    count: u64,
    /// List of publishers
    #[serde(rename = "publishers")]
    data: Vec<super::common::Publisher>,
}

/// Response for publisher imprints list
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ImprintsList {
    /// Total pages available using the current page size
    #[serde(rename = "pages_count")]
    pages: u32,
    /// Total imprints available
    #[serde(rename = "total_count")]
    count: u64,
    /// List of imprints
    #[serde(rename = "publisher_imprints")]
    data: Vec<super::common::Imprint>,
}

/// Response for genres list
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct GenresList {
    /// Total pages available using the current page size
    #[serde(rename = "pages_count")]
    pages: u32,
    /// Total genres available
    #[serde(rename = "total_count")]
    count: u64,
    /// List of genres
    #[serde(rename = "genres")]
    data: Vec<super::common::Genre>,
}

/// Response for creators list
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct CreatorsList {
    /// Total pages available using the current page size
    #[serde(rename = "pages_count")]
    pages: u32,
    /// Total creators available
    #[serde(rename = "total_count")]
    count: u64,
    /// List of creators
    #[serde(rename = "creators")]
    data: Vec<super::common::Creator>,
}

/// Marketplace books information
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct MarketplaceBook {
    /// Issue ID
    id: u32,
    /// Issue UUID
    uuid: String,
    /// The series title
    title: String,
    /// The series + issue full title
    full_title: String,
    /// The issue URL slug
    slug: String,
    /// The publisher information
    publisher: super::common::Publisher,
    /// The imprint information
    #[serde(rename = "publisher_imprint")]
    imprint: Option<super::common::Imprint>,
    /// The issue cover image URLs
    cover: super::common::ImageUrl,
    /// The issue release date in ISO 8601 format
    #[serde(with = "super::datetime")]
    release_date: chrono::DateTime<chrono::FixedOffset>,
    /// The minimum price of the issue in USD
    #[serde(rename = "edition_price_min")]
    minimum_price: u64,
    /// The maximum price of the issue in USD
    #[serde(rename = "edition_price_max")]
    maximum_price: u64,
    /// Total available editions of this issue
    total_editions: u32,
    /// Total editions available in marketplace
    #[serde(rename = "editions_count_in_marketplace")]
    marketplace_count: u32,
    /// Total editions available in marketplace which is remarque
    #[serde(rename = "editions_count_in_marketplace_with_rmq")]
    marketplace_remarque_count: u32,
    /// When is this issue was added to marketplace
    #[serde(rename = "marketplace_added_date", with = "super::datetime")]
    created_at: chrono::DateTime<chrono::FixedOffset>,
}

/// Response for marketplace books list
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct MarketplaceBooksList {
    /// Total pages available using the current page size
    #[serde(rename = "pages_count")]
    pages: u32,
    /// Total books available in marketplace
    #[serde(rename = "total_count")]
    count: u64,
    /// List of books in marketplace
    #[serde(rename = "books")]
    data: Vec<MarketplaceBook>,
}

/// Marketplace edition seller information
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct MarketplaceSeller {
    /// User ID of the seller
    uuid: String,
    /// The username of the seller
    username: String,
    /// The email of the seller (why is this even here???)
    email: String,
    /// The seller first name
    first_name: Option<String>,
    /// The seller last name
    last_name: Option<String>,
    // TODO: image cover (but it's an empty object in response)
}

/// Marketplace edition information
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct MarketplaceEdition {
    /// Marketplace edition ID
    id: String,
    /// The issue UUID
    #[serde(rename = "book_id")]
    issue_uuid: String,
    /// The issue index number
    #[serde(rename = "book_index")]
    index: u32,
    /// The price of the edition in USD
    #[serde(rename = "marketplace_price")]
    price_usd: u64,
    /// Marketplace creation date in ISO 8601 format
    #[serde(rename = "marketplace_created_at", with = "super::datetime")]
    created_at: chrono::DateTime<chrono::FixedOffset>,
    /// The seller information
    #[serde(rename = "owner")]
    seller: MarketplaceSeller,
    /// Seller notes
    #[serde(rename = "seller_notes")]
    notes: Option<String>,
    /// Has signature/remarque
    #[serde(rename = "sig")]
    signature: bool,
    /// The cover URL with the signature/remarque + watermarking
    #[serde(rename = "remarqued_watermark_cover_url")]
    remarque_cover: Option<String>,
}

/// Response for marketplace edition list
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct MarketplaceEditionsList {
    /// Total pages available using the current page size
    #[serde(rename = "pages_count")]
    pages: u32,
    /// Total editions available in marketplace
    #[serde(rename = "total_count")]
    count: u64,
    /// List of editions in marketplace
    #[serde(rename = "editions")]
    data: Vec<MarketplaceEdition>,
}

/// The collected editions information
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct CollectedEdition {
    /// The edition ID
    id: String,
    /// The book index
    #[serde(rename = "book_index")]
    index: u32,
    /// Is in marketplace?
    in_marketplace: bool,
    /// Does have signature/remarque?
    #[serde(rename = "sig")]
    has_signature: bool,
    // TODO: `rmq_plate_url` and `remarqued_watermark_cover_url` fields
}

/// The paginated response for collected editions listing.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct CollectedEditionList {
    /// Total pages available using the current page size
    #[serde(rename = "pages_count")]
    pages: u32,
    /// Total editions owned
    #[serde(rename = "total_count")]
    count: u64,
    /// List of collected editions
    #[serde(rename = "editions")]
    data: Vec<CollectedEdition>,
    /// The issue information of these editions
    issue: super::issues::IssueDetail,
}

/// The response for publisher detail request
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct PublisherDetailResponse {
    /// The actual publisher information
    #[serde(rename = "publisher")]
    #[deref_clone]
    data: super::common::Publisher,
}

/// Customer payment method information
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct CustomerPaymentMethod {
    /// The payment method ID
    id: String,
    /// Stripe payment method ID
    pm_id: String,
    /// Stripe payment method type (e.g. "card")
    pm_type: String,
    /// The card brand (e.g. "visa", "mastercard", etc.)
    brand: String,
    /// The last 4 digits of the card
    last4: String,
    /// Expiry month of the card
    exp_month: String,
    /// Expiry year of the card
    exp_year: String,
    /// Country of the card
    country: Option<String>,
}

/// Customer detailed information
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct CustomerDetail {
    /// The customer ID
    #[serde(rename = "uuid")]
    id: String,
    /// The customer email
    email: String,
    /// The customer user name
    username: Option<String>,
    /// The customer first name
    first_name: Option<String>,
    /// The customer last name
    last_name: Option<String>,
    /// The customer waitlist position
    place_number: u64,
    /// The customer referral code
    referral_code: String,
    /// The customer referral code usage count
    #[serde(rename = "referrals_count")]
    referral_uses: u64,
    /// The customer payment method
    payment_method: Option<CustomerPaymentMethod>,
    /// The customer balance in USD
    ///
    /// Stripe normalizeds balance to cents
    balance: u64,
    /// The customer role (e.g. "Beta")
    roles: String,
    // TODO: `image` field (but it's an empty object in response)
}

/// The response for customer detail request
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct CustomerDetailResponse {
    /// The actual customer information
    #[serde(rename = "customer")]
    #[deref_clone]
    data: CustomerDetail,
}

/// The collected editions information
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct CollectedEditionDetailed {
    /// The edition ID
    id: String,
    /// The book index
    #[serde(rename = "book_index")]
    index: u32,
    /// The issue UUID
    book_id: String,
    /// The issue full title
    full_title: String,
    /// Is in marketplace?
    in_marketplace: bool,
    /// The original cover image URL
    original_url: String,
    /// The mobile cover image URL
    mobile_url: String,
    /// The thumbnail cover image URL
    thumbnail_url: String,
    /// Variant identifier (e.g. "standard", "variant-1", "A", etc.)
    #[serde(rename = "variant_identifier")]
    variant_id: Option<String>,
    /// Variant description (e.g. "Standard Edition", "Variant Cover A", etc.)
    variant_description: String,
    /// Is this edition signable/remarqueable?
    signable: bool,
    /// Is this edition resellable in marketplace?
    resellable: bool,
    /// Is this edition downloadable?
    downloadable: bool,
    /// The age rating of the issue (e.g. "13+")
    age_rating: String,
    /// The language of the issue (e.g. "eng", "jpn", etc.)
    language: String,
    /// The release date of the issue in ISO 8601 format
    #[serde(with = "super::datetime")]
    release_date: chrono::DateTime<chrono::FixedOffset>,
    /// Total pages in the issue
    total_pages: u32,
    /// The genres/tags associated with this issue
    #[serde(default, rename = "categories")]
    genres: Vec<super::common::Genre>,
    /// The creators involved in this issue
    #[serde(default)]
    creators: Vec<super::issues::IssueCreator>,
    /// Does this edition have signature/remarque?
    #[serde(rename = "sig")]
    has_signature: bool,
    // TODO: `rmq_plate_url` and `remarqued_watermark_cover_url` fields
}

/// An information about an series run from reading history
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ReadingHistorySeries {
    /// The series UUID
    id: String,
    /// The series title
    title: String,
    /// The series description
    description: Option<String>,
    /// The series URL slug
    slug: String,
    /// The series creation time in the system
    #[serde(with = "super::datetime")]
    created_at: chrono::DateTime<chrono::FixedOffset>,
}

/// The information for user reading history
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ReadingHistory {
    /// The issue ID
    id: String,
    /// The issue UUID
    uuid: String,
    /// The series title
    title: String,
    /// The issue full title
    full_title: String,
    /// The issue URL slug
    slug: String,
    /// The issue description
    description: Option<String>,
    /// The issue cover image original URL
    original_url: String,
    /// The issue cover image mobile URL
    mobile_url: String,
    /// The issue cover image thumbnail URL
    thumbnail_url: String,
    /// The series run information for this issue
    series_run: ReadingHistorySeries,
    /// The list of owned editions of this issue
    #[serde(rename = "owned_editions")]
    editions: Vec<CollectedEditionDetailed>,
    /// Total pages in this issue
    total_pages: u32,
    /// The currently read page number if any
    #[serde(default)]
    bookmark_page: Option<u32>,
    /// The timestamp of the last time this issue was read/viewed
    #[serde(
        default,
        rename = "bookmark_timestamp",
        serialize_with = "super::datetime::serialize_opt",
        deserialize_with = "super::datetime::deserialize_opt"
    )]
    last_read: Option<chrono::DateTime<chrono::FixedOffset>>,
}

/// The paginated response for user reading history listing.
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct ReadingHistoryList {
    /// Total pages available using the current page size
    #[serde(rename = "pages_count")]
    pages: u32,
    // /// Total history items available
    // #[serde(rename = "total_count")]
    // count: u64,
    /// List of reading history items
    #[serde(rename = "books")]
    data: Vec<ReadingHistory>,
}
