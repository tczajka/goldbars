use crate::lexer::compute_line_starts;

#[test]
fn test_line_starts() {
    let test_data: &[(&str, &[usize])] = &[
        // Trivial.
        ("", &[]),
        // Regular.
        ("abc\n e\nghij\n", &[0, 4, 7]),
        // Empty lines.
        ("\n\n\n", &[0, 1, 2]),
        // Unfinished last line.
        ("abc\ndef", &[0, 4]),
        // CR LF endings.
        ("abc\r\ndef\r\n", &[0, 5]),
        // CR endings.
        ("abc\rdef\r", &[0, 4]),
        // Unicode.
        ("żółw\nż", &[0, 8]),
        // BOM
        ("\u{FEFF}abc\n", &[3]),
        // Second FEFF isn't BOM.
        ("\u{FEFF}\u{FEFF}abc\n", &[3]),
        // All kinds of endings.
        (
            "abc\nabc\rabc\r\nabc\u{B}abc\u{C}abc\u{85}abc\u{2028}abc\u{2029}abc",
            &[0, 4, 8, 13, 17, 21, 26, 32, 38],
        ),
    ];

    for &(text, expected_line_starts) in test_data {
        let line_starts = compute_line_starts(text);
        assert_eq!(line_starts, expected_line_starts);
    }
}
