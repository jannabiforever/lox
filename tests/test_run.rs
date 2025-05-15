use std::process::ExitCode;

use codecrafters_interpreter::lox_run;

macro_rules! test_source_run {
    ($file_name: literal, $expected: literal) => {
        let source = include_str!(concat!("./test_suites/", $file_name, ".lox"));
        let (mut ok_buf, mut err_buf) = (Vec::new(), Vec::new());
        let result = lox_run(source, &mut ok_buf, &mut err_buf);

        let (ok_buf, err_buf) = (
            String::from_utf8(ok_buf).unwrap(),
            String::from_utf8(err_buf).unwrap(),
        );

        assert_eq!(result, ExitCode::SUCCESS);
        assert_eq!(ok_buf.trim(), $expected);
        assert!(err_buf.trim().is_empty());
    };
}

#[test]
fn recursive() {
    test_source_run!("recursive", "55");
}

#[test]
fn nested_func() {
    test_source_run!("nested_func", "8");
}

#[test]
fn block_scope_resolution() {
    test_source_run!("block_scope_resolution", "global\nglobal");
}
