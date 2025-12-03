mod utils;

use csv::Writer;
use std::io::Cursor;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    Blob, HtmlAnchorElement, HtmlTableCellElement, HtmlTableElement, HtmlTableRowElement, Url,
};

#[wasm_bindgen]
pub fn export_table_to_excel(table_id: &str) -> Result<(), JsValue> {
    // 获取全局的 window 和 document 对象
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // 根据 table_id 获取 table 元素
    let table = document.get_element_by_id(table_id).unwrap();
    let table = table.dyn_into::<HtmlTableElement>()?;

    // 创建一个 CSV 写入器
    let mut wtr = Writer::from_writer(Cursor::new(Vec::new()));

    // 遍历 table 中的每一行
    let rows = table.rows();
    for i in 0..rows.length() {
        let row = rows.get_with_index(i).unwrap();
        let row = row.dyn_into::<HtmlTableRowElement>()?;

        let mut row_data = Vec::new();
        // 遍历每一行中的每一个单元格
        let cells = row.cells();
        for j in 0..cells.length() {
            let cell = cells.get_with_index(j).unwrap();
            let cell = cell.dyn_into::<HtmlTableCellElement>()?;
            row_data.push(cell.inner_text());
        }

        // 将行数据写入 CSV
        wtr.write_record(&row_data).unwrap();
    }

    // 完成 CSV 写入
    wtr.flush().unwrap();

    // 获取 CSV 数据
    let csv_data = wtr.into_inner().unwrap();

    // 创建 Blob 对象
    let blob_property_bag = web_sys::BlobPropertyBag::new();
    blob_property_bag.set_type("text/csv;charset=utf-8");
    let blob = Blob::new_with_u8_array_sequence_and_options(
        &js_sys::Array::of1(&js_sys::Uint8Array::from(&csv_data.get_ref()[..])),
        &blob_property_bag,
    )?;

    // 创建下载链接
    let url = Url::create_object_url_with_blob(&blob)?;
    let anchor = document.create_element("a")?;
    let anchor = anchor.dyn_into::<HtmlAnchorElement>()?;
    anchor.set_href(&url);
    anchor.set_download("table_export.csv");

    // 模拟点击下载链接
    anchor.click();

    // 释放 URL 对象
    Url::revoke_object_url(&url)?;

    Ok(())
}
