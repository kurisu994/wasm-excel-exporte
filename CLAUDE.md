# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

这是一个企业级的 Rust WebAssembly 库，用于安全高效地将 HTML 表格数据导出为 CSV 文件。项目采用 v1.1.0 版本，使用 Rust Edition 2024，具有完善的错误处理和 RAII 资源管理。

## 常用命令

### WebAssembly 构建和测试
```bash
# 构建 WebAssembly 包（默认目标为 browser）
wasm-pack build

# 构建特定目标
wasm-pack build --target bundler  # webpack/rollup
wasm-pack build --target nodejs    # Node.js
wasm-pack build --target web       # 原生 Web

# 浏览器测试
wasm-pack test --headless --firefox
wasm-pack test --headless --chrome

# 发布到 NPM
wasm-pack publish

# 构建并发布到自定义 registry
wasm-pack build --target bundler && wasm-pack publish --target bundler
```

### Rust 开发工作流
```bash
# 运行所有测试（包括单元测试和 WebAssembly 测试）
cargo test

# 只运行本地测试（不包括 WebAssembly）
cargo test --lib

# 运行特定测试
cargo test test_filename_extension_handling
cargo test test_csv_writer_operations

# 检查代码（不编译）
cargo check

# 格式化代码
cargo fmt

# 代码检查
cargo clippy -- -D warnings

# 构建优化版本
cargo build --release

# 检查 WebAssembly 代码大小
wasm-pack build --target web --release
```

## 项目架构

### 核心文件结构和职责
- **src/lib.rs**: 企业级主实现，包含完整的错误处理和 RAII 资源管理
  - `export_table_to_csv(table_id, filename)`: 主导出函数（v1.1.0+）
  - `export_table_to_excel(table_id)`: 向后兼容的已弃用函数
  - `UrlGuard`: RAII 资源管理器，确保 URL 对象正确释放
  - 完整的输入验证、类型检查和错误处理机制
  - 内置单元测试：测试文件名处理、验证逻辑、CSV writer 操作

- **src/utils.rs**: WebAssembly 调试工具模块
  - `set_panic_hook()`: 开发环境下的 panic 信息显示

- **tests/unit_tests.rs**: 综合单元测试套件
  - 文件名扩展名处理和边界情况测试
  - 字符串处理和 Unicode 支持
  - CSV writer 操作和大数据处理测试
  - 错误处理和 JsValue 转换测试
  - 集成测试和函数签名兼容性

- **tests/web_original.rs**: WebAssembly 浏览器测试
  - 在实际 WebAssembly 环境中测试函数导出
  - 错误处理机制验证
  - 文件名处理逻辑在浏览器环境中的测试

- **wasm-bindgen.toml**: WebAssembly 构建配置
  - 配置为 `cdylib` 类型，优化体积
  - Release 模式下优化级别设置为 "s"（大小优先）

- **Cargo.toml**: Rust 项目配置（Edition 2024）
  - 双许可证：MIT OR Apache-2.0
  - 依赖项定期更新，使用安全版本

### WebAssembly 架构设计模式
- **错误处理策略**: 使用 `Result<T, JsValue>` 与 JavaScript 互操作，所有潜在的 panic 点都被处理
- **资源管理**: 实现 RAII 模式，`UrlGuard` 确保 Blob URL 的生命周期管理
- **内存安全**: 通过 Rust 的所有权系统防止内存泄漏和数据竞争
- **API 设计**: 新函数支持可选的文件名参数，旧函数标记为 `#[deprecated]`

### JavaScript 集成流程
1. 通过 `web_sys::window()` 和 `document()` 安全获取全局对象
2. 使用 `document.get_element_by_id()` 定位表格元素
3. 动态类型检查：`dyn_into::<HtmlTableElement>()`
4. 遍历 DOM 树：`table.rows()` → `row.cells()` → `cell.inner_text()`
5. CSV 数据序列化：使用 `csv::Writer` 写入内存缓冲区
6. 创建下载链接：`Blob::new()` → `Url::create_object_url()` → `anchor.click()`
7. 自动资源清理：RAII 确保在函数结束时释放 URL 资源

## 核心 API 使用

### 主导出函数（v1.1.0+）
```rust
#[wasm_bindgen]
pub fn export_table_to_csv(table_id: &str, filename: Option<String>) -> Result<(), JsValue>
```
- **参数**: `table_id` - HTML 表格元素 ID, `filename` - 可选的导出文件名
- **功能**: 安全导出表格到 CSV，支持自定义文件名
- **错误处理**: 全面的输入验证和异常处理

### 向后兼容函数
```rust
#[wasm_bindgen]
#[deprecated(note = "请使用 export_table_to_csv(table_id, filename) 替代")]
pub fn export_table_to_excel(table_id: &str) -> Result<(), JsValue>
```

## WebAssembly 特定注意事项

- **目标平台**: 专为现代浏览器设计，支持 WebAssembly 的所有环境
- **内存分配**: 默认使用系统分配器，可选 `wee_alloc` 小型分配器（需要 nightly）
- **调试支持**: 开发特性 `console_error_panic_hook` 提供详细的 panic 信息
- **构建优化**: Release 模式下优先考虑代码大小（`opt-level = "s"`）

## 测试策略

### 本地单元测试
```bash
# 运行所有单元测试
cargo test --lib

# 运行特定测试类别
cargo test test_filename_extension_handling  # 文件名处理
cargo test test_csv_writer_operations        # CSV 操作
cargo test test_string_handling_edge_cases    # 字符串边界情况
cargo test test_memory_efficiency            # 内存效率测试
```

### WebAssembly 浏览器测试
```bash
# 在 Firefox 中测试
wasm-pack test --headless --firefox

# 在 Chrome 中测试
wasm-pack test --headless --chrome
```

### 测试覆盖范围
- **文件名处理**: 各种扩展名、Unicode 字符、特殊符号、边界情况
- **CSV 操作**: 数据写入、引号转义、大数据量处理、内存效率
- **错误处理**: JsValue 转换、格式化错误消息、边界条件
- **字符串处理**: 多语言字符、大小写转换、长度计算
- **集成测试**: 函数签名兼容性、返回类型处理
- **WebAssembly 环境**: 浏览器环境中的实际功能测试

## 错误处理和调试

### 开发环境调试
```javascript
// 在 JavaScript 中启用详细错误信息
import { set_panic_hook } from 'wasm-excel-exporter';
set_panic_hook();
```

### 常见错误类型
- 表格元素不存在或不是有效的 `<table>` 元素
- 表格为空（没有行数据）
- DOM 操作失败（权限问题、页面卸载等）
- Blob 创建失败或浏览器不支持下载
- WebAssembly 初始化失败

### 错误消息格式
- **中文错误消息**: 所有用户可见的错误都使用中文
- **详细信息**: 包含行号、列号等具体定位信息
- **调试信息**: 在开发模式下提供详细的上下文

## 性能和安全特性

- **零拷贝操作**: CSV 数据在内存中直接构建，避免不必要的数据复制
- **内存安全**: Rust 编译时保证，防止缓冲区溢出、使用后释放等漏洞
- **资源管理**: RAII 确保 Web 资源（如 Blob URL）的自动清理
- **输入验证**: 对所有用户输入进行严格的类型和边界检查
- **大数据处理**: 支持高效处理 1000+ 行表格数据，内存使用优化