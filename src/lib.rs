use nom::character::complete::alphanumeric1;
use nom::combinator::{map, verify};
use nom::IResult;

#[derive(Debug, PartialEq, Eq)]
pub struct Callsign(String);

fn contains_both_letter_and_digit(input: &str) -> bool {
    let has_digit = input.chars().any(|c| c.is_ascii_digit());
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
    fn simple_callsigns_are_correctly_parsed() {
        let simple_callsigns = vec![
            "g4emm",
            "2e0ihm",
            "m7dqd",
            "gm8ofx",
        ];

        for input in simple_callsigns {
            let result = callsign_parser(input);
            assert!(result.is_ok(), "Failed to parse valid callsign: {}", input);
            let (remaining, callsign) = result.expect("Expected Ok, got Err");
            assert!(
                remaining.is_empty(),
                "Parser did not consume the entire input for: {}",
                input
            );
            assert_eq!(
                callsign.0, input,
                "Parsed callsign does not match input for: {}",
                input
            );
        }
    }

    #[test]
    fn suffix_callsigns_are_correctly_parsed() {
        let suffix_callsigns = vec![
            "g4emm/p",
            "2e0ihm/m",
            "m7dqd/a",
            "gm8ofx/qrp",
            "lz1xn/mm",
            "JG3TYS/3",
        ];

        for input in suffix_callsigns {
            let result = callsign_parser(input);
            assert!(result.is_ok(), "Failed to parse valid callsign: {}", input);
            let (remaining, callsign) = result.expect("Expected Ok, got Err");
            assert!(
                remaining.is_empty(),
                "Parser did not consume the entire input for: {}",
                input
            );
            assert_eq!(
                callsign.0, input,
                "Parsed callsign does not match input for: {}",
                input
            );
        }
    }
    #[test]
    fn invalid_callsigns_are_correctly_identified() {
        let invalid_callsigns = vec![
            "notacall",
            "12345",
        ];

        for input in invalid_callsigns {
            let result = callsign_parser(input);
            assert!(
                result.is_err(),
                "Incorrectly parsed invalid callsign as valid: {}",
                input
            );
        }
    }
}
