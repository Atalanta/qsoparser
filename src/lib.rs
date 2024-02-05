use std::str::FromStr;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
struct Callsign(String);

#[derive(Debug, PartialEq, Eq)]
struct ParseCallsignError;

impl FromStr for Callsign {
    type Err = ParseCallsignError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = Regex::new(r"(?i)([GM][0-8]|2E[01])[A-Z]{2,3}").unwrap();
        if pattern.is_match(s) {
            Ok(Callsign(s.to_uppercase()))
        } else {
            Err(ParseCallsignError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_english_callsign_correctly() {
        let input = "g4emm";
        let result = input.parse::<Callsign>();
        assert_eq!(result, Ok(Callsign("G4EMM".to_string())));
    }

    #[test]
    fn passes_over_anything_that_isnt_callsign() {
        let result = "notacall".parse::<Callsign>();
        assert!(result.is_err());
    }
}
