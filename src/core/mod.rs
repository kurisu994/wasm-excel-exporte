/// 核心导出协调模块
///
/// 提供统一的导出接口，协调各个导出模块
mod export_csv;
mod export_xlsx;
mod table_extractor;

use export_csv::export_as_csv;
use export_xlsx::export_as_xlsx;
use table_extractor::extract_table_data;
use wasm_bindgen::prelude::*;

/// 导出格式枚举
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExportFormat {
    /// CSV 格式（默认）
    #[default]
    Csv,
    /// Excel XLSX 格式
    Xlsx,
}

/// 统一的表格导出函数
///
/// 支持导出为 CSV 或 Excel 格式，通过 format 参数控制
///
/// # 参数
/// * `table_id` - 要导出的 HTML 表格元素的 ID
/// * `filename` - 可选的导出文件名（不包含扩展名时会自动添加）
/// * `format` - 导出格式（Csv 或 Xlsx），默认为 Csv
///
/// # 返回值
/// * `Ok(())` - 导出成功
/// * `Err(JsValue)` - 导出失败，包含错误信息
///
/// # 示例
/// ```javascript
/// import init, { export_table, ExportFormat } from './pkg/excel_exporter.js';
/// await init();
///
/// // 导出为 CSV（默认）
/// export_table('my-table');
/// export_table('my-table', '数据.csv');
/// export_table('my-table', '数据', ExportFormat.Csv);
///
/// // 导出为 Excel
/// export_table('my-table', '报表.xlsx', ExportFormat.Xlsx);
/// export_table('my-table', '报表', ExportFormat.Xlsx);
/// ```
#[wasm_bindgen]
pub fn export_table(
    table_id: &str,
    filename: Option<String>,
    format: Option<ExportFormat>,
) -> Result<(), JsValue> {
    let format = format.unwrap_or_default();

    // 输入验证
    if table_id.is_empty() {
        return Err(JsValue::from_str("表格 ID 不能为空"));
    }

    // 提取表格数据
    let table_data = extract_table_data(table_id)?;

    // 根据格式导出
    match format {
        ExportFormat::Csv => export_as_csv(table_data, filename),
        ExportFormat::Xlsx => export_as_xlsx(table_data, filename),
    }
}

// ===========================
// 向后兼容的已弃用函数
// ===========================

/// 安全地导出 HTML 表格到 CSV 文件
///
/// **已弃用**：请使用 `export_table(table_id, filename, ExportFormat::Csv)` 替代
///
/// # 参数
/// * `table_id` - 要导出的 HTML 表格元素的 ID
/// * `filename` - 可选的导出文件名（可选，默认为 "table_export.csv"）
///
/// # 返回值
/// * `Ok(())` - 导出成功
/// * `Err(JsValue)` - 导出失败，包含错误信息
#[wasm_bindgen]
#[deprecated(note = "请使用 export_table(table_id, filename, ExportFormat::Csv) 替代")]
pub fn export_table_to_csv(table_id: &str, filename: Option<String>) -> Result<(), JsValue> {
    export_table(table_id, filename, Some(ExportFormat::Csv))
}

/// 导出 HTML 表格到 CSV 文件（带进度回调）
///
/// **已弃用**：暂时保留向后兼容，功能与 export_table_to_csv 相同（进度回调将在 v2.0 统一支持）
///
/// # 参数
/// * `table_id` - 要导出的 HTML 表格元素的 ID
/// * `filename` - 可选的导出文件名（可选，默认为 "table_export.csv"）
/// * `progress_callback` - 进度回调函数（当前版本忽略此参数）
///
/// # 返回值
/// * `Ok(())` - 导出成功
/// * `Err(JsValue)` - 导出失败，包含错误信息
#[wasm_bindgen]
#[deprecated(
    note = "请使用 export_table(table_id, filename, ExportFormat::Csv)，进度回调将在 v2.0 统一支持"
)]
#[allow(unused_variables)]
pub fn export_table_to_csv_with_progress(
    table_id: &str,
    filename: Option<String>,
    progress_callback: Option<js_sys::Function>,
) -> Result<(), JsValue> {
    // 当前版本忽略进度回调，直接导出
    // TODO: v2.0 将统一支持进度回调
    export_table(table_id, filename, Some(ExportFormat::Csv))
}

/// 导出 HTML 表格到 Excel .xlsx 文件
///
/// **已弃用**：请使用 `export_table(table_id, filename, ExportFormat::Xlsx)` 替代
///
/// # 参数
/// * `table_id` - 要导出的 HTML 表格元素的 ID
/// * `filename` - 可选的导出文件名（可选，默认为 "table_export.xlsx"）
///
/// # 返回值
/// * `Ok(())` - 导出成功
/// * `Err(JsValue)` - 导出失败，包含错误信息
#[wasm_bindgen]
#[deprecated(note = "请使用 export_table(table_id, filename, ExportFormat::Xlsx) 替代")]
pub fn export_table_to_xlsx(table_id: &str, filename: Option<String>) -> Result<(), JsValue> {
    export_table(table_id, filename, Some(ExportFormat::Xlsx))
}

/// 为了向后兼容保留的函数（不推荐使用）
///
/// **已弃用**：请使用 `export_table(table_id, filename, ExportFormat::Xlsx)` 替代
#[wasm_bindgen]
#[deprecated(note = "请使用 export_table(table_id, filename, ExportFormat::Xlsx) 替代")]
#[allow(deprecated)]
pub fn export_table_to_excel(table_id: &str) -> Result<(), JsValue> {
    // 为老接口提供向后兼容，默认文件名
    export_table_to_xlsx(table_id, None)
}
