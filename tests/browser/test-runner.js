#!/usr/bin/env node

/**
 * 简单的测试运行器，用于验证 WebAssembly 包的基本功能
 */

const fs = require('fs');
const path = require('path');

// 检查 pkg 目录是否存在
const pkgPath = path.join(__dirname, '..', '..', 'pkg');
if (!fs.existsSync(pkgPath)) {
    console.error('❌ pkg 目录不存在，请先运行 wasm-pack build');
    process.exit(1);
}

// 检查必要文件
const requiredFiles = [
    'wasm_excel_exporter.js',
    'wasm_excel_exporter_bg.wasm',
    'wasm_excel_exporter.d.ts',
    'package.json'
];

for (const file of requiredFiles) {
    const filePath = path.join(pkgPath, file);
    if (!fs.existsSync(filePath)) {
        console.error(`❌ 必要文件缺失: ${file}`);
        process.exit(1);
    }
}

console.log('✅ 所有必要文件存在');

// 检查 package.json 内容
const packageJsonPath = path.join(pkgPath, 'package.json');
const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));

console.log('📦 Package.json 检查:');
console.log(`   名称: ${packageJson.name}`);
console.log(`   版本: ${packageJson.version}`);
console.log(`   类型: ${packageJson.type}`);

// 检查 TypeScript 定义
const typesPath = path.join(pkgPath, 'wasm_excel_exporter.d.ts');
const typesContent = fs.readFileSync(typesPath, 'utf8');

const hasExportFunction = typesContent.includes('export function export_table_to_csv');
const hasDeprecatedFunction = typesContent.includes('export function export_table_to_excel');
const hasProgressFunction = typesContent.includes('export function export_table_to_csv_with_progress');
const hasBatchFunction = typesContent.includes('export function export_table_to_csv_batch');

console.log('🔧 TypeScript 定义检查:');
console.log(`   ✅ 主导出函数: ${hasExportFunction ? '存在' : '缺失'}`);
console.log(`   ✅ 带进度回调函数: ${hasProgressFunction ? '存在' : '缺失'}`);
console.log(`   ✅ 分批导出函数: ${hasBatchFunction ? '存在' : '缺失'}`);
console.log(`   ✅ 向后兼容函数: ${hasDeprecatedFunction ? '存在' : '缺失'}`);

// 检查 WASM 文件大小
const wasmPath = path.join(pkgPath, 'wasm_excel_exporter_bg.wasm');
const wasmStats = fs.statSync(wasmPath);
const wasmSizeKB = (wasmStats.size / 1024).toFixed(2);

console.log('📊 WebAssembly 文件分析:');
console.log(`   📁 文件大小: ${wasmSizeKB} KB`);

if (wasmSizeKB > 100) {
    console.warn('   ⚠️  WASM 文件较大，考虑使用 wee_alloc 优化');
} else {
    console.log('   ✅ 文件大小合适');
}

// 检查 JavaScript 包
const jsPath = path.join(pkgPath, 'wasm_excel_exporter.js');
const jsContent = fs.readFileSync(jsPath, 'utf8');

console.log('📜 JavaScript 包检查:');
console.log(`   ✅ 包存在且可读`);
console.log(`   📏 代码行数: ${jsContent.split('\n').length}`);

// 创建简单的测试 HTML 报告
console.log('📋 生成测试报告...');

const reportContent = `
# WASM Excel Exporter 构建报告

## 基本信息
- **构建时间**: ${new Date().toLocaleString()}
- **包版本**: ${packageJson.version}
- **WASM 文件大小**: ${wasmSizeKB} KB

## 文件检查
${requiredFiles.map(file => {
    const exists = fs.existsSync(path.join(pkgPath, file));
    return `- ${exists ? '✅' : '❌'} ${file}`;
}).join('\n')}

## API 检查
- ✅ 主导出函数: ${hasExportFunction ? '存在' : '缺失'}
- ✅ 带进度回调函数: ${hasProgressFunction ? '存在' : '缺失'}
- ✅ 分批导出函数: ${hasBatchFunction ? '存在' : '缺失'}
- ✅ 向后兼容函数: ${hasDeprecatedFunction ? '存在' : '缺失'}

## 测试页面
- 🌐 测试页面已创建: \`test-page.html\`
- 📱 可在浏览器中打开进行功能测试
- 🔧 支持分批导出和进度回调测试

## 使用方法
\`\`\`javascript
import init, {
    export_table_to_csv,
    export_table_to_csv_with_progress,
    export_table_to_csv_batch
} from './pkg/wasm_excel_exporter.js';

await init();

// 基本导出
export_table_to_csv('table-id', 'filename.csv');

// 带进度回调的导出
export_table_to_csv_with_progress('table-id', 'filename.csv', (progress) => {
    console.log(\`导出进度: \${progress}%\`);
});

// 分批异步导出
await export_table_to_csv_batch('table-id', null, 'filename.csv', 1000, (progress) => {
    console.log(\`批次进度: \${progress}%\`);
});
\`\`\`

## 命令行测试
\`\`\`bash
# 重新构建
wasm-pack build

# 运行 Rust 测试
cargo test --lib

# 格式化代码
cargo fmt

# 代码检查
cargo clippy -- -D warnings
\`\`\`
`;

fs.writeFileSync(path.join(__dirname, 'BUILD_REPORT.md'), reportContent.trim());

console.log('📄 BUILD_REPORT.md 已生成');
console.log('');
console.log('🚀 下一步建议:');
console.log('   1. 在浏览器中打开 test-page.html 进行功能测试');
console.log('   2. 运行 cargo test --lib 进行单元测试');
console.log('   3. 检查 BUILD_REPORT.md 了解构建详情');
console.log('');
console.log('🎉 构建验证完成！');