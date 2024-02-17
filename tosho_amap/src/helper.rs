//! Provides a collection of helper Structs that can be used.

use crate::models::{ComicEpisodeInfoNode, ComicInfo, IAPInfo};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

/// A comic purchase info.
///
/// Created from a [`ComicInfo`], [`ComicEpisodeInfoNode`], and [`IAPInfo`].
#[derive(Clone, Debug, Default)]
pub struct ComicPurchase {
    /// The comic episode ID
    pub id: u64,
    /// The comic episode rental term
    pub rental_term: Option<String>,
    /// Bonus tickets used
    pub bonus: u64,
    /// Purchased tickets used
    pub purchased: u64,
    /// Premium tickets used
    pub premium: u64,
    /// Points used
    pub point: Option<u64>,
    /// Use free daily ticket
    pub is_free_daily: bool,
}

impl ComicPurchase {
    /// Create a new [`ComicPurchase`] from a [`ComicInfo`], [`ComicEpisodeInfoNode`], and [`IAPInfo`].
    ///
    /// Returns `None` if the episode is not purchasable.
    pub fn from_episode_and_comic(
        comic: &ComicInfo,
        episode: &ComicEpisodeInfoNode,
        account: &mut IAPInfo,
    ) -> Option<Self> {
        let id = episode.id;
        let rental_term = comic.rental_term.clone();

        let price = episode.price;

        let bonus = account.bonus;
        let purchased = account.purchased;
        let premium = account.premium;
        let point = account.point;

        let is_free_daily = episode.is_free_daily;

        if is_free_daily {
            return Some(Self {
                id,
                rental_term,
                bonus: 0,
                purchased: 0,
                premium: 0,
                point: None,
                is_free_daily,
            });
        }

        if price == 0 {
            return Some(Self {
                id,
                rental_term,
                bonus: 0,
                purchased: 0,
                premium: 0,
                point: None,
                is_free_daily,
            });
        }

        if price > bonus + purchased + premium + point {
            return None;
        }

        // Priority of payment methods:
        // 1. Bonus
        // 2. Product
        // 3. Premium
        // 4. Point

        let mut cost = price;
        // since it's u64, we can't have negative values
        cost = cost.saturating_sub(bonus);
        if cost == 0 {
            return Some(Self {
                id,
                rental_term,
                bonus: bonus - price,
                purchased,
                premium,
                point: None,
                is_free_daily,
            });
        }

        cost = cost.saturating_sub(purchased);
        if cost == 0 {
            return Some(Self {
                id,
                rental_term,
                bonus,
                purchased: purchased - price,
                premium,
                point: None,
                is_free_daily,
            });
        }

        cost = cost.saturating_sub(premium);
        if cost == 0 {
            return Some(Self {
                id,
                rental_term,
                bonus,
                purchased,
                premium: premium - price,
                point: None,
                is_free_daily,
            });
        }

        cost = cost.saturating_sub(point);
        if cost == 0 {
            Some(Self {
                id,
                rental_term,
                bonus,
                purchased,
                premium,
                point: Some(point - price),
                is_free_daily,
            })
        } else {
            None
        }
    }
}

/// Generate a string of random characters used for token.
///
/// The length of the string is 16.
pub(crate) fn generate_random_token() -> String {
    let rng = thread_rng();
    let token = rng.sample_iter(&Alphanumeric).take(16).collect::<Vec<u8>>();

    String::from_utf8(token).unwrap().to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_token() {
        let token = generate_random_token();
        println!("{}", token);
        assert_eq!(token.len(), 16);
    }
}
