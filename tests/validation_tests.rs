//! 验证模块的单元测试

use wasm_excel_exporter::{validate_filename, ensure_extension};

#[test]
fn test_valid_filenames() {
    assert!(validate_filename("normal_file.csv").is_ok());
    assert!(validate_filename("file-with-dashes.csv").is_ok());
    assert!(validate_filename("file_with_underscores.csv").is_ok());
    assert!(validate_filename("文件名.csv").is_ok());
    assert!(validate_filename("file.csv").is_ok());
}

#[test]
fn test_empty_filename() {
    assert_eq!(
        validate_filename(""),
        Err("文件名不能为空".to_string())
    );
}

#[test]
fn test_path_separators() {
    assert_eq!(
        validate_filename("path/file.csv"),
        Err("文件名不能包含路径分隔符".to_string())
    );
    assert_eq!(
        validate_filename("path\\file.csv"),
        Err("文件名不能包含路径分隔符".to_string())
    );
}

#[test]
fn test_dangerous_characters() {
    assert!(validate_filename("file<.csv").is_err());
    assert!(validate_filename("file>.csv").is_err());
    assert!(validate_filename("file:.csv").is_err());
    assert!(validate_filename("file\".csv").is_err());
    assert!(validate_filename("file|.csv").is_err());
    assert!(validate_filename("file?.csv").is_err());
    assert!(validate_filename("file*.csv").is_err());
}

#[test]
fn test_reserved_names() {
    assert!(validate_filename("CON.csv").is_err());
    assert!(validate_filename("PRN.csv").is_err());
    assert!(validate_filename("AUX.csv").is_err());
    assert!(validate_filename("NUL.csv").is_err());
    assert!(validate_filename("COM1.csv").is_err());
    assert!(validate_filename("LPT1.csv").is_err());
    assert!(validate_filename("con.csv").is_err()); // 小写也应该被捕获
}

#[test]
fn test_leading_trailing_dots_spaces() {
    assert!(validate_filename(".file.csv").is_err());
    assert!(validate_filename("file.csv.").is_err());
    assert!(validate_filename(" file.csv").is_err());
    assert!(validate_filename("file.csv ").is_err());
}

#[test]
fn test_long_filename() {
    let long_name = "a".repeat(256);
    assert!(validate_filename(&long_name).is_err());
}

#[test]
fn test_ensure_extension() {
    assert_eq!(ensure_extension("file", "csv"), "file.csv");
    assert_eq!(ensure_extension("file.txt", "csv"), "file.txt.csv");
    assert_eq!(ensure_extension("file.CSV", "csv"), "file.CSV");
    assert_eq!(ensure_extension("file.csv", "CSV"), "file.csv");
}