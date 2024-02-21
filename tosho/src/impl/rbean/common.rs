use tosho_rbean::RBClient;

use crate::config::save_config;

use super::config::Config;

pub(super) fn save_session_config(client: &RBClient, config: &Config) {
    let mut config = config.clone();
    config.access_token = client.get_token().to_string();
    if let Some(expiry_at) = client.get_expiry_at() {
        config.expiry = expiry_at;
    }

    save_config(config.into(), None);
}
