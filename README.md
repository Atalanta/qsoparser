# QSO Parser

A library of parsers designed to take a single line of notes taken during an amateur radio QSO, and parsing them
into a struct of useful fields suitable for logging, exporting as ADIF, or subsequent searching.

## Current Parsers

### Callsign

Will match (most) current English callsigns.  This is a naive regex, for proof of concept, and is not intended to be
comprehensive.

## Planned Parsers

- RST - will match CW and Phone signal reports, including the < or > annotation to specify if report set or received
- Frequency - will match frequency in kHz
- QTH - will match location details, of one or more words
- OP - will match name of operator
- WX - will match weather details
- Rig - will match details of receiver / transmitter / transceiver user
- Ant - will match details of antenna used
- Pwr - will match power transmitted
- Key - will match details of key used
- WAB - will match WAB square
- Loc - will match locator square
- Serial - will match serial for contest purposes

## Usage

This is not ready to be used, and is not designed to be imported.  This is a work in progress, driven by unit tests.

## Contributing

While this project is currently in its early stages and not ready for external contributions, future collaboration will
be welcomed. Stay tuned for guidelines on how to contribute.
