
use regex::Regex;

#[derive(PartialEq, Debug, Default)]
pub struct ParseResult {
    callsign: Option<String>,
}

impl ParseResult {
    pub fn new() -> Self {
        ParseResult::default()
    }

    pub fn parse_line(mut self, line: &str) -> Self {
        let pattern = Regex::new(r"(?i)([GM][0-8]|2E[01])[A-Z]{2,3}").unwrap();
        if let Some(match_) = pattern.find(line) {
            self.callsign = Some(match_.as_str().to_uppercase());
        }
        self
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_english_callsign_correctly() {
        let result = ParseResult::new().parse_line("g4emm");
        assert_eq!(result, ParseResult {callsign: Some("G4EMM".to_string())});
    }

    #[test]
    fn passes_over_anything_that_isnt_callsign() {
        let result = ParseResult::new().parse_line("blank");
        assert_eq!(result, ParseResult {callsign: None});
    }
}
