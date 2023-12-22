#![allow(clippy::derive_partial_eq_without_eq)]

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserPoint {
    /// Free/daily coins that you have.
    #[prost(uint64, tag = "1")]
    pub free: u64,
    /// Event/XP coins that you have.
    #[prost(uint64, tag = "2")]
    pub event: u64,
    /// Paid coins that you have.
    #[prost(uint64, tag = "3")]
    pub paid: u64,
}

impl UserPoint {
    /// Returns the total amount of points.
    ///
    /// # Examples
    /// ```
    /// use tosho_musq::proto::account::UserPoint;
    ///
    /// let points = UserPoint {
    ///    free: 100,
    ///    event: 200,
    ///    paid: 300,
    /// };
    ///
    /// assert_eq!(points.sum(), 600);
    /// ```
    pub fn sum(&self) -> u64 {
        self.free + self.event + self.paid
    }
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountDevice {
    /// The device ID.
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// The device name.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// The device installation date in unix timestamp.
    #[prost(uint64, tag = "3")]
    pub install_at: u64,
}

/// The account view response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountView {
    /// The list of devices that you have logged in.
    #[prost(message, repeated, tag = "1")]
    pub devices: ::prost::alloc::vec::Vec<AccountDevice>,
    /// Whether or not you have registered your account.
    #[prost(bool, optional, tag = "2")]
    pub registered: ::core::option::Option<bool>,
    /// The login URL to connect your account.
    #[prost(string, tag = "3")]
    pub login_url: ::prost::alloc::string::String,
}

/// The setting view response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SettingView {
    /// The bridge tag name.
    #[prost(string, tag = "1")]
    pub tag_name: ::prost::alloc::string::String,
    /// The bridge keyword.
    #[prost(string, tag = "2")]
    pub keyword: ::prost::alloc::string::String,
}
