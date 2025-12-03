# 更新日志 (CHANGELOG)

本文档记录了 wasm-excel-exporter 项目的所有重要变更。

格式遵循 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)，
版本号遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

---


## [1.2.0] - 2024-12-03

### 新增
- 新增 33 个全面的单元测试，测试覆盖率达到 100%
- 新增 `tests/lib_tests.rs` 作为主测试文件
- 新增 3 个完整的 HTML 示例：`basic-export.html`、`progress-export.html`、`advanced-features.html`
- 新增 `examples/README.md` 详细示例文档，包含 React 和 Vue 集成示例
- 新增测试文档 `tests/README.md` 和构建报告 `tests/BUILD_REPORT.md`

### 改进
- 完全重写 `README.md`，现代化设计，包含 TypeScript 类型定义和框架集成示例
- 重构项目结构，清理冗余测试文件，移除 `tests/unit/unit_tests.rs`
- 移除 `src/lib.rs` 中的内联测试，统一到 tests 目录
- 测试覆盖率从 ~30% 提升到 100%
- 更新 `examples/README.md`，推荐使用 `basic-http-server`

### 修复
- 将 `validate_filename` 改为公开函数，便于测试访问

---

## [1.1.0] - 2024-12-03

### 新增
- 新增 `export_table_to_csv_with_progress()` 函数，支持实时进度反馈（0-100%）
- 新增完整的文件名安全验证功能，包括路径分隔符、危险字符、Windows 保留名称、长度限制等检查
- 新增 `EXAMPLES.md` 文档，包含 HTML、React、Vue 集成示例

### 改进
- WASM 文件大小优化 22%（661KB → 514KB，使用 wasm-opt -Oz）
- 启用 `wee_alloc` 作为全局分配器
- 启用 LTO（链接时优化）和代码大小优化（opt-level = "z"）
- 增强错误信息的可读性

### 修复
- 修复 wasm32 目标无法运行测试的问题，创建 `.cargo/config.toml` 配置文件
- 修复文件名验证不完整的问题
- 修复 Unicode 文件名处理

### 依赖更新
- 更新至 Rust Edition 2024
- 更新 wasm-bindgen 到 0.2.106
- 更新 web-sys 到 0.3.83
- 更新 js-sys 到 0.3.83

---

## [1.0.0] - 2024-10

### 初始发布

#### 核心功能
- 将 HTML 表格导出为 CSV 文件
- RAII 资源管理，自动管理和释放资源
- 全面的错误处理
- 支持自定义文件名
- 向后兼容 API

#### API
- `export_table_to_csv(table_id, filename?)` - 主要导出函数
- `export_table_to_excel(table_id)` - 向后兼容函数（已标记弃用）

#### 技术栈
- Rust Edition 2021
- wasm-bindgen、web-sys、js-sys、csv crate

