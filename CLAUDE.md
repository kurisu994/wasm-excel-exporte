# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

这是一个使用 Rust 编写的 WebAssembly 库，用于将 HTML 表格数据导出为 Excel (CSV) 文件。该库通过 `wasm-bindgen` 与 JavaScript 进行交互，可以从网页中提取表格内容并生成下载链接。

## 常用命令

### 构建和开发
```bash
# 构建 WebAssembly 包
wasm-pack build

# 在浏览器中进行测试
wasm-pack test --headless --firefox
# 或使用 Chrome
wasm-pack test --headless --chrome

# 发布到 NPM
wasm-pack publish
```

### Rust 开发
```bash
# 运行 Rust 测试（在本地环境中）
cargo test

# 检查代码
cargo check

# 格式化代码
cargo fmt
```

## 项目架构

### 核心模块
- **src/lib.rs**: 主要实现文件，包含 `export_table_to_excel` 函数，这是库的核心功能
  - 接受 HTML 表格的 ID 作为参数
  - 遍历表格的行和列，提取文本内容
  - 使用 CSV 格式写入数据
  - 创建 Blob 对象并触发浏览器下载

- **src/utils.rs**: 工具模块，提供 panic hook 设置功能
  - `set_panic_hook()`: 在开发环境中提供更好的错误信息

### 依赖关系
- **wasm-bindgen**: Rust 与 JavaScript 之间的桥梁
- **web-sys**: 提供 Web API 的 Rust 绑定，包括 DOM 操作
- **csv**: CSV 文件生成库
- **js-sys**: JavaScript 系统类型的 Rust 绑定

### 测试结构
- **tests/web.rs**: 浏览器环境下的 WebAssembly 测试
  - 使用 `wasm-bindgen-test` 框架
  - 配置为在浏览器中运行测试

## 开发注意事项

### WebAssembly 特定
- 这是一个 `crate-type = ["cdylib", "rlib"]` 项目，支持编译为动态库和静态库
- 使用 `#[wasm-bindgen]` 宏导出函数到 JavaScript
- 错误处理使用 `Result<T, JsValue>` 类型以与 JavaScript 互操作

### 功能特性
- **console_error_panic_hook**: 开发特性，提供更好的 panic 信息
- **wee_alloc**: 可选的小型内存分配器（需要 nightly Rust）

### JavaScript 集成
该库设计为在前端浏览器环境中使用，通过以下步骤工作：
1. 通过 DOM API 查找指定 ID 的表格元素
2. 遍历表格结构，提取单元格文本内容
3. 将数据序列化为 CSV 格式
4. 创建 Blob 对象并生成临时下载链接
5. 触发浏览器下载并清理资源

## 核心 API

```rust
#[wasm_bindgen]
pub fn export_table_to_excel(table_id: &str) -> Result<(), JsValue>
```

- **参数**: `table_id` - 要导出的 HTML 表格元素的 ID
- **返回**: `Result<(), JsValue>` - 成功时返回 Ok(())，失败时返回 JsValue 错误
- **功能**: 将指定表格导出为 CSV 文件并自动触发浏览器下载