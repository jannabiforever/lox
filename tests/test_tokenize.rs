use codecrafters_interpreter::lox_tokenize;
use std::process::ExitCode;

macro_rules! tokenize_test {
    ($src:expr, exit_code = $exit_code:expr, stdout = $stdout:expr, stderr = $stderr:expr) => {
        let mut ok_buf = Vec::new();
        let mut err_buf = Vec::new();

        let exit_code = lox_tokenize($src, &mut ok_buf, &mut err_buf);

        assert_eq!(exit_code, ExitCode::from($exit_code));
        assert_eq!(String::from_utf8(ok_buf).unwrap(), $stdout);
        assert_eq!(String::from_utf8(err_buf).unwrap(), $stderr);
    };
}

#[test]
fn empty() {
    // #RY8 test-1
    tokenize_test! {
        "",
        exit_code = 0,
        stdout = "EOF  null\n",
        stderr = ""
    };
}

#[test]
fn parentheses() {
    // #OL4 test-1
    tokenize_test! {
        "(",
        exit_code = 0,
        stdout = "LEFT_PAREN ( null\nEOF  null\n",
        stderr = ""
    };
}
