use rust_performance::zero_copy::{parse_borrowed, parse_owned, ParseError};

#[test]
fn borrowed_and_owned_parsers_preserve_the_same_pairs() {
    let borrowed = parse_borrowed("lang=rust;mode=release").expect("valid input");
    let owned = parse_owned("lang=rust;mode=release").expect("valid input");

    assert_eq!(borrowed, [("lang", "rust"), ("mode", "release")]);
    assert_eq!(
        owned,
        [
            (String::from("lang"), String::from("rust")),
            (String::from("mode"), String::from("release"))
        ]
    );
}

#[test]
fn parser_rejects_segments_without_an_equals_sign() {
    assert_eq!(
        parse_borrowed("lang=rust;invalid"),
        Err(ParseError::MissingSeparator)
    );
}
