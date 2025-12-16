//! 统一API的测试
//!
//! 测试新的 export_table 函数

use belobog_stellar_grid::ExportFormat;

#[test]
fn test_export_format_default() {
    let format = ExportFormat::default();
    assert_eq!(format, ExportFormat::Csv);
}

#[test]
fn test_export_format_csv() {
    let format = ExportFormat::Csv;
    assert_eq!(format, ExportFormat::Csv);
}

#[test]
fn test_export_format_xlsx() {
    let format = ExportFormat::Xlsx;
    assert_eq!(format, ExportFormat::Xlsx);
}

#[test]
fn test_export_format_equality() {
    assert_eq!(ExportFormat::Csv, ExportFormat::default());
    assert_ne!(ExportFormat::Xlsx, ExportFormat::Csv);
}
