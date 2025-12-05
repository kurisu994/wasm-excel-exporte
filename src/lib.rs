mod utils;
mod validation;
mod resource;
mod core;
mod batch_export;

// 使用 `wee_alloc` 作为全局分配器以减小 WASM 文件大小
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// 重新导出所有公共 API
pub use validation::{validate_filename, ensure_extension};

pub use core::{
    export_table_to_csv,
    export_table_to_csv_with_progress,
    export_table_to_excel,
};

pub use batch_export::export_table_to_csv_batch;

// 导出 utils 模块的公共函数（如果有的话）
pub use utils::set_panic_hook;