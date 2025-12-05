# 更新日志 (CHANGELOG)

本文档记录了 wasm-excel-exporter 项目的所有重要变更。

格式遵循 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)，
版本号遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

---

## [Unreleased]

### 改进
- 📝 更新 CLAUDE.md 文档，添加模块化架构说明
- 📚 优化 README.md 性能指标和 WASM 大小显示
- 📄 调整 BUILD_REPORT.md 中的 WASM 文件大小记录
- 🔗 统一所有文档中的仓库地址链接

### 测试
- ✅ 添加分批导出和进度回调的浏览器测试
- 📊 更新测试报告文档反映新增功能

---

## [1.2.1] - 2024-12-04

### 新增
- 🆕 新增 `export_table_to_csv_batch()` 异步导出函数，专为大数据量设计（支持百万级数据）
- ✨ 分批异步处理机制，在批次之间让出控制权给浏览器，避免页面卡死
- 📊 实时进度反馈，页面保持响应，用户可以正常操作
- ⚙️ 可配置批次大小（默认 1000 行/批）
- 🏗️ 支持分离表格结构（独立的 tbody 元素），提升大型表格处理灵活性

### 改进
- 🚀 解决大数据导出时页面卡死问题（100 万行数据从崩溃到流畅）
- 📚 更新 README.md，添加新 API 文档和性能对比数据
- 📝 添加未来优化计划章节，规划后续改进方向
- 🧹 重构代码结构，实现模块化架构（validation, resource, core, batch_export）
- 🎨 优化示例页面，增加虚拟滚动和硬件加速支持

### 性能提升
- **小数据**（1,000 行）：无明显差异（~0.1s）
- **中等数据**（10,000 行）：从卡顿到流畅（~1s → ~1.2s）
- **大数据**（100,000 行）：从卡死到可用（~10s 卡死 → ~12s 流畅）
- **超大数据**（1,000,000 行）：从崩溃到完全可用（崩溃 → ~120s 流畅）

### 技术细节
- 添加 `wasm-bindgen-futures = "0.4"` 依赖
- 实现 `yield_to_browser()` 辅助函数
- 使用 `setTimeout(0)` 在批次间让出控制权
- 更新示例文件 `progress-export.html` 使用新 API
- 扩展 web-sys 依赖以包含 HtmlTableSectionElement 类型

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
