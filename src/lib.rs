use nom::character::complete::alphanumeric1;
use nom::IResult;
use std::str::FromStr;
use nom::combinator::verify;

#[derive(Debug, PartialEq, Eq)]
struct Callsign(String);

#[derive(Debug, PartialEq, Eq)]
struct ParseCallsignError;

fn contains_both_letter_and_digit(input: &str) -> bool {
    let has_digit = input.chars().any(|c| c.is_digit(10));
    let has_alpha = input.chars().any(|c| c.is_alphabetic());
    has_digit && has_alpha
}
fn callsign_parser(input: &str) -> IResult<&str, &str> {
    verify(alphanumeric1, contains_both_letter_and_digit)(input)
}

impl FromStr for Callsign {
    type Err = ParseCallsignError;

    fn from_str(notes: &str) -> Result<Self, Self::Err> {
        match callsign_parser(notes) {
            Ok((_, callsign)) => Ok(Callsign(callsign.to_string())),
            Err(_) => Err(ParseCallsignError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_english_full_callsign_correctly() {
        let notes = "g4emm";
        let result = notes.parse::<Callsign>();
        assert_eq!(result, Ok(Callsign("g4emm".to_string())));
    }

    #[test]
    fn parses_english_intermediate_callsign_correctly() {
        let notes = "2e0ihm";
        let result = notes.parse::<Callsign>();
        assert_eq!(result, Ok(Callsign("2e0ihm".to_string())));
    }

    #[test]
    fn parses_english_foundation_callsign_correctly() {
        let notes = "m7dqd";
        let result = notes.parse::<Callsign>();
        assert_eq!(result, Ok(Callsign("m7dqd".to_string())));
    }

    #[test]
    fn parses_portable_and_other_suffixes() {
        let notes = "g0qwe/p";
        let result = notes.parse::<Callsign>();
        assert_eq!(result, Ok(Callsign("g0qwe/p".to_string())));
    }

    #[test]
    fn no_numbers_is_invalid() {
        let result = "notacall".parse::<Callsign>();
        assert!(result.is_err());
    }

    #[test]
    fn no_letters_is_invalid() {
        let result = "12345".parse::<Callsign>();
        assert!(result.is_err());
    }
}
