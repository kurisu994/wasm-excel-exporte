<div align="center">

  <h1><code>wasm-excel-exporter</code></h1>

  <strong>A template for kick starting a Rust and WebAssembly project using <a href="https://github.com/rustwasm/wasm-pack">wasm-pack</a>.</strong>

  <p>
    <a href="https://travis-ci.org/rustwasm/wasm-pack-template"><img src="https://img.shields.io/travis/rustwasm/wasm-pack-template.svg?style=flat-square" alt="Build Status" /></a>
  </p>

  <h3>
    <a href="https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html">Tutorial</a>
    <span> | </span>
    <a href="https://discordapp.com/channels/442252698964721669/443151097398296587">Chat</a>
  </h3>

  <sub>Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a></sub>
</div>

这是一个 Rust WebAssembly 库，用于安全高效地将 HTML 表格数据导出为 CSV 文件。该库使用 `wasm-bindgen` 实现 Rust 与 JavaScript 之间的通信，专门为现代 Web 应用程序设计。

## 🔧 主要改进（v1.1.0）

- ✅ **完全重写的错误处理**：消除所有潜在的 panic 点
- ✅ **内存安全**：使用 RAII 模式确保资源正确释放
- ✅ **输入验证**：全面的参数验证和边界检查
- ✅ **函数重命名**：更准确的命名（CSV 而非 Excel）
- ✅ **自定义文件名**：支持用户指定导出文件名
- ✅ **向后兼容**：保留旧 API 以确保兼容性
- ✅ **Rust Edition 2024**：使用最新的语言特性
- ✅ **更新依赖**：最新的安全版本和性能优化

## Project Structure

- `src/lib.rs`: The main implementation file for the Rust library. It exports a function `export_excel` that takes table data and formats it into an Excel file.
- `Cargo.toml`: The configuration file for the Rust project, defining metadata, dependencies, and build settings.
- `wasm-bindgen.toml`: Configuration file for `wasm-bindgen`, specifying settings for the generated WebAssembly module.


## About

[**📚 Read this template tutorial! 📚**][template-docs]

This template is designed for compiling Rust libraries into WebAssembly and
publishing the resulting package to NPM.

Be sure to check out [other `wasm-pack` tutorials online][tutorials] for other
templates and usages of `wasm-pack`.

[tutorials]: https://rustwasm.github.io/docs/wasm-pack/tutorials/index.html
[template-docs]: https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html

## 🚴 Usage

### 🐑 Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name my-project
cd my-project
```

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
