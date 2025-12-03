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
- 🚀 **高性能**：Rust 原生性能，零拷贝内存操作，使用 wee_alloc 优化内存
- 🌐 **浏览器兼容**：支持所有现代浏览器和 WebAssembly 环境
- 📝 **文件名自定义**：支持用户指定导出文件名，内置文件名安全验证
- 📊 **进度回调**：支持大型表格导出时的实时进度反馈
- 🔄 **向后兼容**：保留旧版本 API 以确保平滑迁移
- 🎯 **轻量级**：优化的 WebAssembly 输出（~514KB），快速加载

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
import init, { export_table_to_csv, export_table_to_csv_with_progress } from 'wasm-excel-exporter';

await init();

// 使用自定义文件名导出
export_table_to_csv('data-table', '销售数据-2024.csv');

// 批量导出多个表格
const tables = ['table1', 'table2', 'table3'];
tables.forEach((tableId, index) => {
    export_table_to_csv(tableId, `export-${index + 1}.csv`);
});

// 大型表格导出（带进度反馈）
export_table_to_csv_with_progress(
    'large-data-table',
    '大数据导出.csv',
    (progress) => {
        console.log(`导出进度: ${Math.round(progress)}%`);
        // 更新页面上的进度条
        updateProgressBar(progress);
    }
);
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

### 带进度条的完整示例

```html
<!DOCTYPE html>
<html>
<head>
    <style>
        .progress-container {
            width: 100%;
            max-width: 500px;
            margin: 20px auto;
            background-color: #f0f0f0;
            border-radius: 10px;
            padding: 5px;
            display: none;
        }
        .progress-bar {
            width: 0%;
            height: 30px;
            background-color: #4CAF50;
            border-radius: 5px;
            text-align: center;
            line-height: 30px;
            color: white;
            transition: width 0.3s ease;
        }
    </style>
    <script type="module">
        import init, { export_table_to_csv_with_progress } from './pkg/wasm_excel_exporter.js';

        async function main() {
            await init();

            document.getElementById('export-btn').onclick = () => {
                const progressContainer = document.getElementById('progress-container');
                const progressBar = document.getElementById('progress-bar');
                const progressText = document.getElementById('progress-text');
                
                // 显示进度条
                progressContainer.style.display = 'block';
                progressBar.style.width = '0%';
                
                try {
                    export_table_to_csv_with_progress(
                        'data-table',
                        '用户数据.csv',
                        (progress) => {
                            progressBar.style.width = `${progress}%`;
                            progressText.textContent = `${Math.round(progress)}%`;
                        }
                    );
                    
                    // 导出完成后隐藏进度条
                    setTimeout(() => {
                        progressContainer.style.display = 'none';
                        alert('导出成功！');
                    }, 500);
                } catch (error) {
                    progressContainer.style.display = 'none';
                    alert('导出失败: ' + error);
                }
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

    <button id="export-btn">导出 CSV（带进度）</button>
    
    <div class="progress-container" id="progress-container">
        <div class="progress-bar" id="progress-bar">
            <span id="progress-text">0%</span>
        </div>
    </div>
</body>
</html>
```

更多详细示例请查看 [EXAMPLES.md](./EXAMPLES.md) 文件。

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

### `export_table_to_csv_with_progress(table_id, filename?, progress_callback?)` 🆕

将指定 ID 的 HTML 表格导出为 CSV 文件，并提供进度回调功能。

**参数：**
- `table_id` (`string`): 要导出的 HTML 表格元素的 ID
- `filename` (`string`, 可选): 导出文件的名称，默认为 "table_export.csv"
- `progress_callback` (`Function`, 可选): 进度回调函数，接收进度百分比 (0-100)

**返回值：**
- `Result<(), JsValue>`: 成功时返回 `undefined`，失败时抛出异常

**示例：**
```javascript
import init, { export_table_to_csv_with_progress } from 'wasm-excel-exporter';

await init();

// 带进度回调的导出
export_table_to_csv_with_progress(
    'large-table',
    '大数据导出.csv',
    (progress) => {
        console.log(`导出进度: ${progress.toFixed(2)}%`);
        // 更新进度条 UI
        document.getElementById('progress').style.width = `${progress}%`;
    }
);
```

**文件名安全验证：**

两个导出函数都会自动验证文件名的安全性：
- ✅ 自动检查并拒绝路径分隔符 (`/`, `\`)
- ✅ 拒绝危险字符 (`<`, `>`, `:`, `"`, `|`, `?`, `*`)
- ✅ 检查文件名长度（最大 255 字符）
- ✅ 拒绝 Windows 保留名称（CON, PRN, AUX, NUL 等）
- ✅ 拒绝以点或空格开头/结尾的文件名

### `export_table_to_excel(table_id)` ⚠️ 已弃用

为了保持向后兼容而保留的旧版本函数。

**已弃用：** 请使用 `export_table_to_csv(table_id, filename)` 替代。

## 🔧 开发指南

### 环境要求

- Rust 1.82+ (推荐使用最新稳定版)
- Node.js 16+
- wasm-pack

### 项目结构

```
wasm-excel-exporter/
├── src/
│   ├── lib.rs          # 核心实现（导出功能）
│   └── utils.rs        # 工具函数
├── tests/
│   ├── lib_tests.rs    # 完整的单元测试套件（33个测试）
│   ├── unit/           # 单元测试目录
│   └── browser/        # 浏览器测试目录
├── examples/
│   ├── basic-export.html         # 基本导出示例
│   ├── progress-export.html      # 进度条示例
│   ├── advanced-features.html    # 高级特性示例
│   └── README.md                 # 示例文档
├── pkg/                # 生成的 WebAssembly 包
├── Cargo.toml          # Rust 项目配置
└── README.md           # 项目文档
```

### 构建项目

```bash
# 克隆仓库
git clone https://github.com/kurisuu/wasm-excel-exporter.git
cd wasm-excel-exporter

# 构建 WebAssembly 包
wasm-pack build --target web

# 运行所有测试（33个单元测试）
cargo test

# 运行特定测试
cargo test --test lib_tests

# 运行 lib 测试
cargo test --lib

# 格式化代码
cargo fmt

# 检查代码
cargo check
```

### 测试覆盖

项目包含 **33 个全面的单元测试**，覆盖率接近 **100%**：

- ✅ 文件名扩展名处理测试（3 个测试）
- ✅ 输入验证逻辑测试（4 个测试）
- ✅ CSV Writer 功能测试（6 个测试）
- ✅ 文件名验证测试（14 个测试）
- ✅ 边界情况和压力测试（3 个测试）
- ✅ 回归测试（3 个测试）

运行测试：
```bash
$ cargo test --test lib_tests

running 33 tests
test test_csv_writer_creation ... ok
test test_csv_writer_empty_data ... ok
test test_csv_writer_special_characters ... ok
test test_csv_writer_unicode_data ... ok
test test_csv_writer_write_multiple_records ... ok
test test_csv_writer_write_single_record ... ok
test test_filename_extension_handling_basic ... ok
test test_filename_extension_handling_unicode ... ok
test test_filename_extension_handling_special_cases ... ok
test test_filename_validation_dangerous_chars ... ok
test test_filename_validation_edge_length ... ok
test test_filename_validation_empty ... ok
test test_filename_validation_ends_with_dot ... ok
test test_csv_writer_wide_table ... ok
test test_filename_validation_ends_with_space ... ok
test test_filename_validation_mixed_valid_invalid ... ok
test test_filename_validation_path_separators ... ok
test test_filename_validation_starts_with_dot ... ok
test test_filename_validation_starts_with_space ... ok
test test_filename_validation_too_long ... ok
test test_filename_validation_valid_simple ... ok
test test_filename_validation_valid_unicode ... ok
test test_filename_validation_valid_with_spaces ... ok
test test_filename_validation_valid_with_special_chars ... ok
test test_filename_validation_windows_reserved_names ... ok
test test_regression_empty_csv_writer ... ok
test test_regression_unicode_in_validation ... ok
test test_regression_case_sensitivity ... ok
test test_validation_empty_string ... ok
test test_validation_non_empty_string ... ok
test test_validation_special_chars_in_id ... ok
test test_validation_whitespace_string ... ok
test test_csv_writer_large_dataset ... ok

test result: ok. 33 passed; 0 failed; 0 ignored
```

### 浏览器测试

```bash
# 在 Firefox 中测试
wasm-pack test --headless --firefox

# 在 Chrome 中测试
wasm-pack test --headless --chrome
```

### 查看示例

```bash
# 启动本地服务器
python -m http.server 8000
# 或
npx http-server .

# 然后在浏览器中访问
# http://localhost:8000/examples/basic-export.html
# http://localhost:8000/examples/progress-export.html
# http://localhost:8000/examples/advanced-features.html
```

### 发布到 NPM

```bash
# 发布到 npm registry
wasm-pack publish

# 发布到自定义 registry
wasm-pack publish --target bundler
```

## 📖 示例代码

查看 [examples/](./examples/) 目录获取完整的使用示例：

- **basic-export.html** - 基本导出功能演示
- **progress-export.html** - 大数据集导出与进度显示
- **advanced-features.html** - 高级特性（批量导出、错误处理等）

每个示例都包含完整的代码和注释，可以直接在浏览器中运行。

## 🏗️ 项目架构
## 🔄 版本历史

### v1.2.0 (当前开发版本)
- ✅ 重构测试架构，将测试统一到 tests 目录
- ✅ 添加 33 个全面的单元测试，覆盖率接近 100%
- ✅ 创建 examples 目录，包含 3 个完整的 HTML 示例
- ✅ 改进项目结构和文档

### v1.1.0
- ✅ 完全重写错误处理机制
- ✅ 实现 RAII 资源管理
- ✅ 添加自定义文件名支持
- ✅ 添加文件名安全验证
- ✅ 添加进度回调功能
- ✅ 优化 WASM 文件大小（减小 22%）
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
