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

/// Arithmetic operators (1/2)
/// https://app.codecrafters.io/courses/interpreter/stages/wa9
#[test]
fn wa9() {
    // #WA9 test-1
    parse_test!("29 * 48 / 44", stdout = "(/ (* 29.0 48.0) 44.0)");

    // #WA9 test-2
    parse_test!("48 / 18 / 56", stdout = "(/ (/ 48.0 18.0) 56.0)");

    // #WA9 test-3
    parse_test!(
        "58 * 12 * 60 / 21",
        stdout = "(/ (* (* 58.0 12.0) 60.0) 21.0)"
    );

    parse_test!(
        "(87 * -26 / (27 * 15))",
        stdout = "(group (/ (* 87.0 (- 26.0)) (group (* 27.0 15.0))))"
    );
}

/// Arithmetic operators (2/2)
/// https://app.codecrafters.io/courses/interpreter/stages/yf2
#[test]
fn yf2() {
    // #YF2 test-1
    parse_test!(r#""hello" + "world""#, stdout = "(+ hello world)");

    // #YF2 test-2
    parse_test!(
        "17 - 15 * 13 - 16",
        stdout = "(- (- 17.0 (* 15.0 13.0)) 16.0)"
    );

    // #YF2 test-3
    parse_test!(
        "47 + 80 - 45 / 19",
        stdout = "(- (+ 47.0 80.0) (/ 45.0 19.0))"
    );

    // #YF2 test-4
    parse_test!(
        "(-14 + 66) * (70 * 83) / (66 + 38)",
        stdout = "(/ (* (group (+ (- 14.0) 66.0)) (group (* 70.0 83.0))) (group (+ 66.0 38.0)))"
    );
}

/// Comparison operators
/// https://app.codecrafters.io/courses/interpreter/stages/uh4
#[test]
fn uh4() {
    // #UH4 test-1
    parse_test!("68 > 38", stdout = "(> 68.0 38.0)");
    // #UH4 test-2
    parse_test!("30 <= 98", stdout = "(<= 30.0 98.0)");
    // #UH4 test-3
    parse_test!("68 < 98 < 128", stdout = "(< (< 68.0 98.0) 128.0)");
    // #UH4 test-4
    parse_test!(
        "(16 - 16) >= -(29 / 64 + 19)",
        stdout = "(>= (group (- 16.0 16.0)) (- (group (+ (/ 29.0 64.0) 19.0))))"
    );
}

/// Equality operators
/// https://app.codecrafters.io/courses/interpreter/stages/ht8
#[test]
fn ht8() {
    // #HT8 test-1
    parse_test!(r#""world" != "hello""#, stdout = "(!= world hello)");

    // #HT8 test-2
    parse_test!(r#""hello" == "hello""#, stdout = "(== hello hello)");

    // #HT8 test-3
    parse_test!("70 == 76", stdout = "(== 70.0 76.0)");

    // #HT8 test-4
    parse_test!(
        "(89 != 88) == ((-39 + 60) >= (87 * 47))",
        stdout = "(== (group (!= 89.0 88.0)) (group (>= (group (+ (- 39.0) 60.0)) (group (* 87.0 47.0)))))"
    );
}

/// Syntactic errors
/// https://app.codecrafters.io/courses/interpreter/stages/wz8
#[test]
fn wz8() {
    // #WZ8 test-1
    parse_test!(
        r#""bar"#,
        exit_code = 65,
        stdout = "",
        stderr = "[line 1] Error: Unterminated string."
    );

    // #WZ8 test-2
    parse_test!(
        "(foo",
        exit_code = 65,
        stdout = "",
        stderr = "[line 1] Error at end: Expected ')'."
    );

    // #WZ8 test-3
    parse_test!(
        "(67 +)",
        exit_code = 65,
        stdout = "",
        stderr = "[line 1] Error at ')': Expected expression."
    );

    // #WZ8 test-4
    parse_test!(
        "+",
        exit_code = 65,
        stdout = "",
        stderr = "[line 1] Error at '+': Expected expression."
    );
}
