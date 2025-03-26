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

#[test]
fn empty() {
    // #RY8 test-1
    tokenize_test! {
        "",
        stdout = "EOF  null"
    };
}

#[test]
fn parentheses() {
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
