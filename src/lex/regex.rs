use regex::Regex;
use std::sync::LazyLock;

pub static NUMBER_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d+(\.\d+)?").unwrap());

pub static RAW_STRING_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"^"[^"]*""#).unwrap());

pub static UNTERMINATED_STRING_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^"[^"]*"#).unwrap());

pub static WORD_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[_a-zA-Z]+").unwrap());

pub static COMMENT_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new("^//[^\n]*\n?").unwrap());

pub static WHITESPACE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\s+").unwrap());
