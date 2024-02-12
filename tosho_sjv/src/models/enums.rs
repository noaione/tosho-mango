use std::str::FromStr;

use serde::{Deserialize, Serialize};
use tosho_macros::{
    enum_error, DeserializeEnum, DeserializeEnum32, DeserializeEnum32Fallback, EnumName,
    EnumU32Fallback, SerializeEnum, SerializeEnum32,
};

/// A boolean type used by the API represented as an integer.
#[derive(Debug, Clone, SerializeEnum32, DeserializeEnum32, EnumName)]
pub enum IntBool {
    /// Property is false
    False = 0,
    /// Property is true
    True = 1,
    /// Property is unknown
    Unknown = -1,
}

impl std::fmt::Display for IntBool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntBool::False => write!(f, "False"),
            IntBool::True => write!(f, "True"),
            IntBool::Unknown => write!(f, "Unknown"),
        }
    }
}

impl PartialEq<IntBool> for IntBool {
    fn eq(&self, other: &IntBool) -> bool {
        matches!(
            (self, other),
            (IntBool::False, IntBool::False)
                | (IntBool::True, IntBool::True)
                | (IntBool::Unknown, IntBool::Unknown)
        )
    }
}

impl PartialEq<i32> for IntBool {
    fn eq(&self, other: &i32) -> bool {
        match self {
            IntBool::False => *other == 0,
            IntBool::True => *other == 1,
            IntBool::Unknown => *other == -1,
        }
    }
}

impl PartialEq<bool> for IntBool {
    fn eq(&self, other: &bool) -> bool {
        match self {
            IntBool::True => *other,
            IntBool::False => !(*other),
            _ => false,
        }
    }
}

impl From<IntBool> for bool {
    fn from(item: IntBool) -> Self {
        matches!(item, IntBool::True)
    }
}

/// The subscription type
///
/// ```
/// use tosho_sjv::models::SubscriptionType;
///
/// let st = SubscriptionType::VM;
/// assert_eq!(st.to_string(), "vm");
/// assert_eq!(st.to_name(), "VM");
///
/// let parsed = "sj".parse::<SubscriptionType>().unwrap();
/// assert_eq!(parsed, SubscriptionType::SJ);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, SerializeEnum, DeserializeEnum, EnumName)]
pub enum SubscriptionType {
    /// VM (Manga) subs type
    VM,
    /// SJ (Jump) subs type
    SJ,
}

enum_error!(SubscriptionTypeFromStrError);

impl FromStr for SubscriptionType {
    type Err = SubscriptionTypeFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "vm" => Ok(SubscriptionType::VM),
            "sj" => Ok(SubscriptionType::SJ),
            _ => Err(SubscriptionTypeFromStrError {
                original: s.to_string(),
            }),
        }
    }
}

impl ToString for SubscriptionType {
    fn to_string(&self) -> String {
        match self {
            SubscriptionType::SJ => "sj".to_string(),
            SubscriptionType::VM => "vm".to_string(),
        }
    }
}

/// The manga rating
///
/// ```
/// use tosho_sjv::models::MangaRating;
///
/// let st = MangaRating::AllAges;
/// assert_eq!(st.to_string(), "a");
/// assert_eq!(st.to_name(), "AllAges");
///
/// let parsed = "tp".parse::<MangaRating>().unwrap();
/// assert_eq!(parsed, MangaRating::TeenPlus);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, SerializeEnum, DeserializeEnum, EnumName)]
pub enum MangaRating {
    /// All ages
    ///
    /// May be suitable for readers or consumers of any age.
    /// For example, may contain mild language and fantasy violence but no swearing or nudity.
    AllAges,
    /// Teen
    ///
    /// May be suitable for early teens and older.
    /// For example, may contain violence, infrequent use of strong language, suggestive themes or situations, crude humor, alcohol and/or tobacco use.
    Teen,
    /// Teen Plus
    ///
    /// May be suitable for older teens and adults.
    /// For example, may contain intense and/or gory violence, sexual content, frequent strong language, alcohol, tobacco and/or other substance use.
    TeenPlus,
    /// Mature
    ///
    /// Suitable for adults only. May contain extreme violence, mature themes and graphic depictions.
    Mature,
}

enum_error!(MangaRatingFromStrError);

impl FromStr for MangaRating {
    type Err = MangaRatingFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "a" => Ok(MangaRating::AllAges),
            "t" => Ok(MangaRating::Teen),
            "tp" => Ok(MangaRating::TeenPlus),
            "m" => Ok(MangaRating::Mature),
            _ => Err(MangaRatingFromStrError {
                original: s.to_string(),
            }),
        }
    }
}

impl ToString for MangaRating {
    fn to_string(&self) -> String {
        match self {
            MangaRating::AllAges => "a".to_string(),
            MangaRating::Teen => "t".to_string(),
            MangaRating::TeenPlus => "tp".to_string(),
            MangaRating::Mature => "m".to_string(),
        }
    }
}

/// The manga imprint type.
#[derive(
    Debug,
    Clone,
    SerializeEnum32,
    DeserializeEnum32Fallback,
    PartialEq,
    EnumName,
    EnumU32Fallback,
    Default,
)]
pub enum MangaImprint {
    /// Unknown imprint.
    #[default]
    Undefined = 0,
    /// Shonen Jump, based on the popular Weekly Shonen Jump by Shueisha
    ShonenJump = 1,
    /// Shojo Beat, a more girl focused imprint
    ShojoBeat = 2,
    /// Weekly Shonen Sunday, a Shogakukan imprint or magazine
    ShonenSunday = 3,
    /// V Signature
    VSignature = 4,
    /// Shonen Jump Advacned, a more older teenage and young adult focused imprint.
    ShonenJumpAdvanced = 5,
    /// V Media, a general purpose label/imprint
    VM = 6,
    /// V Kids, a list for manga targeted for general audience or kids.
    VKids = 7,
    /// V Select, a curated list by the publisher.
    VSelect = 8,
    /// Haikasoru, a Space opera. Dark fantasy. Hard science related manga.
    ///
    /// The best in Japanese science fiction, fantasy and horror.
    Haikasoru = 9,
}

impl MangaImprint {
    /// Get the pretty name of the imprint category.
    ///
    /// # Examples
    /// ```
    /// use tosho_sjv::models::MangaImprint;
    ///
    /// let ssunday = MangaImprint::ShonenSunday;
    ///
    /// assert_eq!(ssunday.pretty_name(), "Shonen Sunday");
    /// ```
    pub fn pretty_name(&self) -> String {
        let name = self.to_name();
        let split_at_upper: Vec<_> = name.match_indices(char::is_uppercase).collect();
        let mut splitted_name: Vec<&str> = vec![];
        for (i, (start, _)) in split_at_upper.iter().enumerate() {
            if i == 0 {
                let data = &name[..*start];
                if !data.is_empty() {
                    splitted_name.push(data);
                }
            }
            let next_start = split_at_upper.get(i + 1);
            match next_start {
                Some((end, _)) => splitted_name.push(&name[*start..*end]),
                None => splitted_name.push(&name[*start..]),
            }
        }

        let mut merge_back = splitted_name.join(" ");
        for word in &["The", "A", "An"] {
            merge_back =
                merge_back.replace(&format!(" {}", *word), &format!(" {}", word.to_lowercase()))
        }

        if splitted_name.len() > 1 {
            match splitted_name[0] {
                "e" => merge_back = merge_back.replacen("e ", "e-", 1),
                "D" => merge_back = merge_back.replacen("D ", "Digital ", 1),
                _ => (),
            }
        }
        if merge_back.contains('_') {
            merge_back = merge_back.replace('_', " ");
        }
        merge_back
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_int_bool() {
        let falsy = super::IntBool::False;
        let truthy = super::IntBool::True;
        let unknown = super::IntBool::Unknown;

        assert_eq!(falsy, 0);
        assert_eq!(truthy, 1);
        assert_eq!(unknown, -1);
    }

    #[test]
    fn test_int_bool_if() {
        let truthy = super::IntBool::True;

        if truthy.into() {
            assert!(true);
        } else {
            assert!(false);
        }

        let falsy = super::IntBool::False;
        if falsy.into() {
            assert!(false);
        } else {
            assert!(true);
        }
    }
}
