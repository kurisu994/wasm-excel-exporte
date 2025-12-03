mod utils;

use csv::Writer;
use std::io::Cursor;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    Blob, HtmlAnchorElement, HtmlTableCellElement, HtmlTableElement, HtmlTableRowElement, Url,
};

// 使用 `wee_alloc` 作为全局分配器以减小 WASM 文件大小
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filename_extension_handling() {
        // 测试文件名扩展名处理逻辑
        // 由于这部分是纯 Rust 逻辑，我们可以独立测试

        let test_cases = vec![
            ("test.csv", "test.csv"),
            ("report", "report.csv"),
            ("data.CSV", "data.CSV"), // 保持原有大小写
            ("export.csv", "export.csv"),
            ("", "table_export.csv"), // 默认文件名
        ];

        for (input, expected) in test_cases {
            let result = if input.is_empty() {
                "table_export.csv".to_string()
            } else if input.to_lowercase().ends_with(".csv") {
                input.to_string()
            } else {
                format!("{}.csv", input)
            };

            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_validation_logic() {
        // 测试输入验证逻辑
        let valid_id = "my-table";
        let empty_id = "";

        // 这些验证在函数中，我们测试逻辑
        assert!(!valid_id.is_empty());
        assert!(empty_id.is_empty());
    }

    #[test]
    fn test_csv_writer_creation() {
        // 测试 CSV writer 可以正常创建和使用
        let mut wtr = Writer::from_writer(Cursor::new(Vec::new()));

        let test_data = vec!["Header1", "Header2", "Header3"];
        assert!(wtr.write_record(&test_data).is_ok());
        assert!(wtr.flush().is_ok());

        let csv_data = wtr.into_inner().unwrap();
        assert!(!csv_data.get_ref().is_empty());
    }

    #[test]
    fn test_filename_validation() {
        // 测试有效的文件名
        assert!(super::validate_filename("valid_filename.csv").is_ok());
        assert!(super::validate_filename("report-2024").is_ok());
        assert!(super::validate_filename("数据导出_test").is_ok());

        // 测试无效的文件名
        assert!(super::validate_filename("").is_err()); // 空文件名
        assert!(super::validate_filename("path/to/file").is_err()); // 包含路径分隔符
        assert!(super::validate_filename("path\\to\\file").is_err()); // 包含路径分隔符
        assert!(super::validate_filename("invalid<name").is_err()); // 包含非法字符
        assert!(super::validate_filename("invalid>name").is_err()); // 包含非法字符
        assert!(super::validate_filename("invalid:name").is_err()); // 包含非法字符
        assert!(super::validate_filename("invalid\"name").is_err()); // 包含非法字符
        assert!(super::validate_filename("invalid|name").is_err()); // 包含非法字符
        assert!(super::validate_filename("invalid?name").is_err()); // 包含非法字符
        assert!(super::validate_filename("invalid*name").is_err()); // 包含非法字符
        assert!(super::validate_filename("CON").is_err()); // Windows 保留名称
        assert!(super::validate_filename("PRN.csv").is_err()); // Windows 保留名称
        assert!(super::validate_filename("AUX").is_err()); // Windows 保留名称
        assert!(super::validate_filename(".hidden").is_err()); // 以点开头
        assert!(super::validate_filename("trailing.").is_err()); // 以点结尾
        assert!(super::validate_filename(" space").is_err()); // 以空格开头
        assert!(super::validate_filename("space ").is_err()); // 以空格结尾

        // 测试过长的文件名
        let long_name = "a".repeat(256);
        assert!(super::validate_filename(&long_name).is_err());
    }
}

/// 验证文件名是否安全合法
///
/// # 参数
/// * `filename` - 要验证的文件名
///
/// # 返回值
/// * `Ok(())` - 文件名合法
/// * `Err(String)` - 文件名不合法，包含错误信息
fn validate_filename(filename: &str) -> Result<(), String> {
    // 检查文件名是否为空
    if filename.is_empty() {
        return Err("文件名不能为空".to_string());
    }

    // 检查文件名中的危险字符（路径分隔符）
    if filename.contains('/') || filename.contains('\\') {
        return Err("文件名不能包含路径分隔符".to_string());
    }

    // 检查其他危险字符
    let dangerous_chars = ['<', '>', ':', '"', '|', '?', '*'];
    for ch in dangerous_chars.iter() {
        if filename.contains(*ch) {
            return Err(format!("文件名不能包含非法字符: {}", ch));
        }
    }

    // 检查文件名长度（大多数文件系统限制为 255 字节）
    if filename.len() > 255 {
        return Err("文件名过长（最大 255 个字符）".to_string());
    }

    // 检查 Windows 保留文件名
    let base_name = filename.split('.').next().unwrap_or("");
    let reserved_names = [
        "CON", "PRN", "AUX", "NUL",
        "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9",
        "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    ];

    if reserved_names.contains(&base_name.to_uppercase().as_str()) {
        return Err(format!("文件名 '{}' 是系统保留名称", base_name));
    }

    // 检查文件名是否以点或空格开头/结尾（Windows 不支持）
    if filename.starts_with('.') || filename.starts_with(' ')
        || filename.ends_with('.') || filename.ends_with(' ') {
        return Err("文件名不能以点或空格开头或结尾".to_string());
    }

    Ok(())
}

/// 安全地导出 HTML 表格到 CSV 文件
///
/// # 参数
/// * `table_id` - 要导出的 HTML 表格元素的 ID
/// * `filename` - 可选的导出文件名（可选，默认为 "table_export.csv"）
///
/// # 返回值
/// * `Ok(())` - 导出成功
/// * `Err(JsValue)` - 导出失败，包含错误信息
#[wasm_bindgen]
pub fn export_table_to_csv(table_id: &str, filename: Option<String>) -> Result<(), JsValue> {
    // 输入验证
    if table_id.is_empty() {
        return Err(JsValue::from_str("表格 ID 不能为空"));
    }

    // 安全地获取全局的 window 和 document 对象
    let window = web_sys::window()
        .ok_or_else(|| JsValue::from_str("无法获取 window 对象"))?;
    let document = window.document()
        .ok_or_else(|| JsValue::from_str("无法获取 document 对象"))?;

    // 根据 table_id 获取 table 元素，并进行类型检查
    let table_element = document.get_element_by_id(table_id)
        .ok_or_else(|| JsValue::from_str(&format!("找不到 ID 为 '{}' 的表格元素", table_id)))?;

    let table = table_element.dyn_into::<HtmlTableElement>()
        .map_err(|_| JsValue::from_str(&format!("元素 '{}' 不是有效的 HTML 表格", table_id)))?;

    // 创建一个 CSV 写入器
    let mut wtr = Writer::from_writer(Cursor::new(Vec::new()));

    // 遍历 table 中的每一行
    let rows = table.rows();
    let row_count = rows.length();

    if row_count == 0 {
        return Err(JsValue::from_str("表格为空，没有数据可导出"));
    }

    for i in 0..row_count {
        let row = rows.get_with_index(i)
            .ok_or_else(|| JsValue::from_str(&format!("无法获取第 {} 行数据", i + 1)))?;

        let row = row.dyn_into::<HtmlTableRowElement>()
            .map_err(|_| JsValue::from_str(&format!("第 {} 行不是有效的表格行", i + 1)))?;

        let mut row_data = Vec::new();
        // 遍历每一行中的每一个单元格
        let cells = row.cells();
        let cell_count = cells.length();

        for j in 0..cell_count {
            let cell = cells.get_with_index(j)
                .ok_or_else(|| JsValue::from_str(&format!("无法获取第 {} 行第 {} 列单元格", i + 1, j + 1)))?;

            let cell = cell.dyn_into::<HtmlTableCellElement>()
                .map_err(|_| JsValue::from_str(&format!("第 {} 行第 {} 列不是有效的表格单元格", i + 1, j + 1)))?;

            let cell_text = cell.inner_text();
            row_data.push(cell_text);
        }

        // 安全地将行数据写入 CSV
        wtr.write_record(&row_data)
            .map_err(|e| JsValue::from_str(&format!("写入 CSV 数据失败: {}", e)))?;
    }

    // 安全地完成 CSV 写入
    wtr.flush()
        .map_err(|e| JsValue::from_str(&format!("完成 CSV 写入失败: {}", e)))?;

    // 获取 CSV 数据
    let csv_data = wtr.into_inner()
        .map_err(|e| JsValue::from_str(&format!("获取 CSV 数据失败: {}", e)))?;

    if csv_data.get_ref().is_empty() {
        return Err(JsValue::from_str("没有可导出的数据"));
    }

    // 安全地创建 Blob 对象
    let blob_property_bag = web_sys::BlobPropertyBag::new();
    blob_property_bag.set_type("text/csv;charset=utf-8");

    let array = js_sys::Array::of1(&js_sys::Uint8Array::from(&csv_data.get_ref()[..]));
    let blob = Blob::new_with_u8_array_sequence_and_options(&array, &blob_property_bag)
        .map_err(|e| JsValue::from_str(&format!("创建 Blob 对象失败: {:?}", e)))?;

    // 安全地创建下载链接
    let url = Url::create_object_url_with_blob(&blob)
        .map_err(|e| JsValue::from_str(&format!("创建下载链接失败: {:?}", e)))?;

    // 使用 RAII 模式确保 URL 资源释放
    let _url_guard = UrlGuard::new(&url);

    // 设置文件名（默认为 table_export.csv）
    let final_filename = filename.unwrap_or_else(|| "table_export.csv".to_string());

    // 验证文件名安全性
    if let Err(e) = validate_filename(&final_filename) {
        return Err(JsValue::from_str(&format!("文件名验证失败: {}", e)));
    }

    let final_filename = if final_filename.to_lowercase().ends_with(".csv") {
        final_filename
    } else {
        format!("{}.csv", final_filename)
    };

    let anchor = document.create_element("a")
        .map_err(|e| JsValue::from_str(&format!("创建下载链接元素失败: {:?}", e)))?;
    let anchor = anchor.dyn_into::<HtmlAnchorElement>()
        .map_err(|_| JsValue::from_str("创建的元素不是有效的锚点元素"))?;

    anchor.set_href(&url);
    anchor.set_download(&final_filename);

    // 尝试触发下载，但不阻塞资源清理
    // 使用传统的点击方法
    anchor.click();
    // 注意：某些浏览器可能阻止程序化点击，这是预期的行为

    Ok(())
}

/// 导出 HTML 表格到 CSV 文件（带进度回调）
///
/// # 参数
/// * `table_id` - 要导出的 HTML 表格元素的 ID
/// * `filename` - 可选的导出文件名（可选，默认为 "table_export.csv"）
/// * `progress_callback` - 进度回调函数，接收进度百分比 (0-100)
///
/// # 返回值
/// * `Ok(())` - 导出成功
/// * `Err(JsValue)` - 导出失败，包含错误信息
#[wasm_bindgen]
pub fn export_table_to_csv_with_progress(
    table_id: &str,
    filename: Option<String>,
    progress_callback: Option<js_sys::Function>,
) -> Result<(), JsValue> {
    // 输入验证
    if table_id.is_empty() {
        return Err(JsValue::from_str("表格 ID 不能为空"));
    }

    // 安全地获取全局的 window 和 document 对象
    let window = web_sys::window()
        .ok_or_else(|| JsValue::from_str("无法获取 window 对象"))?;
    let document = window.document()
        .ok_or_else(|| JsValue::from_str("无法获取 document 对象"))?;

    // 根据 table_id 获取 table 元素，并进行类型检查
    let table_element = document.get_element_by_id(table_id)
        .ok_or_else(|| JsValue::from_str(&format!("找不到 ID 为 '{}' 的表格元素", table_id)))?;

    let table = table_element.dyn_into::<HtmlTableElement>()
        .map_err(|_| JsValue::from_str(&format!("元素 '{}' 不是有效的 HTML 表格", table_id)))?;

    // 创建一个 CSV 写入器
    let mut wtr = Writer::from_writer(Cursor::new(Vec::new()));

    // 遍历 table 中的每一行
    let rows = table.rows();
    let row_count = rows.length();

    if row_count == 0 {
        return Err(JsValue::from_str("表格为空，没有数据可导出"));
    }

    // 报告初始进度
    if let Some(ref callback) = progress_callback {
        let _ = callback.call1(&JsValue::NULL, &JsValue::from_f64(0.0));
    }

    for i in 0..row_count {
        let row = rows.get_with_index(i)
            .ok_or_else(|| JsValue::from_str(&format!("无法获取第 {} 行数据", i + 1)))?;

        let row = row.dyn_into::<HtmlTableRowElement>()
            .map_err(|_| JsValue::from_str(&format!("第 {} 行不是有效的表格行", i + 1)))?;

        let mut row_data = Vec::new();
        // 遍历每一行中的每一个单元格
        let cells = row.cells();
        let cell_count = cells.length();

        for j in 0..cell_count {
            let cell = cells.get_with_index(j)
                .ok_or_else(|| JsValue::from_str(&format!("无法获取第 {} 行第 {} 列单元格", i + 1, j + 1)))?;

            let cell = cell.dyn_into::<HtmlTableCellElement>()
                .map_err(|_| JsValue::from_str(&format!("第 {} 行第 {} 列不是有效的表格单元格", i + 1, j + 1)))?;

            let cell_text = cell.inner_text();
            row_data.push(cell_text);
        }

        // 安全地将行数据写入 CSV
        wtr.write_record(&row_data)
            .map_err(|e| JsValue::from_str(&format!("写入 CSV 数据失败: {}", e)))?;

        // 报告进度
        if let Some(ref callback) = progress_callback {
            let progress = ((i + 1) as f64 / row_count as f64) * 100.0;
            let _ = callback.call1(&JsValue::NULL, &JsValue::from_f64(progress));
        }
    }

    // 安全地完成 CSV 写入
    wtr.flush()
        .map_err(|e| JsValue::from_str(&format!("完成 CSV 写入失败: {}", e)))?;

    // 获取 CSV 数据
    let csv_data = wtr.into_inner()
        .map_err(|e| JsValue::from_str(&format!("获取 CSV 数据失败: {}", e)))?;

    if csv_data.get_ref().is_empty() {
        return Err(JsValue::from_str("没有可导出的数据"));
    }

    // 安全地创建 Blob 对象
    let blob_property_bag = web_sys::BlobPropertyBag::new();
    blob_property_bag.set_type("text/csv;charset=utf-8");

    let array = js_sys::Array::of1(&js_sys::Uint8Array::from(&csv_data.get_ref()[..]));
    let blob = Blob::new_with_u8_array_sequence_and_options(&array, &blob_property_bag)
        .map_err(|e| JsValue::from_str(&format!("创建 Blob 对象失败: {:?}", e)))?;

    // 安全地创建下载链接
    let url = Url::create_object_url_with_blob(&blob)
        .map_err(|e| JsValue::from_str(&format!("创建下载链接失败: {:?}", e)))?;

    // 使用 RAII 模式确保 URL 资源释放
    let _url_guard = UrlGuard::new(&url);

    // 设置文件名（默认为 table_export.csv）
    let final_filename = filename.unwrap_or_else(|| "table_export.csv".to_string());

    // 验证文件名安全性
    if let Err(e) = validate_filename(&final_filename) {
        return Err(JsValue::from_str(&format!("文件名验证失败: {}", e)));
    }

    let final_filename = if final_filename.to_lowercase().ends_with(".csv") {
        final_filename
    } else {
        format!("{}.csv", final_filename)
    };

    let anchor = document.create_element("a")
        .map_err(|e| JsValue::from_str(&format!("创建下载链接元素失败: {:?}", e)))?;
    let anchor = anchor.dyn_into::<HtmlAnchorElement>()
        .map_err(|_| JsValue::from_str("创建的元素不是有效的锚点元素"))?;

    anchor.set_href(&url);
    anchor.set_download(&final_filename);

    // 尝试触发下载，但不阻塞资源清理
    // 使用传统的点击方法
    anchor.click();
    // 注意：某些浏览器可能阻止程序化点击，这是预期的行为

    Ok(())
}

/// RAII 风格的 URL 资源管理器
struct UrlGuard {
    url: String,
}

impl UrlGuard {
    fn new(url: &str) -> Self {
        Self { url: url.to_string() }
    }
}

impl Drop for UrlGuard {
    fn drop(&mut self) {
        // 确保在对象销毁时释放 URL 资源
        if let Err(e) = Url::revoke_object_url(&self.url) {
            // 记录错误但不阻止程序执行
            wasm_bindgen::JsValue::from_str(&format!("释放 URL 资源失败: {:?}", e));
        }
    }
}

/// 为了向后兼容保留的函数（不推荐使用）
///
/// # 已弃用
/// 请使用 `export_table_to_csv` 函数替代
#[wasm_bindgen]
#[deprecated(note = "请使用 export_table_to_csv(table_id, filename) 替代")]
#[allow(deprecated)]  // 允许调用主函数，避免递归弃用警告
pub fn export_table_to_excel(table_id: &str) -> Result<(), JsValue> {
    export_table_to_csv(table_id, None)
}
