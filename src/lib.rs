use nom::character::complete::{char, satisfy};
use nom::multi::many_m_n;
use nom::{combinator::recognize, sequence::tuple, IResult};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct Callsign(String);

#[derive(Debug, PartialEq, Eq)]
struct ParseCallsignError;

fn callsign_parser(notes: &str) -> IResult<&str, &str> {
    recognize(tuple((
        char('g'),
        satisfy(|c: char| c.is_digit(10) && c != '9'),
        many_m_n(2, 3, satisfy(|c: char| c.is_alphabetic())),
    )))(notes)
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
    fn parses_english_callsign_correctly() {
        let notes = "g4emm";
        let result = notes.parse::<Callsign>();
        assert_eq!(result, Ok(Callsign("g4emm".to_string())));
    }

    #[test]
    fn passes_over_anything_that_isnt_callsign() {
        let result = "notacall".parse::<Callsign>();
        assert!(result.is_err());
    }
}
