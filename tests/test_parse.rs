use codecrafters_interpreter::lox_parse;
use std::process::ExitCode;

macro_rules! parse_test {
    ($src:expr, exit_code = $exit_code:expr, stdout = $stdout:expr, stderr = $stderr:expr) => {
        let mut ok_buf = Vec::new();
        let mut err_buf = Vec::new();

        let exit_code = lox_parse($src, &mut ok_buf, &mut err_buf);

        assert_eq!(exit_code, ExitCode::from($exit_code));
        assert_eq!(String::from_utf8(ok_buf).unwrap().trim(), $stdout.trim());
        assert_eq!(String::from_utf8(err_buf).unwrap().trim(), $stderr.trim());
    };
    ($src:expr, stdout = $stdout:expr) => {
        parse_test! {
            $src,
            exit_code = 0,
            stdout = $stdout,
            stderr = ""
        };
    };
}

/// Booleans & Nil
/// https://app.codecrafters.io/courses/interpreter/stages/sc2
#[test]
fn sc2() {
    // #SC2 test-1
    parse_test!("true", stdout = "true");

    // #SC2 test-2
    parse_test!("false", stdout = "false");

    // #SC2 test-3
    parse_test!("nil", stdout = "nil");
}

/// Number literals
/// https://app.codecrafters.io/courses/interpreter/stages/ra8
#[test]
fn ra8() {
    // #RA8 test-1
    parse_test!("12", stdout = "12.0");

    // #RA8 test-2
    parse_test!("0.0", stdout = "0.0");

    // #RA8 test-3
    parse_test!("82.54", stdout = "82.54");
}

/// String literals
/// https://app.codecrafters.io/courses/interpreter/stages/th5
#[test]
fn th5() {
    // #TH5 test-1
    parse_test!(r#""foo hello""#, stdout = "foo hello");

    // #TH5 test-2
    parse_test!(r#""'baz'""#, stdout = "'baz'");

    // #TH5 test-3
    parse_test!(r#""// hello""#, stdout = "// hello");

    // #TH5 test-4
    parse_test!(r#""51""#, stdout = "51");
}

/// Parentheses
/// https://app.codecrafters.io/courses/interpreter/stages/xe6
#[test]
fn xe6() {
    // #XE6 test-1
    parse_test!(r#"("foo")"#, stdout = "(group foo)");

    // #XE6 test-2
    parse_test!("((true))", stdout = "(group (group true))");

    // #XE6 test-3
    parse_test!("(nil)", stdout = "(group nil)");

    // #XE6 test-4
    parse_test!("(77.76)", stdout = "(group 77.76)");
}

/// Unary Operators
/// https://app.codecrafters.io/courses/interpreter/stages/mq1
#[test]
fn mq1() {
    // #MQ1 test-1
    parse_test!("!false", stdout = "(! false)");

    // #MQ1 test-2
    parse_test!("-61", stdout = "(- 61.0)");

    // #MQ1 test-3
    parse_test!("!!true", stdout = "(! (! true))");

    // #MQ1 test-4
    parse_test!("(!!(true))", stdout = "(group (! (! (group true))))");
}
