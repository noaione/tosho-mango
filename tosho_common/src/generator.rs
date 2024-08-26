use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

/// Generate a string of random characters used for token and other ID.
///
/// This will return an all lowercase string of X characters.
pub fn generate_random_token(count: usize) -> String {
    let rng = thread_rng();
    let token: String = rng
        .sample_iter(&Alphanumeric)
        .take(count)
        .map(char::from)
        .collect();

    token.to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_token() {
        let token = generate_random_token(16);
        println!("{}", token);
        assert_eq!(token.len(), 16);
    }
}
