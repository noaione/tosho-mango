//! A module containing information related to user account.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use serde::{Deserialize, Serialize};
use tosho_macros::AutoGetter;

/// User purchase information
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct IAPInfo {
    /// Bonus ticket
    bonus: u64,
    /// Purchased ticket
    #[serde(rename = "product")]
    purchased: u64,
    /// Premium ticket
    premium: u64,
    /// Point that you have
    #[serde(rename = "pp")]
    point: u64,
    /// New bonus ticket
    new_bonus: u64,
    /// The request payload
    payload: String,
    #[serde(rename = "next_pp_second")]
    next_point_second: u64,
    #[serde(rename = "next_pp_time")]
    next_point_time: u64,
    #[serde(rename = "next_pp")]
    next_point: u64,
    available_wall: bool,
    /// The account identifier for the user
    ///
    /// This is different between each token.
    guest_id: String,
}

impl IAPInfo {
    /// Get the total number of tickets
    pub fn sum(&self) -> u64 {
        self.bonus + self.purchased + self.premium
    }

    /// Get the total number of points
    pub fn sum_point(&self) -> u64 {
        self.point + self.new_bonus
    }

    /// Set premium ticket amount
    pub fn set_premium(&mut self, premium: u64) {
        self.premium = premium;
    }

    /// Set bonus ticket amount
    pub fn set_bonus(&mut self, bonus: u64) {
        self.bonus = bonus;
    }

    /// Set purchased ticket amount
    pub fn set_purchased(&mut self, purchased: u64) {
        self.purchased = purchased;
    }

    /// Subtract premium ticket amount
    pub fn subtract_premium(&mut self, premium: u64) {
        self.premium = self.premium.saturating_sub(premium);
    }

    /// Subtract bonus ticket amount
    pub fn subtract_bonus(&mut self, bonus: u64) {
        self.bonus = self.bonus.saturating_sub(bonus);
    }

    /// Subtract purchased ticket amount
    pub fn subtract_purchased(&mut self, purchased: u64) {
        self.purchased = self.purchased.saturating_sub(purchased);
    }
}

/// A node of each available purchase product
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct IAPProductInfoNode {
    /// The product identifier
    #[serde(rename = "product_id")]
    id: String,
    /// The product notice (sometimes is the name or description)
    notice: String,
}

/// A wrapper for [`IAPProductInfoNode`]
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct IAPProductInfo {
    /// The product information
    #[serde(rename = "iap_product_info")]
    info: IAPProductInfoNode,
}

/// A complete in-app purchase information
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct IAPRemainder {
    /// The in-app purchase information
    #[serde(rename = "iap_info")]
    info: IAPInfo,
    /// The in-app purchase product list
    #[serde(rename = "iap_product_list")]
    product_list: Option<Vec<IAPProductInfo>>,
    /// The in-app purchase product version
    #[serde(rename = "iap_product_version")]
    version: Option<u64>,
}

/// The result of a login request
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct LoginResult {
    /// The account ID
    #[serde(rename = "citi_id")]
    id: u64,
    /// The account username
    #[serde(rename = "p_name")]
    name: String,
    /// The account image URL
    #[serde(rename = "profile_img_url")]
    image_url: String,
    /// Is the account a guest account?
    temp: bool,
    /// The login message, if any
    login_message: Option<String>,
    /// In-app purchase information
    #[serde(rename = "iap_info")]
    info: IAPInfo,
}

/// A minimal user account information
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct AccountUserInfo {
    /// The account ID
    #[serde(rename = "citi_id")]
    id: u64,
    /// The account username
    #[serde(rename = "p_name")]
    name: String,
    /// The account image URL
    #[serde(rename = "prof_image_url")]
    image_url: String,
}

/// Response for user account information
#[derive(Debug, Clone, AutoGetter, Serialize, Deserialize)]
pub struct AccountUserResponse {
    /// The account information
    #[serde(rename = "user_info")]
    info: AccountUserInfo,
}
