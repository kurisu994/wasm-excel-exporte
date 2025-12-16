# project_rules.md

> **AI 助手项目规则** - `belobog-stellar-grid` 核心约束和代码修改指南
> 
> **项目版本**：v1.2.1  
> **Rust Edition**：2024  
> **架构**：6 个模块化组件  
> **测试覆盖**：100%（35+ 单元测试）  
> **WASM 大小**：约 117KB

---

## 核心约束（必须遵守）

### 1. 模块化原则

**6 个模块，职责单一，高内聚低耦合**：

```
src/
├── lib.rs              # 主入口：仅做模块声明和重导出（禁止业务逻辑）
├── validation.rs       # 文件名验证：validate_filename() + ensure_extension()
├── resource.rs         # RAII 资源管理：UrlGuard 自动释放 Blob URL
├── core.rs             # 同步导出：CSV/XLSX 基础导出 + 进度回调
├── batch_export.rs     # 异步分批导出：大数据量处理 + yield_to_browser()
└── utils.rs            # 调试工具：set_panic_hook() 开发环境错误显示
```

**禁止事项**：

- ❌ 在 `lib.rs` 中添加**任何**业务逻辑（只能做模块声明和 `pub use` 重导出）
- ❌ 跨模块混合职责（如在 `validation.rs` 中操作 DOM）
- ❌ 绕过模块边界直接访问内部实现（必须通过公共接口）
- ❌ 破坏单一职责原则（每个模块只做一件事）

**模块职责**：
- `validation.rs`：**仅**负责文件名验证和扩展名处理
- `resource.rs`：**仅**负责 RAII 资源管理
- `core.rs`：**仅**负责同步导出功能（CSV/XLSX）
- `batch_export.rs`：**仅**负责异步分批导出功能
- `utils.rs`：**仅**负责调试工具

### 2. 安全优先

**所有用户输入必须验证**：

```rust
// ✅ 正确：先验证，再使用
let filename = validate_filename(&user_input)
    .map_err(|e| JsValue::from_str(&format!("文件名验证失败: {}", e)))?;

// ❌ 错误：直接使用（危险！）
let filename = user_input; // 可能导致路径遍历攻击
```

**强制要求**：

- ✅ 文件名**必须**通过 `validate_filename()` 验证（检查 10+ 种威胁）
  - 路径分隔符、危险字符、Windows 保留名
  - 前后缀点/空格、长度限制、空文件名
- ✅ DOM 操作**必须**检查返回值（使用 `ok_or_else()` 或 `?`）
- ✅ 使用 `Result<T, JsValue>` 而非 `panic!` 或 `unwrap()`
- ✅ 资源管理使用 RAII（`UrlGuard` 自动清理）
- ✅ 错误消息使用**中文**，便于用户理解

### 3. RAII 资源管理

**使用 UrlGuard 自动管理 Blob URL**：

```rust
// ✅ 正确：RAII 自动管理
let url = Url::create_object_url_with_blob(&blob)?;
let _url_guard = UrlGuard::new(&url); // Drop 时自动释放

do_something()?; // 即使这里出错，资源也会被正确释放
// _url_guard 离开作用域时自动调用 Url::revoke_object_url()

// ❌ 错误：手动管理（异常时泄漏）
let url = Url::create_object_url_with_blob(&blob)?;
do_something()?; // 如果这里出错，下面的代码永远不会执行
Url::revoke_object_url(&url)?; // 资源泄漏！
```

**原理**：`UrlGuard` 实现了 `Drop` trait，确保变量离开作用域时自动释放资源，即使发生错误或提前返回。

### 4. 零拷贝原则

**参数使用引用而非所有权转移**：

```rust
// ✅ 正确：使用引用，无内存拷贝
fn process_data(data: &str) {
    // 直接使用数据，不会复制
}

fn validate_filename(filename: &str) -> Result<(), String> {
    // 仅读取，不需要获取所有权
}

// ❌ 错误：获取所有权，可能导致不必要的拷贝
fn process_data(data: String) {
    // 调用者必须传递所有权或克隆数据
}
```

**性能影响**：使用 `&str` 比 `String` 快 10-100 倍（对于大数据），避免堆分配和内存拷贝。

### 5. 中文错误消息

**所有用户可见的错误必须使用中文**：

```rust
// ✅ 正确：详细的中文错误消息
.ok_or_else(|| JsValue::from_str(&format!("找不到 ID 为 '{}' 的表格元素", table_id)))?
.map_err(|e| JsValue::from_str(&format!("获取表格失败: {}", e)))?

// ✅ 正确：验证错误
if filename.is_empty() {
    return Err(JsValue::from_str("文件名不能为空"));
}

// ❌ 错误：英文错误（用户难以理解）
.ok_or_else(|| JsValue::from_str("Table not found"))?

// ❌ 错误：直接转换错误（丢失上下文）
.map_err(|e| JsValue::from(e))?
```

**错误消息规范**：
1. 使用中文，简洁明了
2. 包含上下文信息（如 `table_id`、行号、列号）
3. 说明失败原因和位置

---

## 📋 代码修改规则

### 添加新功能

1. **确定模块**：
   - 导出功能 → `core.rs`（同步）或 `batch_export.rs`（异步）
   - 验证功能 → `validation.rs`
   - 资源管理 → `resource.rs`
   - 调试工具 → `utils.rs`

2. **编写函数**：
   ```rust
   /// 函数说明（中文）
   ///
   /// # 参数
   /// * `param` - 参数说明
   ///
   /// # 返回值
   /// * `Ok(T)` - 成功
   /// * `Err(JsValue)` - 失败，包含中文错误信息
   ///
   /// # 示例
   /// ```javascript
   /// import { new_function } from './pkg/belobog_stellar_grid.js';
   /// await new_function('value');
   /// ```
   #[wasm_bindgen]
   pub fn new_function(param: &str) -> Result<(), JsValue> {
       // 1. 输入验证
       if param.is_empty() {
           return Err(JsValue::from_str("参数不能为空"));
       }
       
       // 2. 业务逻辑
       // ...
       
       // 3. 返回结果
       Ok(())
   }
   ```

3. **在 `lib.rs` 重导出**：
   ```rust
   pub use module::new_function;
   ```

4. **添加测试**（`tests/lib_tests.rs`）：
   ```rust
   #[test]
   fn test_new_function_正常情况() {
       let result = new_function("value");
       assert!(result.is_ok());
   }
   
   #[test]
   fn test_new_function_空参数() {
       let result = new_function("");
       assert!(result.is_err());
   }
   ```

### 修改现有函数

**检查清单**：

- [ ] 是否破坏向后兼容？需要 `#[deprecated]`
- [ ] 是否修改签名？更新所有调用点
- [ ] 是否影响其他模块？测试相关模块
- [ ] 文档注释是否更新？

### 优化性能

**允许**：

- ✅ 使用引用 `&str` / `&[T]`
- ✅ 预分配容量 `Vec::with_capacity()`
- ✅ 分批处理 + `yield_to_browser()`

**禁止**：

- ❌ 使用 `unsafe`（除非必要）
- ❌ 牺牲可读性
- ❌ 绕过安全检查

---

## ✅ 测试要求

**每次修改后必须**：

```bash
# 1. 运行测试
cargo test --lib

# 2. 检查警告
cargo clippy -- -D warnings

# 3. 格式化
cargo fmt
```

**测试命名**：

```rust
#[test]
fn test_<模块>_<函数>_<场景>() { }
```

**必须测试**：

- ✅ 正常输入
- ✅ 边界值
- ✅ 非法输入
- ✅ Unicode
- ✅ 大数据

---

## ⚠️ 常见错误

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

### 3. 跨模块混淆

```rust
// ❌ 错误：在 validation.rs 中操作 DOM
pub fn validate_and_export() { }

// ✅ 正确：职责分离
// validation.rs
pub fn validate_filename(name: &str) -> Result<(), String> { }

// core.rs
pub fn export(...) {
    validate_filename(&filename)?;
}
```

### 4. 使用 panic!

```rust
// ❌ 错误
if data.is_empty() { panic!("数据为空"); }

// ✅ 正确
if data.is_empty() {
    return Err(JsValue::from_str("数据不能为空"));
}
```

### 5. 英文错误

```rust
// ❌ 错误
Err(JsValue::from_str("File not found"))

// ✅ 正确
Err(JsValue::from_str("未找到文件"))
```

---

## 📝 文档规范

### 函数文档（必须）

```rust
/// 简短描述
///
/// # 参数
/// * `param` - 说明
///
/// # 返回值
/// * `Ok(T)` - 成功
/// * `Err(JsValue)` - 失败
#[wasm_bindgen]
pub fn function_name(param: &str) -> Result<(), JsValue>
```

### 模块文档（必须）

```rust
/// 模块名称
///
/// 模块职责说明
```

---

## 🔧 修改前检查清单

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
- [ ] `cargo test --lib` 通过
- [ ] `cargo clippy` 无警告
- [ ] `cargo fmt` 已执行

**文档**：

- [ ] 函数文档（中文）
- [ ] 模块文档更新
- [ ] 复杂逻辑注释

---

## 🎯 关键设计原则

1. **简洁至上**：最简单的解决方案
2. **安全第一**：验证输入、优雅错误处理
3. **用户体验**：中文消息、进度反馈
4. **可维护性**：模块化、文档完整

---

## 🔗 快速参考

### 技术栈

- **Rust**：Edition 2024
- **WebAssembly**：wasm32-unknown-unknown 目标
- **核心依赖**：
  - wasm-bindgen 0.2.106（Rust ↔ JavaScript 互操作）
  - web-sys 0.3.83（DOM API 绑定）
  - csv 1.4.0（CSV 数据序列化）
  - rust_xlsxwriter 0.69.0（Excel 导出）

### 重要路径

- **核心逻辑**：
  - `src/core.rs`（同步导出 CSV/XLSX）
  - `src/batch_export.rs`（异步分批导出）
- **验证**：`src/validation.rs`（文件名安全验证）
- **资源管理**：`src/resource.rs`（RAII UrlGuard）
- **测试**：`tests/lib_tests.rs`（35+ 单元测试）

### 常用命令

```bash
# 运行测试
cargo test --lib

# 代码检查
cargo clippy -- -D warnings

# 格式化
cargo fmt

# 构建 WASM
wasm-pack build --target web --release

# 优化 WASM
wasm-opt -Oz pkg/belobog_stellar_grid_bg.wasm -o pkg/belobog_stellar_grid_bg_opt.wasm
```

### 性能指标

- **WASM 大小**：约 117KB（未压缩）
- **同步导出**：
  - 小表格（<100 行）：<10ms
  - 中表格（100-1000 行）：<100ms
- **异步导出**：
  - 10,000 行：~1.2s（流畅）
  - 100,000 行：~12s（可用）
  - 1,000,000 行：~120s（完全可用）

---
