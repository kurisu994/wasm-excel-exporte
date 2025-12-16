mod batch_export;
mod core;
mod resource;
mod utils;
mod validation;

// 使用 `wee_alloc` 作为全局分配器以减小 WASM 文件大小
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// 重新导出所有公共 API
pub use validation::{ensure_extension, validate_filename};

// 导出新的统一接口
pub use core::{ExportFormat, export_table};

// 向后兼容：导出已弃用的函数
#[allow(deprecated)]
pub use core::{
    export_table_to_csv, export_table_to_csv_with_progress, export_table_to_excel,
    export_table_to_xlsx,
};

pub use batch_export::export_table_to_csv_batch;

// 导出 utils 模块的公共函数
pub use utils::set_panic_hook;
