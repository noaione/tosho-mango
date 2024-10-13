//! A module containing information related to enums used in the library.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use tosho_macros::EnumName;

/// The error action or error code of the request.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
pub enum ErrorAction {
    /// Some default or other unknown error.
    Default = 0,
    /// The request is unauthorized.
    Unauthorized = 1,
    /// Server is under maintenance.
    Maintenance = 2,
    /// The request is blocked by GeoIP.
    GeoIPBlocked = 3,
    /// An error has occurred.
    Unrecognized = -1,
}

/// Enum for the available language in the source.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, EnumName, ::prost::Enumeration,
)]
pub enum Language {
    /// English language.
    English = 0,
    /// Spanish language.
    Spanish = 1,
    /// French language.
    French = 2,
    /// Indonesian language.
    Indonesian = 3,
    /// Brazilian Portuguese language.
    BrazilianPortuguese = 4,
    /// Russian language.
    Russian = 5,
    /// Thai language.
    Thai = 6,
    /// German language.
    German = 7,
    /// Italian language.
    ///
    /// This language is currently unused.
    Italian = 8,
    /// Vietnamese language.
    Vietnamese = 9,
    /// Unknown language.
    Unrecognized = -1,
}

impl Language {
    /// Get the pretty name of the language.
    ///
    /// # Examples
    /// ```rust
    /// use tosho_mplus::proto::Language;
    ///
    /// let pt_br = Language::BrazilianPortuguese;
    ///
    /// assert_eq!(pt_br.pretty_name(), "Brazilian Portuguese");
    /// ```
    pub fn pretty_name(&self) -> String {
        pretty_name_fmt(self.to_name(), " ")
    }

    /// Get the language code for the language.
    ///
    /// # Examples
    /// ```rust
    /// use tosho_mplus::proto::Language;
    ///
    /// let english = Language::English;
    /// assert_eq!(english.as_language_code(), "eng");
    /// ```
    pub fn as_language_code(&self) -> &'static str {
        match self {
            Language::English => "eng",
            Language::Spanish => "spa",
            Language::French => "fra",
            Language::Indonesian => "ind",
            Language::BrazilianPortuguese => "ptb",
            Language::Russian => "rus",
            Language::Thai => "tha",
            Language::German => "deu",
            Language::Italian => "ita",
            Language::Vietnamese => "vie",
            Language::Unrecognized => "unknown",
        }
    }

    /// Get the country code for the language.
    ///
    /// # Examples
    /// ```rust
    /// use tosho_mplus::proto::Language;
    ///
    /// let english = Language::English;
    /// assert_eq!(english.as_country_code(), "en");
    /// ```
    pub fn as_country_code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Spanish => "es",
            Language::French => "fr",
            Language::Indonesian => "in",
            Language::BrazilianPortuguese => "pt",
            Language::Russian => "ru",
            Language::Thai => "th",
            Language::German => "de",
            Language::Italian => "it",
            Language::Vietnamese => "vi",
            Language::Unrecognized => "unknown",
        }
    }
}

/// The title update status
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
pub enum TitleUpdateStatus {
    /// No update available.
    None = 0,
    /// New title.
    New = 1,
    /// Updated chapter available.
    Updated = 2,
    /// Re-edition/Re-release chapter available.
    ReEdition = 3,
    /// Update for creator manga
    Creator = 4,
    /// An error has occurred.
    Unrecognized = -1,
}

/// The page type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
pub enum PageType {
    /// Single page
    Single = 0,
    /// Left-side of the page (on double page)
    Left = 1,
    /// Right-side of the page (on double page)
    Right = 2,
    /// A merged spread page
    Double = 3,
    /// An error has occurred.
    Unrecognized = -1,
}

/// The chapter type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
pub enum ChapterType {
    /// Latest chapter
    Latest = 0,
    /// Current chapter in a sequence (not latest)
    Sequence = 1,
    /// No sequence, just a chapter
    NoSequence = 2,
    /// An error has occurred.
    Unrecognized = -1,
}

/// Enums for update profile result
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
pub enum UpdateProfileResult {
    /// Profile updated successfully
    Success = 0,
    /// Another profile with the same name already exists
    Duplicate = 1,
    /// The name contains disallowed words
    BadName = 2,
    /// An error has occurred.
    Unrecognized = -1,
}

/// Feedback type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
pub enum FeedbackType {
    /// Feedback is a question (original poster)
    Question = 0,
    /// Feedback is a answer to a question
    Answer = 1,
    /// An error has occurred.
    Unrecognized = -1,
}

/// Plan offer type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
pub enum PlanOfferType {
    /// No offer
    Free = 0,
    /// Introductory offer to the plan
    Introductory = 1,
    /// Promotional offer to the plan
    Promotional = 2,
    /// An error has occurred.
    Unrecognized = -1,
}

/// Title release schedule
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, EnumName, ::prost::Enumeration,
)]
pub enum TitleReleaseSchedule {
    /// No schedule
    None = 0,
    /// Daily/everyday schedule
    Daily = 1,
    /// Weekly schedule
    Weekly = 2,
    /// Bi-weekly schedule
    BiWeekly = 3,
    /// Monthly schedule
    Monthly = 4,
    /// Bi-monthly schedule
    BiMonthly = 5,
    /// Tri-monthly schedule
    TriMonthly = 6,
    /// Other schedule
    Other = 7,
    /// Series is completed
    Completed = 8,
    /// An error has occurred.
    Unrecognized = -1,
}

impl TitleReleaseSchedule {
    /// Get the pretty name of the release schedule.
    ///
    /// # Examples
    /// ```rust
    /// use tosho_mplus::proto::TitleReleaseSchedule;
    ///
    /// let biweekly = TitleReleaseSchedule::BiWeekly;
    ///
    /// assert_eq!(biweekly.pretty_name(), "Bi-Weekly");
    /// ```
    pub fn pretty_name(&self) -> String {
        pretty_name_fmt(self.to_name(), "-")
    }
}

/// Title rating
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
pub enum TitleRating {
    /// All ages rating
    AllAges = 0,
    /// Teen rating
    Teen = 1,
    /// Teen+ rating
    TeenPlus = 2,
    /// Mature rating
    Mature = 3,
    /// An error has occurred.
    Unrecognized = -1,
}

fn pretty_name_fmt(name: &str, sep: &str) -> String {
    let split_at_upper: Vec<_> = name.match_indices(char::is_uppercase).collect();
    let mut splitted_name: Vec<&str> = vec![];
    split_at_upper
        .iter()
        .enumerate()
        .for_each(|(i, (start, _))| {
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
        });

    let mut merge_back = splitted_name.join(sep);
    let some_words = &["The", "A", "An"];
    some_words.iter().for_each(|&word| {
        merge_back =
            merge_back.replace(&format!(" {}", word), &format!(" {}", word.to_lowercase()));
    });

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

/// Chapter position in the series chapters list group
///
/// This is a custom enum that does not exist originally in the API.
/// Made to make it easier to understand the position of the chapter in the series.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
pub enum ChapterPosition {
    /// Chapter located in the middle of the chapter list (usually need subs or tickets)
    Middle = 0,
    /// Chapter located in first chapter list (usually free)
    First = 1,
    /// Chapter located in last chapter list (usually free)
    Last = 2,
    /// An error has occurred.
    Unrecognized = -1,
}
