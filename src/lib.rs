use nom::character::complete::{alphanumeric1, char};
use nom::combinator::{map, opt, verify};
use nom::sequence::{preceded, tuple};
use nom::IResult;

#[derive(Debug, PartialEq, Eq)]
pub struct Callsign {
    callsign: String,
    suffix: Option<String>,
    prefix: Option<String>,
}

fn contains_both_letter_and_digit(input: &str) -> bool {
    let has_digit = input.chars().any(|c| c.is_ascii_digit());
    let has_alpha = input.chars().any(|c| c.is_alphabetic());
    has_digit && has_alpha
}

pub fn callsign_parser(input: &str) -> IResult<&str, Callsign> {
    let parse_callsign = verify(alphanumeric1, contains_both_letter_and_digit);
    let parse_suffix = opt(preceded(char('/'), alphanumeric1));

    map(
        tuple((parse_callsign, parse_suffix)),
        |(callsign, suffix): (&str, Option<&str>)| Callsign {
            callsign: callsign.to_string(),
            suffix: suffix.map(String::from),
            prefix: None,
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_callsigns_are_correctly_parsed() {
        let simple_callsigns = vec!["g4emm", "2e0ihm", "m7dqd", "gm8ofx"];

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
                callsign.callsign, input,
                "Parsed callsign does not match input for: {}",
                input
            );
        }
    }

    #[test]
    fn suffix_callsigns_are_correctly_parsed() {
        let test_cases = vec![
            ("g4emm/p", "g4emm", Some("p")),
            ("2e0ihm/m", "2e0ihm", Some("m")),
            ("m7dqd/a", "m7dqd", Some("a")),
            ("gm8ofx/qrp", "gm8ofx", Some("qrp")),
            ("lz1xn/mm", "lz1xn", Some("mm")),
            ("JG3TYS/3", "JG3TYS", Some("3")),
            ("VE7SDF/K7", "VE7SDF", Some("K7")),
        ];

        for (input, expected_call, expected_suffix) in test_cases {
            let result = callsign_parser(input);
            assert!(
                result.is_ok(),
                "Failed to parse valid callsign with suffix: {}",
                input
            );
            let (remaining, callsign) = result.expect("Expected Ok, got Err");

            assert!(
                remaining.is_empty(),
                "Parser did not consume the entire input for: {}",
                input
            );
            assert_eq!(
                callsign.callsign, expected_call,
                "Parsed callsign part does not match input for: {}",
                input
            );
            assert_eq!(
                callsign.suffix,
                expected_suffix.map(String::from),
                "Parsed suffix does not match input for: {}",
                input
            );
        }
    }

    #[test]
    fn prefix_callsigns_are_correctly_parsed() {
        let test_cases = vec![
            ("f/m0boz", "m0boz", Some("f")),
            ("oz8/hb9lag", "hb9lag", Some("oz8")),
        ];

        for (input, expected_call, expected_prefix) in test_cases {
            let result = callsign_parser(input);
            assert!(
                result.is_ok(),
                "Failed to parse valid callsign with suffix: {}",
                input
            );
            let (remaining, callsign) = result.expect("Expected Ok, got Err");

            assert!(
                remaining.is_empty(),
                "Parser did not consume the entire input for: {}",
                input
            );
            assert_eq!(
                callsign.callsign, expected_call,
                "Parsed callsign part does not match input for: {}",
                input
            );
            assert_eq!(
                callsign.prefix,
                expected_prefix.map(String::from),
                "Parsed suffix does not match input for: {}",
                input
            );
        }
    }

    #[test]
    fn invalid_callsigns_are_correctly_identified() {
        let invalid_callsigns = vec!["notacall", "12345"];

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
