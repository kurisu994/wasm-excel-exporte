mod utils;

use csv::Writer;
use std::io::Cursor;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    Blob, HtmlAnchorElement, HtmlTableCellElement, HtmlTableElement, HtmlTableRowElement,
    HtmlTableSectionElement, Url,
};

// 使用 `wee_alloc` 作为全局分配器以减小 WASM 文件大小
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


/// 验证文件名是否安全合法
///
/// # 参数
/// * `filename` - 要验证的文件名
///
/// # 返回值
/// * `Ok(())` - 文件名合法
/// * `Err(String)` - 文件名不合法，包含错误信息
///
/// # 注意
/// 这个函数主要供内部使用，但也导出以便测试
#[doc(hidden)]
pub fn validate_filename(filename: &str) -> Result<(), String> {
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

/// 分批异步导出 HTML 表格到 CSV 文件
///
/// 这个函数将表格数据分批处理，在批次之间让出控制权给浏览器事件循环，
/// 从而避免在处理大量数据时阻塞主线程导致页面卡死。
///
/// # 参数
/// * `table_id` - 要导出的 HTML 表格元素的 ID
/// * `filename` - 可选的导出文件名（可选，默认为 "table_export.csv"）
/// * `batch_size` - 每批处理的行数（默认 1000）
/// * `progress_callback` - 进度回调函数，接收进度百分比 (0-100)
///
/// # 返回值
/// * `Promise<void>` - 异步操作的 Promise
///
/// # 示例
/// ```javascript
/// import { export_table_to_csv_batch } from './pkg/wasm_excel_exporter.js';
///
/// await export_table_to_csv_batch(
///     'my-table',
///     'data.csv',
///     1000,  // 每批 1000 行
///     (progress) => {
///         console.log(`进度: ${progress}%`);
///     }
/// );
/// ```
#[wasm_bindgen]
pub async fn export_table_to_csv_batch(
    table_id: String,
    tbody_id: Option<String>,
    filename: Option<String>,
    batch_size: Option<u32>,
    progress_callback: Option<js_sys::Function>,
) -> Result<JsValue, JsValue> {
    // 输入验证
    if table_id.is_empty() {
        return Err(JsValue::from_str("表格 ID 不能为空"));
    }

    let batch_size = batch_size.unwrap_or(1000) as usize;
    if batch_size == 0 {
        return Err(JsValue::from_str("批次大小必须大于 0"));
    }

    // 安全地获取全局的 window 和 document 对象
    let window = web_sys::window()
        .ok_or_else(|| JsValue::from_str("无法获取 window 对象"))?;
    let document = window.document()
        .ok_or_else(|| JsValue::from_str("无法获取 document 对象"))?;

    // 1. 获取主表格（通常包含表头）
    let table_element = document.get_element_by_id(&table_id)
        .ok_or_else(|| JsValue::from_str(&format!("找不到 ID 为 '{}' 的表格元素", table_id)))?;
    let table = table_element.dyn_into::<HtmlTableElement>()
        .map_err(|_| JsValue::from_str(&format!("元素 '{}' 不是有效的 HTML 表格", table_id)))?;
    let table_rows = table.rows();
    let table_row_count = table_rows.length() as usize;

    // 2. 获取数据表格体（如果有）
    let mut tbody_rows_collection = None;
    let mut tbody_row_count = 0;

    if let Some(tid) = tbody_id {
        if !tid.is_empty() {
            let tbody_element = document.get_element_by_id(&tid)
                .ok_or_else(|| JsValue::from_str(&format!("找不到 ID 为 '{}' 的 tbody 元素", tid)))?;
            
            // 尝试转换为 HtmlTableSectionElement (tbody)
            let tbody = tbody_element.dyn_into::<HtmlTableSectionElement>()
                .map_err(|_| JsValue::from_str(&format!("元素 '{}' 不是有效的 HTML 表格部分(tbody)", tid)))?;
            
            let rows = tbody.rows();
            tbody_row_count = rows.length() as usize;
            tbody_rows_collection = Some(rows);
        }
    }

    let total_rows = table_row_count + tbody_row_count;

    if total_rows == 0 {
        return Err(JsValue::from_str("表格为空，没有数据可导出"));
    }

    // 创建 CSV 写入器
    let mut wtr = Writer::from_writer(Cursor::new(Vec::new()));

    // 报告初始进度
    if let Some(ref callback) = progress_callback {
        let _ = callback.call1(&JsValue::NULL, &JsValue::from_f64(0.0));
    }

    // 分批处理数据
    let mut current_row = 0;
    while current_row < total_rows {
        let batch_end = std::cmp::min(current_row + batch_size, total_rows);

        // 处理当前批次
        for i in current_row..batch_end {
            let row_element = if i < table_row_count {
                // 从主表格读取
                table_rows.get_with_index(i as u32)
            } else {
                // 从 tbody 读取
                if let Some(ref rows) = tbody_rows_collection {
                    rows.get_with_index((i - table_row_count) as u32)
                } else {
                    None
                }
            };

            let row = row_element
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

        current_row = batch_end;

        // 报告进度
        if let Some(ref callback) = progress_callback {
            let progress = (current_row as f64 / total_rows as f64) * 100.0;
            let _ = callback.call1(&JsValue::NULL, &JsValue::from_f64(progress));
        }

        // 在批次之间让出控制权
        if current_row < total_rows {
            yield_to_browser().await?;
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

    // 触发下载
    anchor.click();

    Ok(JsValue::UNDEFINED)
}

/// 让出控制权给浏览器事件循环
///
/// 使用 setTimeout(0) 创建一个微任务，允许浏览器处理其他事件
async fn yield_to_browser() -> Result<(), JsValue> {
    let promise = js_sys::Promise::new(&mut |resolve, _reject| {
        let window = web_sys::window().expect("无法获取 window 对象");
        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 0);
    });

    JsFuture::from(promise).await?;
    Ok(())
}


