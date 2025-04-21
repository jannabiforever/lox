use std::process::ExitCode;

use codecrafters_interpreter::lox_run;

#[test]
fn test_recursive() {
    let source = include_str!("./test_suites/recursive.lox");
    let (mut ok_buf, mut err_buf) = (Vec::new(), Vec::new());
    let exit_code = lox_run(source, &mut ok_buf, &mut err_buf);

    assert_eq!(exit_code, ExitCode::SUCCESS);
    assert!(err_buf.is_empty());

    let ok_buf = String::from_utf8(ok_buf).unwrap();
    assert_eq!(&ok_buf, "55");
}
