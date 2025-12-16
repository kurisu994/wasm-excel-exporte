# 核心导出模块架构

本目录包含了 excel-exporter 的核心导出功能，采用模块化设计，每个文件负责单一职责。

## 📁 模块结构

```
src/core/
├── mod.rs                # 核心协调模块（统一导出接口）
├── export_csv.rs         # CSV 格式导出模块
├── export_xlsx.rs        # Excel XLSX 格式导出模块
└── table_extractor.rs    # 表格数据提取模块
```

## 🔧 模块职责

### mod.rs - 核心协调模块
- **职责**：提供统一的导出 API，协调各个子模块
- **主要函数**：
  - `export_table()` - 统一导出函数（新 API）
  - `ExportFormat` - 导出格式枚举
- **向后兼容**：
  - `export_table_to_csv()` - 已弃用
  - `export_table_to_xlsx()` - 已弃用
  - `export_table_to_excel()` - 已弃用
  - `export_table_to_csv_with_progress()` - 已弃用
- **大小**：4.8KB

### export_csv.rs - CSV 导出模块
- **职责**：处理 CSV 格式的导出逻辑
- **主要函数**：
  - `export_as_csv()` - CSV 导出核心逻辑
  - `create_and_download_csv()` - 创建 CSV Blob 并下载
- **依赖**：
  - `csv` crate - CSV 序列化
  - `validation` - 文件名验证
  - `resource` - RAII 资源管理
- **大小**：3.2KB

### export_xlsx.rs - Excel 导出模块
- **职责**：处理 Excel XLSX 格式的导出逻辑
- **主要函数**：
  - `export_as_xlsx()` - Excel 导出核心逻辑
  - `create_and_download_xlsx()` - 创建 Excel Blob 并下载
- **依赖**：
  - `rust_xlsxwriter` crate - Excel 文件生成
  - `validation` - 文件名验证
  - `resource` - RAII 资源管理
- **大小**：3.4KB

### table_extractor.rs - 表格提取模块
- **职责**：从 DOM 中提取 HTML 表格数据
- **主要函数**：
  - `extract_table_data()` - 提取表格为二维数组
- **依赖**：
  - `web_sys` - DOM API 绑定
- **特点**：
  - 详细的错误消息（包含行号、列号）
  - 类型安全的 DOM 操作
  - 零拷贝数据提取
- **大小**：2.6KB

## 🔄 数据流

```
用户调用 export_table()
    ↓
[mod.rs] 验证输入 & 确定格式
    ↓
[table_extractor.rs] 从 DOM 提取表格数据
    ↓
Vec<Vec<String>> (二维数组)
    ↓
根据格式分发：
    ├─→ [export_csv.rs] 转换为 CSV → Blob → 下载
    └─→ [export_xlsx.rs] 转换为 Excel → Blob → 下载
```

## ✅ 设计原则

1. **单一职责**：每个模块只负责一种导出格式或一项功能
2. **高内聚低耦合**：模块间通过明确的接口交互
3. **易于扩展**：添加新格式只需新增一个模块
4. **便于维护**：每个文件独立，修改互不影响

## 🚀 添加新导出格式

要添加新的导出格式（如 JSON、TSV 等），只需：

1. 在 `src/core/` 创建新文件，如 `export_json.rs`
2. 实现 `export_as_json()` 函数
3. 在 `mod.rs` 中：
   - 添加 `mod export_json;`
   - 在 `ExportFormat` 枚举中添加新变体
   - 在 `export_table()` 的 match 中添加分支

示例：
```rust
// src/core/export_json.rs
pub fn export_as_json(
    table_data: Vec<Vec<String>>, 
    filename: Option<String>
) -> Result<(), JsValue> {
    // 实现 JSON 导出逻辑
}

// src/core/mod.rs
mod export_json;
use export_json::export_as_json;

#[wasm_bindgen]
pub enum ExportFormat {
    Csv,
    Xlsx,
    Json,  // 新增
}

match format {
    ExportFormat::Csv => export_as_csv(table_data, filename),
    ExportFormat::Xlsx => export_as_xlsx(table_data, filename),
    ExportFormat::Json => export_as_json(table_data, filename),  // 新增
}
```

## 📊 模块大小对比

| 模块                  | 大小  | 行数 | 职责               |
| --------------------- | ----- | ---- | ------------------ |
| mod.rs                | 4.8KB | ~150 | 协调和向后兼容     |
| export_csv.rs         | 3.2KB | ~100 | CSV 导出           |
| export_xlsx.rs        | 3.4KB | ~105 | Excel 导出         |
| table_extractor.rs    | 2.6KB | ~80  | DOM 数据提取       |
| **总计**              | 14KB  | ~435 | 核心导出功能       |

相比重构前的单文件 `core.rs`（11KB），模块化后代码量增加了约 3KB，但：
- ✅ 逻辑更清晰
- ✅ 维护更容易
- ✅ 扩展更简单
- ✅ 测试更独立

## 🧪 测试策略

每个模块都应有对应的单元测试：
- `export_csv.rs` → CSV 序列化测试
- `export_xlsx.rs` → Excel 生成测试
- `table_extractor.rs` → DOM 提取测试
- `mod.rs` → 集成测试

## 📝 维护指南

### 修改 CSV 导出逻辑
- 编辑 `export_csv.rs`
- 无需修改其他模块

### 修改 Excel 导出逻辑
- 编辑 `export_xlsx.rs`
- 无需修改其他模块

### 修改表格提取逻辑
- 编辑 `table_extractor.rs`
- 所有导出格式自动受益

### 添加新 API
- 在 `mod.rs` 中添加
- 使用现有子模块的函数

## 🔒 安全性

所有模块都遵循项目的安全原则：
- ✅ RAII 资源管理（UrlGuard）
- ✅ 文件名验证（validate_filename）
- ✅ 中文错误消息
- ✅ 零 panic 设计
- ✅ 类型安全的 DOM 操作
