use std::process::ExitCode;

use codecrafters_interpreter::lox_evaluate;

macro_rules! evaluate_test {
    ($src:expr, exit_code = $exit_code:expr, stdout = $stdout:expr, stderr = $stderr:expr) => {
        let mut ok_buf = Vec::new();
        let mut err_buf = Vec::new();

        let exit_code = lox_evaluate($src, &mut ok_buf, &mut err_buf);

        assert_eq!(
            exit_code,
            ExitCode::from($exit_code),
            "\nEXITCODE: expected {}, but got {:?}",
            $exit_code,
            exit_code
        );

        let got = String::from_utf8(ok_buf).unwrap();
        assert_eq!(
            got.trim(),
            $stdout.trim(),
            "\nSTDOUT: expected {}, but got {}",
            $stdout,
            got
        );

        let got = String::from_utf8(err_buf).unwrap();
        assert_eq!(
            got.trim(),
            $stderr.trim(),
            "\nSTDERR: expected {}, but got {}",
            $stderr,
            got
        );
    };
    ($src:expr, stdout = $stdout:expr) => {
        evaluate_test! {
            $src,
            exit_code = 0,
            stdout = $stdout,
            stderr = ""
        };
    };
}

/// Literals: Booleans & Nil
/// https://app.codecrafters.io/courses/interpreter/stages/iz6
#[test]
fn iz6() {
    // #IZ6 test-1
    evaluate_test!("true", stdout = "true");

    // #IZ6 test-2
    evaluate_test!("false", stdout = "false");

    // #IZ6 test-3
    evaluate_test!("nil", stdout = "nil");
}

/// Literals: Strings & Numbers
/// https://app.codecrafters.io/courses/interpreter/stages/lv1
#[test]
fn lv1() {
    // #LV1 test-1
    evaluate_test!("20", stdout = "20");

    // #LV2 test-2
    evaluate_test!("73.47", stdout = "73.47");

    // #LV3 test-3
    evaluate_test!(r#""quz baz""#, stdout = "quz baz");

    // #LV4 test-4
    evaluate_test!("19", stdout = "19");
}

/// Parentheses
/// https://app.codecrafters.io/courses/interpreter/stages/oq9
#[test]
fn oq9() {
    // #OQ9 test-1
    evaluate_test!("(true)", stdout = "true");

    // #OQ9 test-2
    evaluate_test!("(25)", stdout = "25");

    // #OQ9 test-3
    evaluate_test!(r#"("hello foo")"#, stdout = "hello foo");

    // #OQ9 test-4
    evaluate_test!("((false))", stdout = "false");
}

/// Unary Operators: Negation & Not
/// https://app.codecrafters.io/courses/interpreter/stages/dc1
#[test]
fn dc1() {
    // #DC1 test-1
    evaluate_test!("-42", stdout = "-42");

    // #DC1 test-2
    evaluate_test!("!true", stdout = "false");

    // #DC1 test-3
    evaluate_test!("!nil", stdout = "true");

    // #DC1 test-4
    evaluate_test!("(!!25)", stdout = "true");
}

/// Arithmetic Operators (1/2)
/// https://app.codecrafters.io/courses/interpreter/stages/bp3
#[test]
fn bp3() {
    // #BP3 test-1
    evaluate_test!("16 * 58", stdout = "928");

    // #BP3 test-2
    evaluate_test!("40 / 5", stdout = "8");

    // #BP3 test-3
    evaluate_test!("7 * 3 / 7 / 1", stdout = "3");

    // #BP3 test-4
    evaluate_test!("(18 * 3 / (3 * 6))", stdout = "3");
}
