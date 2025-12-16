use wasm_bindgen::JsCast;
/// 表格数据提取模块
///
/// 提供从 DOM 中提取表格数据的功能
use wasm_bindgen::prelude::*;
use web_sys::{HtmlTableCellElement, HtmlTableElement, HtmlTableRowElement};

/// 从 HTML 表格中提取数据
///
/// # 参数
/// * `table_id` - HTML 表格元素的 ID
///
/// # 返回值
/// * `Ok(Vec<Vec<String>>)` - 二维字符串数组，表示表格数据
/// * `Err(JsValue)` - 提取失败，包含错误信息
pub fn extract_table_data(table_id: &str) -> Result<Vec<Vec<String>>, JsValue> {
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
