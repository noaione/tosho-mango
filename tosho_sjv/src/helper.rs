use aho_corasick::AhoCorasick;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

/// Generate a string of random characters used for token.
///
/// The length of the string is 16.
#[allow(dead_code)]
pub(crate) fn generate_random_token() -> String {
    let rng = thread_rng();
    let token = rng.sample_iter(&Alphanumeric).take(16).collect::<Vec<u8>>();

    String::from_utf8(token).unwrap().to_lowercase()
}

/// Search the big cache JSON for specific title
///
/// TODO: Change contents to proper models
#[allow(dead_code)]
pub(crate) fn search_by_text(contents: Vec<String>, target: &str) -> Vec<String> {
    // split by spaces
    let target: Vec<&str> = target.split_ascii_whitespace().collect();

    let ac = AhoCorasick::builder()
        .ascii_case_insensitive(true)
        .build(target)
        .unwrap();

    let mut matches = vec![];
    for content in contents {
        if ac.find(&content).is_some() {
            matches.push(content);
        }
    }

    matches
}
