<div align="center">

  <h1><code>wasm-excel-exporter</code></h1>

  <p><strong>🦀 一个安全高效的 Rust WebAssembly 库，用于将 HTML 表格数据导出为 CSV 文件</strong></p>

  <p>
    <img src="https://img.shields.io/badge/version-1.1.0-blue.svg" alt="Version" />
    <img src="https://img.shields.io/badge/rust-edition%202024-orange.svg" alt="Rust Edition" />
    <img src="https://img.shields.io/badge/license-MIT%2FApache--2.0-green.svg" alt="License" />
    <img src="https://img.shields.io/badge/wasm-pack-supported-purple.svg" alt="wasm-pack" />
  </p>

  <sub>Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a></sub>
</div>

## 📋 概述

`wasm-excel-exporter` 是一个高性能的 WebAssembly 库，专门用于在现代 Web 浏览器中将 HTML 表格数据安全地导出为 CSV 文件。该库采用 Rust 编写，通过 `wasm-bindgen` 实现与 JavaScript 的无缝集成，具有企业级的错误处理和内存安全保障。

### ✨ 核心特性

- 🔒 **内存安全**：使用 RAII 模式确保资源自动管理和正确释放
- 🛡️ **错误处理**：全面的输入验证和异常处理，消除所有潜在的 panic 点
- 🚀 **高性能**：Rust 原生性能，零拷贝内存操作
- 🌐 **浏览器兼容**：支持所有现代浏览器和 WebAssembly 环境
- 📝 **文件名自定义**：支持用户指定导出文件名
- 🔄 **向后兼容**：保留旧版本 API 以确保平滑迁移
- 🎯 **轻量级**：优化的 WebAssembly 输出，快速加载

## 🚀 快速开始

### 安装

```bash
# 使用 npm 安装
npm install wasm-excel-exporter

# 或使用 yarn
yarn add wasm-excel-exporter
```

### 基本用法

```javascript
import init, { export_table_to_csv } from 'wasm-excel-exporter';

// 初始化 WebAssembly 模块
await init();

// 导出表格到 CSV 文件
try {
    export_table_to_csv('my-table-id');
    console.log('表格导出成功！');
} catch (error) {
    console.error('导出失败:', error);
}
```

### 高级用法

```javascript
import init, { export_table_to_csv } from 'wasm-excel-exporter';

await init();

// 使用自定义文件名导出
export_table_to_csv('data-table', '销售数据-2024.csv');

// 批量导出多个表格
const tables = ['table1', 'table2', 'table3'];
tables.forEach((tableId, index) => {
    export_table_to_csv(tableId, `export-${index + 1}.csv`);
});
```

### HTML 示例

```html
<!DOCTYPE html>
<html>
<head>
    <script type="module">
        import init, { export_table_to_csv } from './pkg/wasm_excel_exporter.js';

        async function main() {
            await init();

            document.getElementById('export-btn').onclick = () => {
                export_table_to_csv('data-table', '用户数据.csv');
            };
        }

        main();
    </script>
</head>
<body>
    <table id="data-table">
        <thead>
            <tr>
                <th>姓名</th>
                <th>年龄</th>
                <th>城市</th>
            </tr>
        </thead>
        <tbody>
            <tr>
                <td>张三</td>
                <td>28</td>
                <td>北京</td>
            </tr>
            <tr>
                <td>李四</td>
                <td>32</td>
                <td>上海</td>
            </tr>
        </tbody>
    </table>

    <button id="export-btn">导出 CSV</button>
</body>
</html>
```

## 📚 API 参考

### `export_table_to_csv(table_id, filename?)`

将指定 ID 的 HTML 表格导出为 CSV 文件。

**参数：**
- `table_id` (`string`): 要导出的 HTML 表格元素的 ID
- `filename` (`string`, 可选): 导出文件的名称，默认为 "table_export.csv"

**返回值：**
- `Result<(), JsValue>`: 成功时返回 `undefined`，失败时抛出异常

**示例：**
```javascript
// 使用默认文件名
export_table_to_csv('my-table');

// 使用自定义文件名
export_table_to_csv('my-table', 'report.csv');
```

### `export_table_to_excel(table_id)` ⚠️ 已弃用

为了保持向后兼容而保留的旧版本函数。

**已弃用：** 请使用 `export_table_to_csv(table_id, filename)` 替代。

## 🔧 开发指南

### 环境要求

- Rust 1.75+ (推荐使用最新稳定版)
- Node.js 16+
- wasm-pack

### 构建项目

```bash
# 克隆仓库
git clone https://github.com/kurisuu/wasm-excel-exporter.git
cd wasm-excel-exporter

# 构建 WebAssembly 包
wasm-pack build

# 运行 Rust 测试
cargo test

# 格式化代码
cargo fmt

# 检查代码
cargo check
```

### 浏览器测试

```bash
# 在 Firefox 中测试
wasm-pack test --headless --firefox

# 在 Chrome 中测试
wasm-pack test --headless --chrome
```

### 发布到 NPM

```bash
# 发布到 npm registry
wasm-pack publish

# 发布到自定义 registry
wasm-pack publish --target bundler
```

## 🏗️ 项目架构

```
wasm-excel-exporter/
├── src/
│   ├── lib.rs          # 核心实现文件
│   └── utils.rs        # 工具函数模块
├── tests/
│   └── web.rs          # 浏览器环境测试
├── pkg/                # 生成的 WebAssembly 包
├── Cargo.toml          # Rust 项目配置
├── README.md           # 项目文档
└── CLAUDE.md           # Claude Code 指令
```

### 核心模块

- **`src/lib.rs`**: 主要实现文件，包含表格导出逻辑
  - `export_table_to_csv()`: 主要导出函数
  - `UrlGuard`: RAII 风格的资源管理器
  - 完善的错误处理和输入验证

- **`src/utils.rs`**: 工具模块
  - `set_panic_hook()`: 开发环境调试支持

### 技术栈

- **核心语言**: Rust (Edition 2024)
- **WebAssembly**: wasm-bindgen
- **Web API**: web-sys
- **CSV 处理**: csv crate
- **JavaScript 互操作**: js-sys
- **内存分配**: wee_alloc (可选)
- **调试支持**: console_error_panic_hook

## 🔄 版本历史

### v1.1.0 (当前版本)
- ✅ 完全重写错误处理机制
- ✅ 实现 RAII 资源管理
- ✅ 添加自定义文件名支持
- ✅ 更新至 Rust Edition 2024
- ✅ 依赖项安全更新

### v1.0.x
- 🎉 初始版本发布
- 📦 基础表格导出功能
- 🔗 WebAssembly 集成


### 开发流程

1. Fork 项目
2. 创建功能分支: `git checkout -b feature/amazing-feature`
3. 提交更改: `git commit -m '添加某个功能'`
4. 推送分支: `git push origin feature/amazing-feature`
5. 创建 Pull Request

## 📄 许可证

本项目采用双重许可证：

- [MIT License](LICENSE-MIT)
- [Apache License 2.0](LICENSE-APACHE)

您可以选择其中任一许可证使用本项目。

## 🔗 相关链接

- [wasm-pack 文档](https://rustwasm.github.io/docs/wasm-pack/)
- [Rust and WebAssembly 工作组](https://rustwasm.github.io/)
- [WebAssembly 官方网站](https://webassembly.org/)
- [CSV 格式规范](https://tools.ietf.org/html/rfc4180)

## 🆘 支持

如果您遇到问题或有疑问，请：

1. 查看 [FAQ](docs/FAQ.md)
2. 搜索现有的 [Issues](https://github.com/kurisuu/wasm-excel-exporter/issues)
3. 创建新的 Issue 描述您的问题
4. 加入我们的 [讨论区](https://github.com/kurisuu/wasm-excel-exporter/discussions)

---

<div align="center">

**⭐ 如果这个项目对您有帮助，请给我们一个 Star！**

Made with ❤️ by Kurisu

</div>
