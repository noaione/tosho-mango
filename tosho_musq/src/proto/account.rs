#![allow(clippy::derive_partial_eq_without_eq)]

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
