# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

这是一个企业级的 Rust WebAssembly 库，用于安全高效地将 HTML 表格数据导出为 CSV 和 XLSX 文件。

**核心特性**：
- **版本**：v1.0.0（Rust Edition 2024）
- **架构**：7 个模块化组件（core 模块细分为 4 个子模块）
- **安全性**：RAII 资源管理 + 文件名验证 + 零 panic 设计
- **性能**：零拷贝 + 分批异步处理 + LTO 优化
- **测试**：39 个单元测试，100% 代码覆盖率

项目仓库地址：https://github.com/kurisu994/belobog-stellar-grid

### 版本管理

版本号需在以下文件中保持同步：
- `Cargo.toml` - 主要版本定义（当前：1.0.0）
- `wasm-bindgen.toml` - WebAssembly 构建配置（当前：1.0.0）
- `CLAUDE.md` - 项目文档

## 开发核心约束

### 模块化架构（严格遵循）

项目采用 7 个模块（core 细分为 4 个子模块）：

```
src/
├── lib.rs                     # 主入口：仅做模块声明和重导出
├── validation.rs              # 文件名验证
├── resource.rs                # RAII 资源管理：UrlGuard
├── core/                      # 核心导出模块组
│   ├── mod.rs                 # 协调模块：export_table() 统一 API
│   ├── table_extractor.rs     # 数据提取
│   ├── export_csv.rs          # CSV 导出
│   └── export_xlsx.rs         # Excel 导出
├── batch_export.rs            # 异步分批导出
└── utils.rs                   # 调试工具：set_panic_hook()
```

**架构约束**：
- `lib.rs` 仅做模块声明和 `pub use` 重导出
- 每个模块职责单一，互不干扰
- 禁止在 `lib.rs` 中添加任何业务逻辑

### 安全优先（强制要求）

**输入验证**：
- 文件名必须通过 `validate_filename()` 验证（检查 10+ 种威胁）
- DOM 操作必须检查返回值
- 所有函数返回 `Result<T, JsValue>`，禁止 `panic!`

**RAII 资源管理**：
```rust
// ✅ 正确：RAII 自动管理
let url = Url::create_object_url_with_blob(&blob)?;
let _url_guard = UrlGuard::new(&url); // Drop 时自动调用 revoke_object_url

// ❌ 错误：手动管理（异常时泄漏）
let url = Url::create_object_url_with_blob(&blob)?;
do_something()?;
Url::revoke_object_url(&url)?; // 可能永远不会执行
```

### 零拷贝原则

参数使用引用而非所有权转移：
```rust
// ✅ 正确：使用引用，无内存拷贝
fn process_data(data: &str) { }

// ❌ 错误：获取所有权，可能导致不必要的拷贝
fn process_data(data: String) { }
```

### 中文错误消息

所有用户可见的错误必须使用中文：
```rust
// ✅ 正确：详细的中文错误消息
.ok_or_else(|| JsValue::from_str(&format!("找不到 ID 为 '{}' 的表格元素", table_id)))?

// ❌ 错误：英文错误
.ok_or_else(|| JsValue::from_str("Table not found"))?
```

## 代码修改规范

### 添加新功能

1. 确定模块：导出功能 → `core/`，验证 → `validation.rs`
2. 编写函数（使用 `#[wasm_bindgen]`）
3. 在 `lib.rs` 重导出：`pub use module::new_function;`
4. 添加测试

### 测试要求

每次修改后必须：
```bash
# 1. 运行测试
cargo test

# 2. 检查警告
cargo clippy -- -D warnings

# 3. 格式化
cargo fmt
```

**测试说明**：
- `cargo test --lib` 只运行 `src/` 目录下的测试（当前为 0 个）
- `cargo test` 运行所有测试，包括 `tests/` 目录下的（共 39 个测试）

## 常用命令

### WebAssembly 构建
```bash
# 构建 WebAssembly 包
wasm-pack build

# 构建特定目标
wasm-pack build --target bundler  # webpack/rollup
wasm-pack build --target nodejs    # Node.js
wasm-pack build --target web       # 原生 Web

# 使用项目构建脚本（推荐）
./build.sh  # 自动执行清理、测试、构建和优化

# 浏览器测试
wasm-pack test --headless --firefox
wasm-pack test --headless --chrome
```

### Rust 开发
```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test --test lib_tests      # 35个单元测试
cargo test --test test_unified_api  # 4个API测试

# 检查代码
cargo check

# 格式化代码
cargo fmt

# 代码检查
cargo clippy -- -D warnings

# 构建优化版本
cargo build --release
```

### 构建脚本

`build.sh` 脚本自动化完整的构建流程：
```bash
#!/bin/bash
# 1. 清理旧的构建文件
# 2. 运行所有单元测试
# 3. 构建 release 版本
# 4. 使用 wasm-pack 构建
# 5. 可选：使用 wasm-opt 进行额外优化
```

## 项目架构

### 核心模块

1. **src/lib.rs** - 主入口模块
   - 职责：模块声明和公共 API 重导出
   - 原则：仅做声明和导出，禁止任何业务逻辑

2. **src/validation.rs** - 文件名验证模块
   - `validate_filename(&str) -> Result<(), String>`: 10+ 种安全检查
   - `ensure_extension(&str, &str) -> String`: 确保正确扩展名

3. **src/resource.rs** - RAII 资源管理模块
   - `UrlGuard::new(&str) -> Self`: 创建资源守卫
   - 实现 `Drop` trait，自动调用 `Url::revoke_object_url()`

4. **src/core/mod.rs** - 核心协调模块
   - `export_table(table_id, filename?, format?, callback?) -> Result<(), JsValue>`
   - `ExportFormat` 枚举：`Csv` | `Xlsx`

5. **src/core/table_extractor.rs** - 表格数据提取模块
   - `extract_table_data(table_id) -> Result<Vec<Vec<String>>, JsValue>`

6. **src/core/export_csv.rs** - CSV 导出模块
   - `export_as_csv(table_data, filename, callback) -> Result<(), JsValue>`

7. **src/core/export_xlsx.rs** - Excel 导出模块
   - `export_as_xlsx(table_data, filename, callback) -> Result<(), JsValue>`

8. **src/batch_export.rs** - 分批异步导出模块
   - `export_table_to_csv_batch(...) -> Promise<void>`: 异步导出
   - `yield_to_browser() -> Promise<void>`: 让出控制权给浏览器

9. **src/utils.rs** - WebAssembly 调试工具模块
   - `set_panic_hook()`: 设置 panic 钩子

### 测试模块

- **tests/lib_tests.rs** - 35 个综合单元测试（100% 覆盖）
- **tests/test_unified_api.rs** - 4 个统一 API 测试
- **tests/browser/web_original.rs** - WebAssembly 浏览器测试

### 配置文件

- **Cargo.toml** - Rust 项目配置（Edition 2024）
- **wasm-bindgen.toml** - WebAssembly 构建配置
- **.cargo/config.toml** - Cargo 构建配置

## 核心 API

### 1. 统一导出（推荐）

```rust
pub fn export_table(
    table_id: &str,
    filename: Option<String>,
    format: Option<ExportFormat>,
    progress_callback: Option<js_sys::Function>
) -> Result<(), JsValue>

pub enum ExportFormat {
    Csv,  // 默认
    Xlsx,
}
```

**JavaScript 调用**：
```javascript
import init, { export_table, ExportFormat } from './pkg/belobog_stellar_grid.js';
await init();

// 最简单的用法（CSV）
export_table('my-table');

// 导出为 Excel
export_table('my-table', '报表.xlsx', ExportFormat.Xlsx);

// 带进度回调
export_table('large-table', '大数据', ExportFormat.Csv, (progress) => {
  console.log(`进度: ${progress.toFixed(1)}%`);
});
```

### 2. 分批异步导出（向后兼容）

```rust
pub async fn export_table_to_csv_batch(
    table_id: String,
    tbody_id: Option<String>,
    filename: Option<String>,
    batch_size: Option<u32>,
    progress_callback: Option<js_sys::Function>
) -> Result<JsValue, JsValue>
```

**适用场景**：大数据量（10,000+ 行）

## WebAssembly 特定注意事项

### 构建和优化

**目标平台**：现代浏览器（Chrome 90+, Firefox 88+, Safari 14+, Edge 90+）

**内存分配器**：
- 默认：系统分配器（性能优先）
- 可选：`wee_alloc` 小型分配器（体积优先）

### 文件大小优化

**优化流程**：
```bash
# 1. Release 构建（约 1.3MB）
wasm-pack build --target web --release

# 2. 使用 wasm-opt 优化（目标约 117KB）
wasm-opt -Oz pkg/belobog_stellar_grid_bg.wasm \
    -o pkg/belobog_stellar_grid_bg_opt.wasm
```

**注意**：117KB 是经过 `wasm-opt -Oz` 优化后的目标大小

## 测试覆盖

项目拥有 **100% 的测试覆盖率**，包含 39 个测试：

| 测试类别 | 数量 | 说明 |
| --- | --- | --- |
| 文件名处理 | 5 | 扩展名、Unicode、特殊情况 |
| 输入验证 | 4 | 空字符串、非空、空格、特殊字符 |
| CSV Writer | 6 | 创建、写入、Unicode、特殊字符、大数据 |
| 文件名验证 | 14 | 有效和无效文件名测试 |
| 边界测试 | 3 | 长度、性能、边界值 |
| 回归测试 | 3 | 防止已修复 bug 复现 |
| 统一 API | 4 | 新的 export_table() API 测试 |

## 常见错误及解决方案

### 1. 忘记验证输入
```rust
// ❌ 错误
pub fn export(filename: String) { }

// ✅ 正确
pub fn export(filename: String) -> Result<(), JsValue> {
    validate_filename(&filename).map_err(|e| JsValue::from_str(&e))?;
}
```

### 2. 资源泄漏
```rust
// ❌ 错误：异常时泄漏
let url = Url::create_object_url(&blob)?;
do_something()?;
Url::revoke_object_url(&url)?;

// ✅ 正确：自动清理
let _guard = UrlGuard::new(&url);
do_something()?;
```

### 3. 使用 panic!
```rust
// ❌ 错误
if data.is_empty() { panic!("数据为空"); }

// ✅ 正确
if data.is_empty() {
    return Err(JsValue::from_str("数据不能为空"));
}
```

### 4. 英文错误
```rust
// ❌ 错误
Err(JsValue::from_str("File not found"))

// ✅ 正确
Err(JsValue::from_str("未找到文件"))
```

## 修改前检查清单

**功能性**：
- [ ] 实现预期功能
- [ ] 处理边界情况
- [ ] 错误处理完整（中文）

**安全性**：
- [ ] 验证所有输入
- [ ] 使用 RAII 管理资源
- [ ] 无 `panic!` / `unwrap()`

**架构**：
- [ ] 遵循模块化
- [ ] 未破坏抽象
- [ ] 依赖清晰

**测试**：
- [ ] 添加测试
- [ ] `cargo test` 通过
- [ ] `cargo clippy` 无警告
- [ ] `cargo fmt` 已执行

**文档**：
- [ ] 函数文档（中文）
- [ ] 模块文档更新
- [ ] 复杂逻辑注释

## 性能和安全特性

### 性能优化
- 零拷贝操作：减少 50-70% 的内存分配
- 分批异步处理：支持百万级数据导出
- 编译优化：LTO、opt-level="s"、codegen-units=1

### 安全特性
- 内存安全：Rust 所有权系统保证
- RAII 资源管理：`UrlGuard` 自动释放 Blob URL
- 输入验证：10+ 种威胁检测
- 错误处理：零 `panic!` 和 `unwrap()`
- 大数据处理：分批异步避免页面卡死

## 关键设计原则

1. **简洁至上**：最简单的解决方案
2. **安全第一**：验证输入、优雅错误处理
3. **用户体验**：中文消息、进度反馈
4. **可维护性**：模块化、文档完整