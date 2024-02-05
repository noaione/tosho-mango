use documented::DocumentedFields;
use serde::{Deserialize, Serialize};
use tosho_macros::{
    DeserializeEnum32, DeserializeEnum32Fallback, EnumCount, EnumName, EnumU32Fallback,
    SerializeEnum32,
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

/// The purchase status of an episode.
#[derive(Debug, Clone, SerializeEnum32, DeserializeEnum32, PartialEq, EnumName)]
pub enum EpisodeBadge {
    /// Episode need to be purchased by point or ticket (if possible)
    Purchaseable = 1,
    /// Episode is free to be viewed
    Free = 2,
    /// Episode is purchased
    Purchased = 3,
    /// Episode is on rental
    Rental = 4,
}

/// The device platform type.
#[derive(Debug, Clone, SerializeEnum32, DeserializeEnum32, PartialEq, EnumName)]
pub enum DevicePlatform {
    // Is Apple/iOS
    Apple = 1,
    // Is Android
    Android = 2,
    /// Is Website
    Web = 3,
}

/// Gender type of the user.
#[derive(Debug, Clone, SerializeEnum32, DeserializeEnum32, PartialEq, EnumName)]
pub enum GenderType {
    Male = 1,
    Female = 2,
    Other = 3,
}

/// The publication category type.
#[derive(Debug, Clone, SerializeEnum32, DeserializeEnum32, PartialEq, EnumName)]
pub enum PublishCategory {
    /// Series is being serialized
    Serializing = 1,
    /// Series is complete!
    Complete = 2,
    ReadingOut = 3,
}

/// The magazine category type.
#[derive(
    Debug,
    Clone,
    SerializeEnum32,
    DeserializeEnum32Fallback,
    PartialEq,
    EnumName,
    EnumCount,
    EnumU32Fallback,
    Default,
    DocumentedFields,
)]
pub enum MagazineCategory {
    /// Unknown magazine.
    #[default]
    Undefined = 0,
    /// KM Original series.
    Original = 1,
    /// Weekly Shounen Magazine.
    WeeklyShounenMagazine = 2,
    /// Bessatsu Shounen Magazine, a spin-off WSM (Monthly).
    BessatsuShounenMagazine = 3,
    /// Gravure magazine.
    Gravure = 4,
    /// Misc comic/book.
    Misc = 5,
    /// Misc magazine.
    MiscMagazine = 6,
    /// Magazine from another company.
    OtherCompany = 7,
    /// Monthly Shounen Sirius.
    MonthlyShounenSirius = 8,
    /// Suiyobi no Sirius, a spin-off MSS, released every Wednesday.
    SuiyobiShounenSirius = 9,
    /// Monthly Shounen Magazine.
    MonthlyShounenMagazine = 10,
    /// Shounen Magazine R, a supplement magazine for WSM. (Discontinued)
    ///
    /// Originally a bi-monthly magazine, but now it's a monthly digital-only magazine.
    ShounenMagazineR = 11,
    /// Bekkan Getsumaga.
    BekkanGetsumaga = 12,
    /// Weekly Young Magazine, Seinen-focused magazine.
    WeeklyYoungMagazine = 13,
    /// Weekly Young Magazine the 3rd, a supplementary for WYM.
    WeeklyYoungMagazineThe3rd = 14,
    /// Monthly Young Magazine, a sister magazine of WYM.
    MonthlyYoungMagazine = 15,
    /// Shounen Magazine Edge, a monthly magazine.
    ShounenMagazineEdge = 16,
    /// Morning magazine, a weekly seinen magazine.
    Morning = 17,
    /// Morning Two, a monthly version of Morning.
    MorningTwo = 18,
    /// Afternoon magazine, a monthly seinen magazine.
    Afternoon = 19,
    /// Good! Afternoon, sister magazine of Afternoon, a monthly seinen magazine.
    GoodAfternoon = 20,
    /// Evening magazine, a bi-weekly seinen magazine, discontinued and moved some to Comic Days.
    Evening = 21,
    /// Comic BomBom, a monthly kids magazine.
    ComicBombom = 22,
    /// e-Young Magazine, a digital-only of Young magazine which focused on user-submitted serialized content.
    #[allow(non_camel_case_types)] // used since we have a unique way to extract name.
    eYoungMagazine = 23,
    /// Digital Morning, a digital-only version of Morning magazine.
    DMorning = 24,
    /// Comic Days, a digital-only seinen magazine.
    ComicDays = 25,
    /// Palcy, a digital-only magazine collaboration with Pixiv.
    Palcy = 26,
    /// Cycomi, a digital-only magazine collaboration with Cygames.
    Cycomi = 27,
    /// Manga Box, a mobile app for manga from Kodansha, Shogakukan, and other publishers.
    MangaBox = 28,
    /// Nakayoshi/Good Friend, a monthly shoujo magazine.
    Nakayoshi = 29,
    /// Bessatsu Friend, a monthly shoujo magazine.
    BessatsuFriend = 30,
    /// Dessert, a monthly shoujo/josei magazine.
    Dessert = 31,
    /// Kiss, a monthly josei magazine.
    Kiss = 32,
    /// Hatsu Kiss, a monthly josei magazine (originally bi-monthly until 2018).
    HatsuKiss = 33,
    /// Be Love, a monthly josei magazine.
    BeLove = 34,
    /// Honey Milk. a web magazine focused on Boys Love.
    HoneyMilk = 35,
    /// Ane Friend, a web shoujo/josei magazine.
    AneFriend = 36,
    /// Comic Tint, a web shoujo/josei magazine.
    ComicTint = 37,
    /// Kodansha Gakujutsu Bunko or Kodansha Academic Paperback Library.
    GakujutsuBunko = 38,
    /// Seikaisha or Star Seas Company, a subsidiary of Kodansha.
    Seikaisha = 39,
    /// Baby Mofu, a website focused on books and manga related to childcare.
    BabyMofu = 40,
    /// Ichijinsha, a subsidiary of Kodansha.
    ///
    /// Owns the following:
    /// - Febri
    /// - Comic Rex (4-koma)
    /// - Monthly Comic Zero Sum (Josei focused)
    /// - Comic Yuri Hime (Girls Love)
    /// - gateau
    /// - IDOLM@STER Million Live Magazine Plus+
    Ichijinsha = 41,
    /// Comic Create, a web comic magazine.
    ComicCreate = 42,
    /// Comic Bull, a magazine by Sports Bull.
    ComicBull = 43,
    /// Young Magazine Web, a digital-only magazine for Young Magazine.
    YoungMagazineWeb = 44,
    /// White Heart, a digital-only magazine for josei manga.
    WhiteHeart = 45,
    /// Monthly Magazine Base, a digital-only magazine for shounen manga, replacement for Shounen Magazine R.
    MonthlyMagazineBase = 46,
    /// Kodansha Light Novel imprint/label.
    LightNovel = 47,
    /// Article/Report published by Kodansha.
    Article = 48,
}

impl MagazineCategory {
    /// Get the pretty name of the magazine category.
    ///
    /// # Examples
    /// ```
    /// use tosho_kmkc::models::MagazineCategory;
    ///
    /// let e_young = MagazineCategory::eYoungMagazine;
    ///
    /// assert_eq!(e_young.pretty_name(), "e-Young Magazine");
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

    /// Get the documentation or docstring of the current magazine category.
    pub fn get_doc(&self) -> Result<&'static str, documented::Error> {
        Self::get_field_comment(self.to_name())
    }
}

/// The favorite status of the titles.
#[derive(Debug, Clone, SerializeEnum32, DeserializeEnum32, PartialEq, EnumName)]
pub enum FavoriteStatus {
    /// Title is not favorited.
    None = 0,
    /// Title is favorited.
    Favorite = 1,
    /// Title is purchased?
    Purchased = 2,
}

/// The support status of the titles.
#[derive(Debug, Clone, SerializeEnum32, DeserializeEnum32, PartialEq, EnumName)]
pub enum SupportStatus {
    /// Not allowed to support the titles.
    NotAllowed = 0,
    /// Allowed to support the titles.
    Allowed = 1,
    /// Already supported the titles.
    Supported = 2,
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::MagazineCategory;

    #[derive(Debug, Deserialize, Serialize)]
    struct TestMagJson {
        magazine: MagazineCategory,
    }

    #[test]
    fn test_magazine_serde() {
        // test deserialize
        let wsm_json = r#"{"magazine": 2}"#;
        let wsm: TestMagJson = serde_json::from_str(wsm_json).unwrap();

        assert_eq!(wsm.magazine, MagazineCategory::WeeklyShounenMagazine);
        // test serialize
        let wsm_ser = serde_json::to_string(&wsm).unwrap();
        assert_eq!(wsm_ser, r#"{"magazine":2}"#);
    }

    #[test]
    fn test_magazine_serde_fallback() {
        // test deserialize for fallback
        let unknown_json = r#"{"magazine": 100}"#;
        let unknown: TestMagJson = serde_json::from_str(unknown_json).unwrap();

        assert_eq!(unknown.magazine, MagazineCategory::Undefined);
        // test serialize for fallback
        let unknown_ser = serde_json::to_string(&unknown).unwrap();
        assert_eq!(unknown_ser, r#"{"magazine":0}"#);
    }

    #[test]
    fn test_magazine_category_pretty() {
        let e_young = super::MagazineCategory::eYoungMagazine;
        let digital_morning = super::MagazineCategory::DMorning;
        let comic_bull = super::MagazineCategory::ComicBull;
        let weekly_shounen_magazine = super::MagazineCategory::WeeklyShounenMagazine;

        assert_eq!(e_young.pretty_name(), "e-Young Magazine");
        assert_eq!(digital_morning.pretty_name(), "Digital Morning");
        assert_eq!(comic_bull.pretty_name(), "Comic Bull");
        assert_eq!(
            weekly_shounen_magazine.pretty_name(),
            "Weekly Shounen Magazine"
        );
    }

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

    #[test]
    fn test_enum_doc_for_magazine() {
        let magazine = MagazineCategory::eYoungMagazine;
        assert_eq!(magazine.get_doc(), Ok("e-Young Magazine, a digital-only of Young magazine which focused on user-submitted serialized content."));

        let ichijinsha_doc = "Ichijinsha, a subsidiary of Kodansha.

Owns the following:
- Febri
- Comic Rex (4-koma)
- Monthly Comic Zero Sum (Josei focused)
- Comic Yuri Hime (Girls Love)
- gateau
- IDOLM@STER Million Live Magazine Plus+";

        let ichijinsha = MagazineCategory::Ichijinsha;
        assert_eq!(ichijinsha.get_doc(), Ok(ichijinsha_doc));
    }

    #[test]
    fn test_enum_count() {
        assert_eq!(MagazineCategory::count(), 49);
    }
}
