use regex::Regex;
use std::sync::LazyLock;

pub(super) static PLAIN_NUMBER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^-?(?:\d+|\d*\.\d+)$").unwrap());

// Digit groups of exactly three, so "47,658" is cleaned but the French decimal "1,5" is not.
pub(super) static GROUPED_COMMA: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(-?\d{1,3}(?:,\d{3})+)(\.\d+)?$").unwrap());

pub(super) const SPACE_SEPARATORS: [char; 3] = [' ', '\u{a0}', '\u{202f}'];

// Space-grouped (French, Nordic) with an optional decimal comma, plus the Swiss apostrophe form.
pub(super) static GROUPED_SPACE: LazyLock<Regex> = LazyLock::new(|| {
    let class: String = SPACE_SEPARATORS.iter().collect();
    Regex::new(&format!(r"^(-?\d{{1,3}}(?:[{class}]\d{{3}})+)(?:,(\d+))?$")).unwrap()
});

pub(super) static GROUPED_QUOTE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(-?\d{1,3}(?:'\d{3})+)(\.\d+)?$").unwrap());
