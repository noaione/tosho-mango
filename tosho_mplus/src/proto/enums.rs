//! A module containing information related to enums used in the library.
//!
//! If something is missing, please [open an issue](https://github.com/noaione/tosho-mango/issues/new/choose) or a [pull request](https://github.com/noaione/tosho-mango/compare).

use tosho_macros::EnumName;

/// The error action or error code of the request.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
pub enum ErrorAction {
    /// An error has occurred.
    Unrecognized = -1,
    /// Some default or other unknown error.
    Default = 0,
    /// The request is unauthorized.
    Unauthorized = 1,
    /// Server is under maintenance.
    Maintenance = 2,
    /// The request is blocked by GeoIP.
    GeoIPBlocked = 3,
}

/// Enum for the available language in the source.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, EnumName, ::prost::Enumeration,
)]
pub enum Language {
    /// Unknown language.
    Unrecognized = -1,
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
    Italian = 8,
    /// Vietnamese language.
    Vietnamese = 9,
}

impl Language {
    /// Get the pretty name of the language.
    ///
    /// # Examples
    /// ```
    /// use tosho_mplus::proto::Language;
    ///
    /// let pt_br = Language::BrazilianPortuguese;
    ///
    /// assert_eq!(ssunday.pretty_name(), "Brazilian Portuguese");
    /// ```
    pub fn pretty_name(&self) -> String {
        let name = self.to_name();
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

        let mut merge_back = splitted_name.join(" ");
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
}
