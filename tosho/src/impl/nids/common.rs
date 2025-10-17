use chrono::Datelike;
use clap::ValueEnum;
use color_print::cformat;
use num_format::{Locale, ToFormattedString};
use tosho_nids::{constants::BASE_HOST, filters::FilterType};

use crate::linkify;

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
    OnSale,
    BestSelling,
    NewReleases,
}

impl ValueEnum for FilterScopeInput {
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            FilterScopeInput::Frontlist => Some(clap::builder::PossibleValue::new("frontlist")),
            FilterScopeInput::Backlist => Some(clap::builder::PossibleValue::new("backlist")),
            FilterScopeInput::OnSale => Some(clap::builder::PossibleValue::new("on-sale")),
            FilterScopeInput::BestSelling => {
                Some(clap::builder::PossibleValue::new("best-selling"))
            }
            FilterScopeInput::NewReleases => {
                Some(clap::builder::PossibleValue::new("new-releases"))
            }
        }
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[
            FilterScopeInput::Frontlist,
            FilterScopeInput::Backlist,
            FilterScopeInput::OnSale,
            FilterScopeInput::BestSelling,
            FilterScopeInput::NewReleases,
        ]
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
            "on-sale" | "onsale" | "on_sale" => Ok(FilterScopeInput::OnSale),
            "best-selling" | "bestselling" | "best_selling" => Ok(FilterScopeInput::BestSelling),
            "new-releases" | "newreleases" | "new_releases" => Ok(FilterScopeInput::NewReleases),
            _ => Err(format!("Invalid scope: {s}")),
        }
    }
}

impl From<FilterScopeInput> for tosho_nids::filters::FilterScope {
    fn from(value: FilterScopeInput) -> Self {
        match value {
            FilterScopeInput::Frontlist => tosho_nids::filters::FilterScope::Frontlist,
            FilterScopeInput::Backlist => tosho_nids::filters::FilterScope::Backlist,
            FilterScopeInput::OnSale => tosho_nids::filters::FilterScope::OnSale,
            FilterScopeInput::BestSelling => tosho_nids::filters::FilterScope::BestSelling,
            FilterScopeInput::NewReleases => tosho_nids::filters::FilterScope::NewReleases,
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
    Ok((
        FilterType::from_string(filtered),
        parts[1].trim().to_string(),
    ))
}

pub(super) fn parse_sort_by(s: &str) -> Result<SortByInput, String> {
    let filtered = alphabetical_filter(s)?;
    Ok(tosho_nids::filters::SortBy::from_string(filtered))
}

pub(super) fn fmt_date(date: &chrono::DateTime<chrono::FixedOffset>) -> String {
    // Mon DD, YYYY
    date.format("%b %d, %Y").to_string()
}

pub(crate) fn get_scope_dates() -> (String, String) {
    let now = chrono::Utc::now();
    // get maximum time in current day
    let end_of_day = now
        .date_naive()
        .and_hms_opt(23, 59, 59)
        .unwrap_or_else(|| now.naive_utc());
    let start_of_day = now
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap_or_else(|| now.naive_utc());
    // minus start_of_day by 7 days
    let start_date = start_of_day - chrono::Duration::days(7);
    // format as RFC3339
    (
        start_date
            .and_local_timezone(chrono::Utc)
            .unwrap()
            .to_rfc3339(),
        end_of_day
            .and_local_timezone(chrono::Utc)
            .unwrap()
            .to_rfc3339(),
    )
}

pub(super) enum PaginateAction {
    Next,
    Previous,
    Exit(u32),
}

pub(super) async fn pagination_helper(
    current_page: u32,
    maximum_pages: u32,
    console: &crate::term::Terminal,
) -> PaginateAction {
    let mut options = vec![];
    if current_page > 1 {
        options.push(crate::term::ConsoleChoice::new(
            "prev",
            format!("Previous Page ({}/{})", current_page - 1, maximum_pages),
        ));
    }
    if current_page < maximum_pages {
        options.push(crate::term::ConsoleChoice::new(
            "next",
            format!("Next Page ({}/{})", current_page + 1, maximum_pages),
        ));
    }
    options.push(crate::term::ConsoleChoice::new("exit", "Exit Pagination"));

    let selection = console.choice("What do you want to do?", options);
    match selection {
        Some(choice) => match choice.name.as_str() {
            "next" => PaginateAction::Next,
            "prev" => PaginateAction::Previous,
            "exit" => PaginateAction::Exit(0),
            _ => {
                console.warn("Invalid choice, exiting.");
                PaginateAction::Exit(1)
            }
        },
        None => {
            console.warn("Aborted by user, exiting.");
            PaginateAction::Exit(0)
        }
    }
}

pub(super) fn format_series_run_date(
    start_date: Option<&chrono::NaiveDate>,
    end_date: Option<&chrono::NaiveDate>,
) -> Option<String> {
    match (start_date, end_date) {
        (Some(start), Some(end)) => {
            // Do year only if same year, else do month, year formatting
            if start.year() == end.year() {
                Some(format!("{} – {}", start.format("%Y"), end.format("%Y")))
            } else {
                Some(format!(
                    "{} – {}",
                    start.format("%b %Y"),
                    end.format("%b %Y")
                ))
            }
        }
        (Some(start), None) => Some(format!("{} – Present", start.format("%b %Y"))),
        (None, Some(end)) => Some(format!("Unknown – {}", end.format("%b %Y"))),
        (None, None) => None,
    }
}

pub(super) fn format_series_run(
    title: &str,
    start_date: Option<&chrono::NaiveDate>,
    end_date: Option<&chrono::NaiveDate>,
) -> String {
    match format_series_run_date(start_date, end_date) {
        Some(date_str) => format!("{} ({})", title, date_str),
        None => title.to_string(),
    }
}

pub(super) fn print_series_summary(
    series: &tosho_nids::models::SeriesRunDetailed,
    console: &crate::term::Terminal,
    is_owned: bool,
) {
    let item_url = if is_owned {
        format!("https://{}/mycollection/{}", BASE_HOST, series.uuid())
    } else {
        format!(
            "https://{}/series/{}/{}",
            BASE_HOST,
            series.id(),
            series.slug()
        )
    };

    let linked_title = linkify!(&item_url, series.title());
    let id_pair = if is_owned {
        cformat!("<m,s>{}</m,s> / {}", series.uuid(), series.id())
    } else {
        cformat!("<m,s>{}</m,s> / {}", series.id(), series.uuid())
    };

    console.info(cformat!("  <s>{}</s> ({})", linked_title, id_pair));
    let mut series_smols = vec![cformat!("<b,s>{}</b,s>", series.publisher().name())];
    if let Some(run) = format_series_run_date(series.start_date(), series.end_date()) {
        series_smols.push(cformat!("<s,dim>{}</s,dim>", run));
    }
    match series.issues_count() {
        0 => {}
        1 => series_smols.push(cformat!("<s>1</s> issue")),
        n => series_smols.push(cformat!(
            "<s>{}</s> issues",
            n.to_formatted_string(&Locale::en)
        )),
    };

    console.info(format!("   {}", item_url));
    console.info(format!("   {}", series_smols.join(" | ")));
}
