#!/bin/bash
# 构建和优化脚本

set -e

echo "🦀 开始构建 excel-exporter..."

# 清理旧的构建文件
echo "🧹 清理旧的构建文件..."
cargo clean

# 运行测试
echo "🧪 运行单元测试..."
cargo test --lib

# 构建 release 版本
echo "🔨 构建 release 版本..."
cargo build --release --target wasm32-unknown-unknown

# 使用 wasm-pack 构建
echo "📦 使用 wasm-pack 构建..."
wasm-pack build --target web --out-dir pkg

# 如果安装了 wasm-opt，进行额外优化
if command -v wasm-opt &> /dev/null; then
    echo "⚡ 使用 wasm-opt 优化文件大小..."
    wasm-opt -Oz target/wasm32-unknown-unknown/release/wasm_excel_exporter.wasm \
        -o target/wasm32-unknown-unknown/release/wasm_excel_exporter_opt.wasm

    # 显示文件大小对比
    echo "📊 文件大小对比："
    ls -lh target/wasm32-unknown-unknown/release/wasm_excel_exporter*.wasm
else
    echo "⚠️  wasm-opt 未安装，跳过额外优化"
    echo "   提示：可以通过 'cargo install wasm-opt' 安装"
fi

echo "✅ 构建完成！"
echo ""
echo "📦 构建产物位置："
echo "   - pkg/ (wasm-pack 输出)"
echo "   - target/wasm32-unknown-unknown/release/ (原始 wasm 文件)"

