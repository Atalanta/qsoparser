use nom::character::complete::alphanumeric1;
use nom::combinator::{map, verify};
use nom::IResult;

#[derive(Debug, PartialEq, Eq)]
pub struct Callsign(String);

fn contains_both_letter_and_digit(input: &str) -> bool {
    let has_digit = input.chars().any(|c| c.is_digit(10));
    let has_alpha = input.chars().any(|c| c.is_alphabetic());
    has_digit && has_alpha
}
pub fn callsign_parser(input: &str) -> IResult<&str, Callsign> {
    map(
        verify(alphanumeric1, contains_both_letter_and_digit),
        |parsed_callsign: &str| Callsign(parsed_callsign.to_string()),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_english_full_callsign_correctly() {
        let notes = "g4emm";
        let result = callsign_parser(notes);
        assert_eq!(result, Ok(("", Callsign("g4emm".to_string()))));
    }

    #[test]
    fn parses_english_intermediate_callsign_correctly() {
        let notes = "2e0ihm";
        let result = callsign_parser(notes);
        assert_eq!(result, Ok(("", Callsign("2e0ihm".to_string()))));
    }

    #[test]
    fn parses_english_foundation_callsign_correctly() {
        let notes = "m7dqd";
        let result = callsign_parser(notes);
        assert_eq!(result, Ok(("", Callsign("m7dqd".to_string()))));
    }

    #[test]
    fn parses_british_regional_callsign_correctly() {
        let notes = "gm8ofx";
        let result = callsign_parser(notes);
        assert_eq!(result, Ok(("", Callsign("gm8ofx".to_string()))));
    }

    #[test]
    fn parses_portable_and_other_suffixes() {
        let notes = "g0qwe/p";
        let result = callsign_parser(notes);
        assert_eq!(result, Ok(("", Callsign("g0qwe/p".to_string()))));
    }

    #[test]
    fn parses_travelling_prefixes() {
        let notes = "g/f6tyu";
        let result = callsign_parser(notes);
        assert_eq!(result, Ok(("", Callsign("g/f6tyu".to_string()))));
    }

    #[test]
    fn no_numbers_is_invalid() {
        let notes = "notacall";
        let result = callsign_parser(notes);
        assert!(result.is_err());
    }

    #[test]
    fn no_letters_is_invalid() {
        let notes = "12345";
        let result = callsign_parser(notes);
        assert!(result.is_err());
    }
}
