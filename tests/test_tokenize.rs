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

/// Scanning: Relational operators
/// https://app.codecrafters.io/courses/interpreter/stages/et2
#[test]
fn et2() {
    // #ET2 test-1
    tokenize_test!(
        ">=",
        stdout = "GREATER_EQUAL >= null
EOF  null"
    );

    // #ET2 test-2
    tokenize_test!(
        "<<<=>>>=",
        stdout = "LESS < null
LESS < null
LESS_EQUAL <= null
GREATER > null
GREATER > null
GREATER_EQUAL >= null
EOF  null"
    );

    // #ET2 test-3
    tokenize_test!(
        "><>=<<=",
        stdout = "GREATER > null
LESS < null
GREATER_EQUAL >= null
LESS < null
LESS_EQUAL <= null
EOF  null"
    );

    // #ET2 test-4
    tokenize_test!(
        "(){!>===}",
        stdout = "LEFT_PAREN ( null
RIGHT_PAREN ) null
LEFT_BRACE { null
BANG ! null
GREATER_EQUAL >= null
EQUAL_EQUAL == null
RIGHT_BRACE } null
EOF  null"
    );
}

/// Scanning: Division operator & comments
/// https://app.codecrafters.io/courses/interpreter/stages/ml2
#[test]
fn ml2() {
    // #ML2 test-1
    tokenize_test!("//Comment", stdout = "EOF  null");

    // #ML2 test-2
    tokenize_test!(
        "(///Unicode:£§᯽☺♣)",
        stdout = "LEFT_PAREN ( null
EOF  null"
    );

    // #ML2 test-3
    tokenize_test!(
        "/",
        stdout = "SLASH / null
EOF  null"
    );

    // #ML2 test-4
    tokenize_test!(
        "({(>==*)})//Comment",
        stdout = "LEFT_PAREN ( null
LEFT_BRACE { null
LEFT_PAREN ( null
GREATER_EQUAL >= null
EQUAL = null
STAR * null
RIGHT_PAREN ) null
RIGHT_BRACE } null
RIGHT_PAREN ) null
EOF  null"
    );
}

/// Scanning: Whitespace
/// https://app.codecrafters.io/courses/interpreter/stages/er2
#[test]
fn er2() {
    // #ER2 test-1
    tokenize_test!(" ", stdout = "EOF  null");

    // #ER2 test-2
    tokenize_test!("\t", stdout = "EOF  null");

    // #ER2 test-3
    tokenize_test!(
        "{ }
(( \t.,))",
        stdout = "LEFT_BRACE { null
RIGHT_BRACE } null
LEFT_PAREN ( null
LEFT_PAREN ( null
DOT . null
COMMA , null
RIGHT_PAREN ) null
RIGHT_PAREN ) null
EOF  null"
    );

    // #ER2 test-4
    tokenize_test!(
        "{\t
  }
((+><.>=))",
        stdout = "LEFT_BRACE { null
RIGHT_BRACE } null
LEFT_PAREN ( null
LEFT_PAREN ( null
PLUS + null
GREATER > null
LESS < null
DOT . null
GREATER_EQUAL >= null
RIGHT_PAREN ) null
RIGHT_PAREN ) null
EOF  null"
    );
}

/// Scanning: Multi-line errors
/// https://app.codecrafters.io/courses/interpreter/stages/tz7
#[test]
fn tz7() {
    // #TZ7 test-1
    tokenize_test!(
        "() \n@",
        exit_code = 65,
        stdout = "LEFT_PAREN ( null
RIGHT_PAREN ) null
EOF  null",
        stderr = "[line 2] Error: Unexpected character: @"
    );

    // #TZ7 test-2
    tokenize_test!(
        "@ $%",
        exit_code = 65,
        stdout = "EOF  null",
        stderr = "[line 1] Error: Unexpected character: @
[line 1] Error: Unexpected character: $
[line 1] Error: Unexpected character: %"
    );

    // #TZ7 test-3
    tokenize_test!(
        "()  #\t{}
@
$
+++
// Let's Go!
+++
#",
        exit_code = 65,
        stdout = "LEFT_PAREN ( null
RIGHT_PAREN ) null
LEFT_BRACE { null
RIGHT_BRACE } null
PLUS + null
PLUS + null
PLUS + null
PLUS + null
PLUS + null
PLUS + null
EOF  null",
        stderr = "[line 1] Error: Unexpected character: #
[line 2] Error: Unexpected character: @
[line 3] Error: Unexpected character: $
[line 7] Error: Unexpected character: #"
    );

    // #TZ7 test-4
    tokenize_test!(
        "({; @})",
        exit_code = 65,
        stdout = "LEFT_PAREN ( null
LEFT_BRACE { null
SEMICOLON ; null
RIGHT_BRACE } null
RIGHT_PAREN ) null
EOF  null",
        stderr = "[line 1] Error: Unexpected character: @"
    );
}

/// Scanning: String literals
/// https://app.codecrafters.io/courses/interpreter/stages/ue7
#[test]
fn ue7() {
    // #UE7 test-1
    tokenize_test!(
        r#""hello""#,
        stdout = r#"STRING "hello" hello
EOF  null"#
    );

    // #UE7 test-2
    tokenize_test!(
        r#""foo" "unterminated"#,
        exit_code = 65,
        stdout = r#"STRING "foo" foo
EOF  null"#,
        stderr = "[line 1] Error: Unterminated string."
    );

    // #UE7 test-3
    tokenize_test!(
        r#""foo     bar 123 // hello world!""#,
        stdout = r#"STRING "foo     bar 123 // hello world!" foo     bar 123 // hello world!
EOF  null"#
    );

    // #UE7 test-4
    tokenize_test!(
        r#"("world"+"hello") != "other_string""#,
        stdout = r#"LEFT_PAREN ( null
STRING "world" world
PLUS + null
STRING "hello" hello
RIGHT_PAREN ) null
BANG_EQUAL != null
STRING "other_string" other_string
EOF  null"#
    );
}

/// Scanning: Number literals
/// https://app.codecrafters.io/courses/interpreter/stages/kj0
#[test]
fn kj0() {
    // #KJ0 test-1
    tokenize_test!(
        "22",
        stdout = "NUMBER 22 22.0
EOF  null"
    );

    // #KJ0 test-2
    tokenize_test!(
        "9030.5924",
        stdout = "NUMBER 9030.5924 9030.5924
EOF  null"
    );

    // #KJ0 test-3
    tokenize_test!(
        "15.0000",
        stdout = "NUMBER 15.0000 15.0
EOF  null"
    );

    // #KJ0 test-4
    tokenize_test!(
        r#"(65+54) > 29 != ("Success" != "Failure") != (76 >= 41)"#,
        stdout = r#"LEFT_PAREN ( null
NUMBER 65 65.0
PLUS + null
NUMBER 54 54.0
RIGHT_PAREN ) null
GREATER > null
NUMBER 29 29.0
BANG_EQUAL != null
LEFT_PAREN ( null
STRING "Success" Success
BANG_EQUAL != null
STRING "Failure" Failure
RIGHT_PAREN ) null
BANG_EQUAL != null
LEFT_PAREN ( null
NUMBER 76 76.0
GREATER_EQUAL >= null
NUMBER 41 41.0
RIGHT_PAREN ) null
EOF  null"#
    );
}

/// Scanning: Identifiers
/// https://app.codecrafters.io/courses/interpreter/stages/ey7
#[test]
fn ey7() {
    // #EY7 test-1
    tokenize_test!(
        "bar baz",
        stdout = "IDENTIFIER bar null
IDENTIFIER baz null
EOF  null"
    );

    // #EY7 test-2
    tokenize_test!(
        "_1236az f00 foo bar baz",
        stdout = "IDENTIFIER _1236az null
IDENTIFIER f00 null
IDENTIFIER foo null
IDENTIFIER bar null
IDENTIFIER baz null
EOF  null"
    );

    // #EY7 test-3
    tokenize_test!(
        r#"message = "Hello, World!"
number = 123"#,
        stdout = r#"IDENTIFIER message null
EQUAL = null
STRING "Hello, World!" Hello, World!
IDENTIFIER number null
EQUAL = null
NUMBER 123 123.0
EOF  null"#
    );

    // #EY7 test-4
    tokenize_test!(
        r#"{
// This is a complex test case
str1 = "Test"
str2 = "Case"
num1 = 100
num2 = 200.00
result = (str1 == str2) != ((num1 + num2) >= 300)
}"#,
        stdout = r#"LEFT_BRACE { null
IDENTIFIER str1 null
EQUAL = null
STRING "Test" Test
IDENTIFIER str2 null
EQUAL = null
STRING "Case" Case
IDENTIFIER num1 null
EQUAL = null
NUMBER 100 100.0
IDENTIFIER num2 null
EQUAL = null
NUMBER 200.00 200.0
IDENTIFIER result null
EQUAL = null
LEFT_PAREN ( null
IDENTIFIER str1 null
EQUAL_EQUAL == null
IDENTIFIER str2 null
RIGHT_PAREN ) null
BANG_EQUAL != null
LEFT_PAREN ( null
LEFT_PAREN ( null
IDENTIFIER num1 null
PLUS + null
IDENTIFIER num2 null
RIGHT_PAREN ) null
GREATER_EQUAL >= null
NUMBER 300 300.0
RIGHT_PAREN ) null
RIGHT_BRACE } null
EOF  null"#
    );
}

/// Scanning: Reserved words
/// https://app.codecrafters.io/courses/interpreter/stages/pq5
#[test]
fn pq5() {
    // #PQ5 test-1
    tokenize_test!(
        "return",
        stdout = "RETURN return null
EOF  null"
    );

    // #PQ5 test-2
    tokenize_test!(
        "FALSE AND FUN TRUE SUPER print PRINT for ELSE nil else VAR RETURN false if fun IF NIL while and true FOR return or CLASS OR THIS super this class var WHILE",
        stdout = "IDENTIFIER FALSE null
IDENTIFIER AND null
IDENTIFIER FUN null
IDENTIFIER TRUE null
IDENTIFIER SUPER null
PRINT print null
IDENTIFIER PRINT null
FOR for null
IDENTIFIER ELSE null
NIL nil null
ELSE else null
IDENTIFIER VAR null
IDENTIFIER RETURN null
FALSE false null
IF if null
FUN fun null
IDENTIFIER IF null
IDENTIFIER NIL null
WHILE while null
AND and null
TRUE true null
IDENTIFIER FOR null
RETURN return null
OR or null
IDENTIFIER CLASS null
IDENTIFIER OR null
IDENTIFIER THIS null
SUPER super null
THIS this null
CLASS class null
VAR var null
IDENTIFIER WHILE null
EOF  null"
    );

    // #PQ5 test-3
    tokenize_test!(
        r#"var greeting = "Hello"
if (greeting == "Hello") {
    return true
} else {
    return false
}"#,
        stdout = r#"VAR var null
IDENTIFIER greeting null
EQUAL = null
STRING "Hello" Hello
IF if null
LEFT_PAREN ( null
IDENTIFIER greeting null
EQUAL_EQUAL == null
STRING "Hello" Hello
RIGHT_PAREN ) null
LEFT_BRACE { null
RETURN return null
TRUE true null
RIGHT_BRACE } null
ELSE else null
LEFT_BRACE { null
RETURN return null
FALSE false null
RIGHT_BRACE } null
EOF  null"#
    );

    // #PQ5 test-4
    tokenize_test!(
        r#"var result = (a + b) > 7 or "Success" != "Failure" or x >= 5
while (result) {
    var counter = 0
    counter = counter + 1
    if (counter == 10) {
        return nil
    }
}"#,
        stdout = r#"VAR var null
IDENTIFIER result null
EQUAL = null
LEFT_PAREN ( null
IDENTIFIER a null
PLUS + null
IDENTIFIER b null
RIGHT_PAREN ) null
GREATER > null
NUMBER 7 7.0
OR or null
STRING "Success" Success
BANG_EQUAL != null
STRING "Failure" Failure
OR or null
IDENTIFIER x null
GREATER_EQUAL >= null
NUMBER 5 5.0
WHILE while null
LEFT_PAREN ( null
IDENTIFIER result null
RIGHT_PAREN ) null
LEFT_BRACE { null
VAR var null
IDENTIFIER counter null
EQUAL = null
NUMBER 0 0.0
IDENTIFIER counter null
EQUAL = null
IDENTIFIER counter null
PLUS + null
NUMBER 1 1.0
IF if null
LEFT_PAREN ( null
IDENTIFIER counter null
EQUAL_EQUAL == null
NUMBER 10 10.0
RIGHT_PAREN ) null
LEFT_BRACE { null
RETURN return null
NIL nil null
RIGHT_BRACE } null
RIGHT_BRACE } null
EOF  null"#
    );
}
