/// 核心导出功能模块
///
/// 提供统一的 HTML 表格导出功能，支持多种格式
use crate::resource::UrlGuard;
use crate::validation::{ensure_extension, validate_filename};
use csv::Writer;
use rust_xlsxwriter::Workbook;
use std::io::Cursor;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{
    Blob, HtmlAnchorElement, HtmlTableCellElement, HtmlTableElement, HtmlTableRowElement, Url,
};

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

impl ExportFormat {
    /// 获取默认文件扩展名
    fn extension(&self) -> &str {
        match self {
            ExportFormat::Csv => "csv",
            ExportFormat::Xlsx => "xlsx",
        }
    }

    /// 获取 MIME 类型
    fn mime_type(&self) -> &str {
        match self {
            ExportFormat::Csv => "text/csv;charset=utf-8",
            ExportFormat::Xlsx => {
                "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
            }
        }
    }

    /// 获取默认文件名
    fn default_filename(&self) -> String {
        format!("table_export.{}", self.extension())
    }
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
        ExportFormat::Csv => export_as_csv(table_data, filename, format),
        ExportFormat::Xlsx => export_as_xlsx(table_data, filename, format),
    }
}

/// 提取表格数据的内部辅助函数
///
/// 返回二维数组 Vec<Vec<String>>
fn extract_table_data(table_id: &str) -> Result<Vec<Vec<String>>, JsValue> {
    // 安全地获取全局的 window 和 document 对象
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("无法获取 window 对象"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("无法获取 document 对象"))?;

    // 根据 table_id 获取 table 元素，并进行类型检查
    let table_element = document
        .get_element_by_id(table_id)
        .ok_or_else(|| JsValue::from_str(&format!("找不到 ID 为 '{}' 的表格元素", table_id)))?;

    let table = table_element
        .dyn_into::<HtmlTableElement>()
        .map_err(|_| JsValue::from_str(&format!("元素 '{}' 不是有效的 HTML 表格", table_id)))?;

    // 遍历 table 中的每一行
    let rows = table.rows();
    let row_count = rows.length();

    if row_count == 0 {
        return Err(JsValue::from_str("表格为空，没有数据可导出"));
    }

    let mut table_data = Vec::new();

    for i in 0..row_count {
        let row = rows
            .get_with_index(i)
            .ok_or_else(|| JsValue::from_str(&format!("无法获取第 {} 行数据", i + 1)))?;

        let row = row
            .dyn_into::<HtmlTableRowElement>()
            .map_err(|_| JsValue::from_str(&format!("第 {} 行不是有效的表格行", i + 1)))?;

        let mut row_data = Vec::new();
        let cells = row.cells();
        let cell_count = cells.length();

        for j in 0..cell_count {
            let cell = cells.get_with_index(j).ok_or_else(|| {
                JsValue::from_str(&format!("无法获取第 {} 行第 {} 列单元格", i + 1, j + 1))
            })?;

            let cell = cell.dyn_into::<HtmlTableCellElement>().map_err(|_| {
                JsValue::from_str(&format!(
                    "第 {} 行第 {} 列不是有效的表格单元格",
                    i + 1,
                    j + 1
                ))
            })?;

            let cell_text = cell.inner_text();
            row_data.push(cell_text);
        }

        table_data.push(row_data);
    }

    Ok(table_data)
}

/// 导出为 CSV 格式
fn export_as_csv(
    table_data: Vec<Vec<String>>,
    filename: Option<String>,
    format: ExportFormat,
) -> Result<(), JsValue> {
    // 创建一个 CSV 写入器
    let mut wtr = Writer::from_writer(Cursor::new(Vec::new()));

    // 写入所有数据
    for row_data in table_data {
        wtr.write_record(&row_data)
            .map_err(|e| JsValue::from_str(&format!("写入 CSV 数据失败: {}", e)))?;
    }

    // 安全地完成 CSV 写入
    wtr.flush()
        .map_err(|e| JsValue::from_str(&format!("完成 CSV 写入失败: {}", e)))?;

    // 获取 CSV 数据
    let csv_data = wtr
        .into_inner()
        .map_err(|e| JsValue::from_str(&format!("获取 CSV 数据失败: {}", e)))?;

    if csv_data.get_ref().is_empty() {
        return Err(JsValue::from_str("没有可导出的数据"));
    }

    // 创建并下载文件
    create_and_download(csv_data.get_ref(), filename, format)
}

/// 导出为 Excel XLSX 格式
fn export_as_xlsx(
    table_data: Vec<Vec<String>>,
    filename: Option<String>,
    format: ExportFormat,
) -> Result<(), JsValue> {
    // 创建工作簿与工作表
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // 写入所有数据
    for (i, row_data) in table_data.iter().enumerate() {
        for (j, cell_text) in row_data.iter().enumerate() {
            worksheet
                .write_string(i as u32, j as u16, cell_text)
                .map_err(|e| JsValue::from_str(&format!("写入 Excel 单元格失败: {}", e)))?;
        }
    }

    // 将工作簿写入内存缓冲区
    let xlsx_bytes = workbook
        .save_to_buffer()
        .map_err(|e| JsValue::from_str(&format!("生成 Excel 文件失败: {}", e)))?;

    if xlsx_bytes.is_empty() {
        return Err(JsValue::from_str("没有可导出的数据"));
    }

    // 创建并下载文件
    create_and_download(&xlsx_bytes, filename, format)
}

/// 创建 Blob 并触发下载
fn create_and_download(
    data: &[u8],
    filename: Option<String>,
    format: ExportFormat,
) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("无法获取 window 对象"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("无法获取 document 对象"))?;

    // 安全地创建 Blob 对象
    let blob_property_bag = web_sys::BlobPropertyBag::new();
    blob_property_bag.set_type(format.mime_type());

    let array = js_sys::Array::of1(&js_sys::Uint8Array::from(data));
    let blob = Blob::new_with_u8_array_sequence_and_options(&array, &blob_property_bag)
        .map_err(|e| JsValue::from_str(&format!("创建 Blob 对象失败: {:?}", e)))?;

    // 安全地创建下载链接
    let url = Url::create_object_url_with_blob(&blob)
        .map_err(|e| JsValue::from_str(&format!("创建下载链接失败: {:?}", e)))?;

    // 使用 RAII 模式确保 URL 资源释放
    let _url_guard = UrlGuard::new(&url);

    // 设置文件名（默认根据格式）
    let final_filename = filename.unwrap_or_else(|| format.default_filename());

    // 验证文件名安全性
    if let Err(e) = validate_filename(&final_filename) {
        return Err(JsValue::from_str(&format!("文件名验证失败: {}", e)));
    }

    let final_filename = ensure_extension(&final_filename, format.extension());

    let anchor = document
        .create_element("a")
        .map_err(|e| JsValue::from_str(&format!("创建下载链接元素失败: {:?}", e)))?;
    let anchor = anchor
        .dyn_into::<HtmlAnchorElement>()
        .map_err(|_| JsValue::from_str("创建的元素不是有效的锚点元素"))?;

    anchor.set_href(&url);
    anchor.set_download(&final_filename);
    anchor.click();

    Ok(())
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
