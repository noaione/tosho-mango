use clap::ValueEnum;
use tosho_macros::EnumName;

pub(crate) type CommaSeparatedNumber = Vec<usize>;

#[derive(Clone, EnumName)]
pub enum WeeklyCodeCli {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl ValueEnum for WeeklyCodeCli {
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            WeeklyCodeCli::Sunday => Some(clap::builder::PossibleValue::new("sun")),
            WeeklyCodeCli::Monday => Some(clap::builder::PossibleValue::new("mon")),
            WeeklyCodeCli::Tuesday => Some(clap::builder::PossibleValue::new("tue")),
            WeeklyCodeCli::Wednesday => Some(clap::builder::PossibleValue::new("wed")),
            WeeklyCodeCli::Thursday => Some(clap::builder::PossibleValue::new("thu")),
            WeeklyCodeCli::Friday => Some(clap::builder::PossibleValue::new("fri")),
            WeeklyCodeCli::Saturday => Some(clap::builder::PossibleValue::new("sat")),
        }
    }

    #[cfg(not(tarpaulin_include))]
    fn value_variants<'a>() -> &'a [Self] {
        &[
            WeeklyCodeCli::Sunday,
            WeeklyCodeCli::Monday,
            WeeklyCodeCli::Tuesday,
            WeeklyCodeCli::Wednesday,
            WeeklyCodeCli::Thursday,
            WeeklyCodeCli::Friday,
            WeeklyCodeCli::Saturday,
        ]
    }

    fn from_str(input: &str, ignore_case: bool) -> Result<Self, String> {
        let s = if ignore_case {
            input.to_lowercase()
        } else {
            input.to_string()
        };

        match s.as_str() {
            "sun" => Ok(WeeklyCodeCli::Sunday),
            "mon" => Ok(WeeklyCodeCli::Monday),
            "tue" => Ok(WeeklyCodeCli::Tuesday),
            "wed" => Ok(WeeklyCodeCli::Wednesday),
            "thu" => Ok(WeeklyCodeCli::Thursday),
            "fri" => Ok(WeeklyCodeCli::Friday),
            "sat" => Ok(WeeklyCodeCli::Saturday),
            _ => Err(format!("Invalid weekly code: {}", input)),
        }
    }
}

impl From<WeeklyCodeCli> for tosho_musq::WeeklyCode {
    fn from(value: WeeklyCodeCli) -> Self {
        match value {
            WeeklyCodeCli::Sunday => tosho_musq::WeeklyCode::Sunday,
            WeeklyCodeCli::Monday => tosho_musq::WeeklyCode::Monday,
            WeeklyCodeCli::Tuesday => tosho_musq::WeeklyCode::Tuesday,
            WeeklyCodeCli::Wednesday => tosho_musq::WeeklyCode::Wednesday,
            WeeklyCodeCli::Thursday => tosho_musq::WeeklyCode::Thursday,
            WeeklyCodeCli::Friday => tosho_musq::WeeklyCode::Friday,
            WeeklyCodeCli::Saturday => tosho_musq::WeeklyCode::Saturday,
        }
    }
}

impl From<tosho_musq::WeeklyCode> for WeeklyCodeCli {
    fn from(value: tosho_musq::WeeklyCode) -> Self {
        match value {
            tosho_musq::WeeklyCode::Sunday => WeeklyCodeCli::Sunday,
            tosho_musq::WeeklyCode::Monday => WeeklyCodeCli::Monday,
            tosho_musq::WeeklyCode::Tuesday => WeeklyCodeCli::Tuesday,
            tosho_musq::WeeklyCode::Wednesday => WeeklyCodeCli::Wednesday,
            tosho_musq::WeeklyCode::Thursday => WeeklyCodeCli::Thursday,
            tosho_musq::WeeklyCode::Friday => WeeklyCodeCli::Friday,
            tosho_musq::WeeklyCode::Saturday => WeeklyCodeCli::Saturday,
        }
    }
}

impl WeeklyCodeCli {
    /// Get the index of the weekday
    pub fn indexed(&self) -> i32 {
        match self {
            WeeklyCodeCli::Monday => 1,
            WeeklyCodeCli::Tuesday => 2,
            WeeklyCodeCli::Wednesday => 3,
            WeeklyCodeCli::Thursday => 4,
            WeeklyCodeCli::Friday => 5,
            WeeklyCodeCli::Saturday => 6,
            WeeklyCodeCli::Sunday => 7,
        }
    }
}

/// Value parser for comma separated numbers
pub(super) fn parse_comma_number(s: &str) -> Result<CommaSeparatedNumber, String> {
    let mut numbers = Vec::new();

    for number in s.split(',') {
        let number = number.trim();
        let number = number
            .parse()
            .map_err(|_| format!("Invalid number: {}", number))?;

        numbers.push(number);
    }

    Ok(numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_comma_number() {
        let parsed = parse_comma_number("1,2,3,4,5");
        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_parse_comma_number_invalid() {
        let parsed = parse_comma_number("aaa,bbb");
        assert!(parsed.is_err());
    }
}
