use codecrafters_interpreter::lox_tokenize;
use std::process::ExitCode;

macro_rules! tokenize_test {
    ($src:expr, exit_code = $exit_code:expr, stdout = $stdout:expr, stderr = $stderr:expr) => {
        let mut ok_buf = Vec::new();
        let mut err_buf = Vec::new();

        let exit_code = lox_tokenize($src, &mut ok_buf, &mut err_buf);

        assert_eq!(exit_code, ExitCode::from($exit_code));
        assert_eq!(String::from_utf8(ok_buf).unwrap().trim(), $stdout.trim());
        assert_eq!(String::from_utf8(err_buf).unwrap().trim(), $stderr.trim());
    };
    ($src:expr, stdout = $stdout:expr) => {
        tokenize_test! {
            $src,
            exit_code = 0,
            stdout = $stdout,
            stderr = ""
        };
    };
}

/// Scanning: Empty file
/// https://app.codecrafters.io/courses/interpreter/stages/ry8
#[test]
fn ry8() {
    // #RY8 test-1
    tokenize_test! {
        "",
        stdout = "EOF  null"
    };
}

/// Scanning: Parentheses
/// https://app.codecrafters.io/courses/interpreter/stages/ol4
#[test]
fn ol4() {
    // #OL4 test-1
    tokenize_test! {
        "(",
        stdout =
"LEFT_PAREN ( null
EOF  null"
    };

    // #OL4 test-2
    tokenize_test! {
        "))",
        stdout =
"RIGHT_PAREN ) null
RIGHT_PAREN ) null
EOF  null"
    };

    // #OL4 test-3
    tokenize_test! {
        "()())",
        stdout =
"LEFT_PAREN ( null
RIGHT_PAREN ) null
LEFT_PAREN ( null
RIGHT_PAREN ) null
RIGHT_PAREN ) null
EOF  null"
    };

    // #OL4 test-4
    tokenize_test!(
        "())((()",
        stdout = "LEFT_PAREN ( null
RIGHT_PAREN ) null
RIGHT_PAREN ) null
LEFT_PAREN ( null
LEFT_PAREN ( null
LEFT_PAREN ( null
RIGHT_PAREN ) null
EOF  null"
    );
}

/// Scanning: Braces
/// https://app.codecrafters.io/courses/interpreter/stages/oe8
#[test]
fn oe8() {
    // #OE8 test-1
    tokenize_test!(
        "}",
        stdout = "RIGHT_BRACE } null
EOF  null"
    );

    // #OE8 test-2
    tokenize_test!(
        "{{}}",
        stdout = "LEFT_BRACE { null
LEFT_BRACE { null
RIGHT_BRACE } null
RIGHT_BRACE } null
EOF  null"
    );

    // #OE8 test-3
    tokenize_test!(
        "{{}{}",
        stdout = "LEFT_BRACE { null
LEFT_BRACE { null
RIGHT_BRACE } null
LEFT_BRACE { null
RIGHT_BRACE } null
EOF  null"
    );

    // #OE8 test-4
    tokenize_test!(
        "{)}{()}",
        stdout = "LEFT_BRACE { null
RIGHT_PAREN ) null
RIGHT_BRACE } null
LEFT_BRACE { null
LEFT_PAREN ( null
RIGHT_PAREN ) null
RIGHT_BRACE } null
EOF  null"
    );
}

/// Scanning: Other single-character tokens
/// https://app.codecrafters.io/courses/interpreter/stages/xc5
#[test]
fn xc5() {
    // #XC5 test-1
    tokenize_test!(
        "+-",
        stdout = "PLUS + null
MINUS - null
EOF  null"
    );

    // #XC5 test-2
    tokenize_test!(
        "++--**..,,;;",
        stdout = "PLUS + null
PLUS + null
MINUS - null
MINUS - null
STAR * null
STAR * null
DOT . null
DOT . null
COMMA , null
COMMA , null
SEMICOLON ; null
SEMICOLON ; null
EOF  null"
    );

    // #XC5 test-3
    tokenize_test!(
        ".+,-;*.",
        stdout = "DOT . null
PLUS + null
COMMA , null
MINUS - null
SEMICOLON ; null
STAR * null
DOT . null
EOF  null"
    );

    // #XC5 test-4
    tokenize_test!(
        "({+;-.,})",
        stdout = "LEFT_PAREN ( null
LEFT_BRACE { null
PLUS + null
SEMICOLON ; null
MINUS - null
DOT . null
COMMA , null
RIGHT_BRACE } null
RIGHT_PAREN ) null
EOF  null"
    );
}

/// Scanning: Lexical errors
/// https://app.codecrafters.io/courses/interpreter/stages/ea6
#[test]
fn ea6() {
    // #EA6 test-1
    tokenize_test!(
        "@",
        exit_code = 65,
        stdout = "EOF  null",
        stderr = "[line 1] Error: Unexpected character: @"
    );

    // #EA6 test-2
    tokenize_test!(
        ",.$(#",
        exit_code = 65,
        stdout = "COMMA , null
DOT . null
LEFT_PAREN ( null
EOF  null",
        stderr = "[line 1] Error: Unexpected character: $
[line 1] Error: Unexpected character: #"
    );

    // #EA6 test-3
    tokenize_test!(
        "#$$%#",
        exit_code = 65,
        stdout = "EOF  null",
        stderr = "[line 1] Error: Unexpected character: #
[line 1] Error: Unexpected character: $
[line 1] Error: Unexpected character: $
[line 1] Error: Unexpected character: %
[line 1] Error: Unexpected character: #"
    );

    // #EA6 test-4
    tokenize_test!(
        "{(+@-#;.*)}",
        exit_code = 65,
        stdout = "LEFT_BRACE { null
LEFT_PAREN ( null
PLUS + null
MINUS - null
SEMICOLON ; null
DOT . null
STAR * null
RIGHT_PAREN ) null
RIGHT_BRACE } null
EOF  null",
        stderr = "[line 1] Error: Unexpected character: @
[line 1] Error: Unexpected character: #"
    );
}

/// Scanning: Assignment & equality Operators
/// https://app.codecrafters.io/courses/interpreter/stages/mp7
#[test]
fn mp7() {
    // #MP7 test-1
    tokenize_test!(
        "=",
        stdout = "EQUAL = null
EOF  null"
    );

    // #MP7 test-2
    tokenize_test!(
        "==",
        stdout = "EQUAL_EQUAL == null
EOF  null"
    );

    // #MP7 test-3
    tokenize_test!(
        "({=}){==}",
        stdout = "LEFT_PAREN ( null
LEFT_BRACE { null
EQUAL = null
RIGHT_BRACE } null
RIGHT_PAREN ) null
LEFT_BRACE { null
EQUAL_EQUAL == null
RIGHT_BRACE } null
EOF  null"
    );

    // #MP7 test-4
    tokenize_test!(
        "(($#%===))",
        exit_code = 65,
        stdout = "LEFT_PAREN ( null
LEFT_PAREN ( null
EQUAL_EQUAL == null
EQUAL = null
RIGHT_PAREN ) null
RIGHT_PAREN ) null
EOF  null",
        stderr = "[line 1] Error: Unexpected character: $
[line 1] Error: Unexpected character: #
[line 1] Error: Unexpected character: %"
    );
}

/// Scanning: Negation & inequality operators
/// https://app.codecrafters.io/courses/interpreter/stages/bu3
#[test]
fn bu3() {
    // #BU3 test-1
    tokenize_test!(
        "!=",
        stdout = "BANG_EQUAL != null
EOF  null"
    );

    // #BU3 test-2
    tokenize_test!(
        "!!===",
        stdout = "BANG ! null
BANG_EQUAL != null
EQUAL_EQUAL == null
EOF  null"
    );

    // #BU3 test-3
    tokenize_test!(
        "!{!}(!===)=",
        stdout = "BANG ! null
LEFT_BRACE { null
BANG ! null
RIGHT_BRACE } null
LEFT_PAREN ( null
BANG_EQUAL != null
EQUAL_EQUAL == null
RIGHT_PAREN ) null
EQUAL = null
EOF  null"
    );

    // #BU3 test-4
    tokenize_test!(
        "{(===$@%)}",
        exit_code = 65,
        stdout = "LEFT_BRACE { null
LEFT_PAREN ( null
EQUAL_EQUAL == null
EQUAL = null
RIGHT_PAREN ) null
RIGHT_BRACE } null
EOF  null",
        stderr = "[line 1] Error: Unexpected character: $
[line 1] Error: Unexpected character: @
[line 1] Error: Unexpected character: %"
    );
}
