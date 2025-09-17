use clap::ValueEnum;
use tosho_nids::filters::FilterType;

pub(super) type FilterPairInput = (FilterType, String);
pub(super) type SortByInput = tosho_nids::filters::SortBy;

const FILTER_CONTROL: &[char; 2] = &['_', '-'];

#[derive(Clone)]
pub(crate) enum SortOrderInput {
    Asc,
    Desc,
}

impl ValueEnum for SortOrderInput {
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            SortOrderInput::Asc => Some(clap::builder::PossibleValue::new("asc")),
            SortOrderInput::Desc => Some(clap::builder::PossibleValue::new("desc")),
        }
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[SortOrderInput::Asc, SortOrderInput::Desc]
    }

    fn from_str(s: &str, ignore_case: bool) -> Result<Self, String> {
        let s = if ignore_case {
            s.to_lowercase()
        } else {
            s.to_string()
        };
        match s.as_str() {
            "asc" => Ok(SortOrderInput::Asc),
            "desc" => Ok(SortOrderInput::Desc),
            "ASC" => Ok(SortOrderInput::Asc),
            "DESC" => Ok(SortOrderInput::Desc),
            _ => Err(format!("Invalid sort order: {s}")),
        }
    }
}

#[derive(Clone)]
pub(crate) enum FilterScopeInput {
    Frontlist,
    Backlist,
}

impl ValueEnum for FilterScopeInput {
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            FilterScopeInput::Frontlist => Some(clap::builder::PossibleValue::new("frontlist")),
            FilterScopeInput::Backlist => Some(clap::builder::PossibleValue::new("backlist")),
        }
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[FilterScopeInput::Frontlist, FilterScopeInput::Backlist]
    }

    fn from_str(s: &str, ignore_case: bool) -> Result<Self, String> {
        let s = if ignore_case {
            s.to_lowercase()
        } else {
            s.to_string()
        };
        match s.as_str() {
            "frontlist" => Ok(FilterScopeInput::Frontlist),
            "backlist" => Ok(FilterScopeInput::Backlist),
            _ => Err(format!("Invalid scope: {s}")),
        }
    }
}

impl From<FilterScopeInput> for tosho_nids::filters::FilterScope {
    fn from(value: FilterScopeInput) -> Self {
        match value {
            FilterScopeInput::Frontlist => tosho_nids::filters::FilterScope::Frontlist,
            FilterScopeInput::Backlist => tosho_nids::filters::FilterScope::Backlist,
        }
    }
}

impl From<SortOrderInput> for tosho_nids::filters::SortOrder {
    fn from(value: SortOrderInput) -> Self {
        match value {
            SortOrderInput::Asc => tosho_nids::filters::SortOrder::ASC,
            SortOrderInput::Desc => tosho_nids::filters::SortOrder::DESC,
        }
    }
}

fn alphabetical_filter(s: &str) -> Result<String, String> {
    let s = s.trim().to_lowercase();
    if s.is_empty() {
        return Err("cannot be empty".to_string());
    }

    if s.chars()
        .all(|c| c.is_ascii_alphabetic() || FILTER_CONTROL.contains(&c))
    {
        Ok(s)
    } else {
        Err("must only contain alphabetical characters, hyphens or underscores".to_string())
    }
}

pub(super) fn parse_filter_pairs(s: &str) -> Result<FilterPairInput, String> {
    let parts: Vec<&str> = s.splitn(2, '=').collect();
    if parts.len() != 2 {
        return Err(format!("Invalid filter format: {s}"));
    }

    let filtered = alphabetical_filter(parts[0])?;
    Ok((FilterType::from_str(filtered), parts[1].trim().to_string()))
}

pub(super) fn parse_sort_by(s: &str) -> Result<SortByInput, String> {
    let filtered = alphabetical_filter(s)?;
    Ok(tosho_nids::filters::SortBy::from_str(filtered))
}
