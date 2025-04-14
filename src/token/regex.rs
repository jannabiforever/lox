use std::sync::LazyLock;

use regex::Regex;

pub(super) static NUMBER_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\d+(\.\d+)?").unwrap());

pub(super) static RAW_STRING_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^"[^"]*""#).unwrap());

pub(super) static UNTERMINATED_STRING_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^"[^"]*"#).unwrap());

pub(super) static WORD_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[_a-zA-Z][_a-zA-Z0-9]*").unwrap());

pub(super) static COMMENT_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^//[^\n]*\n?").unwrap());

pub(super) static WHITESPACE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s+").unwrap());
