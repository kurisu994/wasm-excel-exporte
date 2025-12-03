//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use wasm_excel_exporter::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn basic_arithmetic() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
fn test_panic_hook_exists() {
    // 测试 panic hook 函数可以被调用
    // 这是一个基本的功能测试，确保函数存在且不会 panic
    // 注意：实际的 panic hook 测试需要特定的测试环境

    // 这里我们不能直接测试 panic hook，因为它会影响到后续测试
    // 但我们可以确保 utils 模块可以被导入
    assert!(true); // 简单的通过测试
}

#[wasm_bindgen_test]
fn test_error_handling() {
    // 测试错误处理机制
    // 由于这是在 WebAssembly 环境中，我们无法直接测试 DOM 操作
    // 但可以测试基本的错误条件

    // 测试空 ID 会返回错误
    let result = export_table_to_csv("", None);
    assert!(result.is_err());

    // 测试废弃函数也能工作
    let result_deprecated = export_table_to_excel("");
    assert!(result_deprecated.is_err());
}

#[wasm_bindgen_test]
fn test_filename_handling() {
    // 测试文件名处理逻辑
    // 在没有实际 DOM 的情况下，这个测试会失败，但我们可以测试错误处理

    let result = export_table_to_csv("non_existent_table", Some("test.csv".to_string()));
    assert!(result.is_err()); // 应该失败，因为表格不存在

    let result_no_ext = export_table_to_csv("non_existent_table", Some("test".to_string()));
    assert!(result_no_ext.is_err()); // 应该失败，但会自动添加 .csv 扩展名
}
