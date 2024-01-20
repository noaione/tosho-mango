use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IAPInfo {
    /// Bonus ticket
    pub bonus: u64,
    /// Purchased ticket
    #[serde(rename = "product")]
    pub purchased: u64,
    /// Premium ticket
    pub premium: u64,
    /// Point that you have
    #[serde(rename = "pp")]
    pub point: u64,
    pub new_bonus: u64,
    pub payload: String,
    #[serde(rename = "next_pp_second")]
    pub next_point_second: u64,
    #[serde(rename = "next_pp_time")]
    pub next_point_time: u64,
    #[serde(rename = "next_pp")]
    pub next_point: u64,
    pub available_wall: bool,
    pub guest_id: String,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IAPProductInfoNode {
    #[serde(rename = "product_id")]
    pub id: String,
    pub notice: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IAPProductInfo {
    #[serde(rename = "iap_product_info")]
    pub info: IAPProductInfoNode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IAPRemainder {
    #[serde(rename = "iap_info")]
    pub info: IAPInfo,
    #[serde(rename = "iap_product_list")]
    pub product_list: Option<Vec<IAPProductInfo>>,
    #[serde(rename = "iap_product_version")]
    pub version: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResult {
    #[serde(rename = "citi_id")]
    pub id: u64,
    #[serde(rename = "p_name")]
    pub name: String,
    #[serde(rename = "profile_img_url")]
    pub image_url: String,
    pub temp: bool,
    pub login_message: Option<String>,
    #[serde(rename = "iap_info")]
    pub info: IAPInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountUserInfo {
    #[serde(rename = "citi_id")]
    pub id: u64,
    #[serde(rename = "p_name")]
    pub name: String,
    #[serde(rename = "prof_image_url")]
    pub image_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountUserResponse {
    #[serde(rename = "user_info")]
    pub info: AccountUserInfo,
}
