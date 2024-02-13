use rand::{distributions::Alphanumeric, thread_rng, Rng};

/// Generate a string of random characters used for token.
///
/// The length of the string is 16.
pub(crate) fn generate_random_token() -> String {
    let rng = thread_rng();
    let token = rng.sample_iter(&Alphanumeric).take(16).collect::<Vec<u8>>();

    String::from_utf8(token).unwrap().to_lowercase()
}
