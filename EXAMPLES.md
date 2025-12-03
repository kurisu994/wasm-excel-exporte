# 使用示例

## 基本用法

### 导出表格到 CSV（无进度回调）

```javascript
import init, { export_table_to_csv } from './pkg/wasm_excel_exporter.js';

async function exportTable() {
    // 初始化 WASM 模块
    await init();
    
    try {
        // 导出表格
        export_table_to_csv('my-table', '数据导出_2024');
        console.log('导出成功！');
    } catch (error) {
        console.error('导出失败:', error);
    }
}
```

### 导出表格到 CSV（带进度回调）

```javascript
import init, { export_table_to_csv_with_progress } from './pkg/wasm_excel_exporter.js';

async function exportTableWithProgress() {
    // 初始化 WASM 模块
    await init();
    
    // 创建进度条元素
    const progressBar = document.getElementById('progress-bar');
    const progressText = document.getElementById('progress-text');
    
    // 定义进度回调函数
    const progressCallback = (progress) => {
        progressBar.style.width = `${progress}%`;
        progressText.textContent = `${Math.round(progress)}%`;
        console.log(`导出进度: ${progress.toFixed(2)}%`);
    };
    
    try {
        // 导出表格（带进度回调）
        export_table_to_csv_with_progress(
            'my-table',
            '数据导出_2024',
            progressCallback
        );
        console.log('导出成功！');
    } catch (error) {
        console.error('导出失败:', error);
    }
}
```

## HTML 示例

### 完整的带进度条的导出示例

```html
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>表格导出示例</title>
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
        
        table {
            border-collapse: collapse;
            width: 100%;
            margin: 20px 0;
        }
        
        th, td {
            border: 1px solid #ddd;
            padding: 8px;
            text-align: left;
        }
        
        th {
            background-color: #4CAF50;
            color: white;
        }
        
        button {
            padding: 10px 20px;
            font-size: 16px;
            cursor: pointer;
            background-color: #4CAF50;
            color: white;
            border: none;
            border-radius: 5px;
            margin: 5px;
        }
        
        button:hover {
            background-color: #45a049;
        }
    </style>
</head>
<body>
    <h1>表格导出示例</h1>
    
    <table id="my-table">
        <thead>
            <tr>
                <th>姓名</th>
                <th>年龄</th>
                <th>职位</th>
                <th>部门</th>
            </tr>
        </thead>
        <tbody>
            <tr>
                <td>张三</td>
                <td>28</td>
                <td>工程师</td>
                <td>研发部</td>
            </tr>
            <tr>
                <td>李四</td>
                <td>32</td>
                <td>设计师</td>
                <td>设计部</td>
            </tr>
            <tr>
                <td>王五</td>
                <td>25</td>
                <td>产品经理</td>
                <td>产品部</td>
            </tr>
        </tbody>
    </table>
    
    <div>
        <button onclick="exportBasic()">基本导出</button>
        <button onclick="exportWithProgress()">带进度条导出</button>
    </div>
    
    <div class="progress-container" id="progress-container">
        <div class="progress-bar" id="progress-bar">
            <span id="progress-text">0%</span>
        </div>
    </div>
    
    <script type="module">
        import init, { 
            export_table_to_csv, 
            export_table_to_csv_with_progress 
        } from './pkg/wasm_excel_exporter.js';
        
        // 初始化 WASM 模块
        await init();
        
        // 基本导出
        window.exportBasic = function() {
            try {
                export_table_to_csv('my-table', '员工数据_基本导出');
                alert('导出成功！');
            } catch (error) {
                alert('导出失败: ' + error);
            }
        };
        
        // 带进度条的导出
        window.exportWithProgress = function() {
            const progressContainer = document.getElementById('progress-container');
            const progressBar = document.getElementById('progress-bar');
            const progressText = document.getElementById('progress-text');
            
            // 显示进度条
            progressContainer.style.display = 'block';
            progressBar.style.width = '0%';
            progressText.textContent = '0%';
            
            // 进度回调函数
            const progressCallback = (progress) => {
                progressBar.style.width = `${progress}%`;
                progressText.textContent = `${Math.round(progress)}%`;
            };
            
            try {
                export_table_to_csv_with_progress(
                    'my-table',
                    '员工数据_进度导出',
                    progressCallback
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
    </script>
</body>
</html>
```

## 文件名验证示例

库会自动验证文件名的安全性：

```javascript
// ✅ 有效的文件名
export_table_to_csv('my-table', 'valid_filename');
export_table_to_csv('my-table', 'report-2024');
export_table_to_csv('my-table', '数据导出_test');

// ❌ 无效的文件名（会抛出错误）
try {
    export_table_to_csv('my-table', 'path/to/file'); // 包含路径分隔符
} catch (error) {
    console.error('文件名验证失败:', error);
}

try {
    export_table_to_csv('my-table', 'invalid<name'); // 包含非法字符
} catch (error) {
    console.error('文件名验证失败:', error);
}

try {
    export_table_to_csv('my-table', 'CON'); // Windows 保留名称
} catch (error) {
    console.error('文件名验证失败:', error);
}
```

## 错误处理示例

```javascript
async function safeExport(tableId, filename) {
    try {
        await init();
        export_table_to_csv(tableId, filename);
        return { success: true };
    } catch (error) {
        // 处理不同类型的错误
        if (error.includes('找不到')) {
            console.error('表格元素不存在');
        } else if (error.includes('文件名')) {
            console.error('文件名不合法');
        } else if (error.includes('为空')) {
            console.error('表格没有数据');
        } else {
            console.error('未知错误:', error);
        }
        return { success: false, error: error };
    }
}

// 使用示例
const result = await safeExport('my-table', '数据导出');
if (result.success) {
    console.log('导出成功！');
} else {
    console.error('导出失败:', result.error);
}
```

## 大型表格优化建议

对于大型表格（超过 1000 行），建议使用带进度回调的版本：

```javascript
async function exportLargeTable(tableId, filename) {
    await init();
    
    // 显示加载提示
    const loadingDiv = document.createElement('div');
    loadingDiv.textContent = '正在导出...';
    document.body.appendChild(loadingDiv);
    
    try {
        export_table_to_csv_with_progress(
            tableId,
            filename,
            (progress) => {
                loadingDiv.textContent = `正在导出... ${Math.round(progress)}%`;
            }
        );
        loadingDiv.textContent = '导出成功！';
    } catch (error) {
        loadingDiv.textContent = '导出失败: ' + error;
    } finally {
        setTimeout(() => {
            document.body.removeChild(loadingDiv);
        }, 2000);
    }
}
```

## React 集成示例

```jsx
import React, { useState, useEffect } from 'react';
import init, { export_table_to_csv_with_progress } from './pkg/wasm_excel_exporter.js';

function TableExporter({ tableId, filename }) {
    const [wasmReady, setWasmReady] = useState(false);
    const [progress, setProgress] = useState(0);
    const [exporting, setExporting] = useState(false);
    
    useEffect(() => {
        init().then(() => setWasmReady(true));
    }, []);
    
    const handleExport = async () => {
        if (!wasmReady) {
            alert('WASM 模块尚未加载完成');
            return;
        }
        
        setExporting(true);
        setProgress(0);
        
        try {
            await export_table_to_csv_with_progress(
                tableId,
                filename,
                (p) => setProgress(p)
            );
            alert('导出成功！');
        } catch (error) {
            alert('导出失败: ' + error);
        } finally {
            setExporting(false);
            setProgress(0);
        }
    };
    
    return (
        <div>
            <button onClick={handleExport} disabled={!wasmReady || exporting}>
                {exporting ? `导出中... ${Math.round(progress)}%` : '导出表格'}
            </button>
            {exporting && (
                <div style={{ width: '100%', backgroundColor: '#f0f0f0' }}>
                    <div style={{
                        width: `${progress}%`,
                        height: '20px',
                        backgroundColor: '#4CAF50',
                        transition: 'width 0.3s'
                    }} />
                </div>
            )}
        </div>
    );
}

export default TableExporter;
```

## Vue 集成示例

```vue
<template>
  <div>
    <button @click="handleExport" :disabled="!wasmReady || exporting">
      {{ exporting ? `导出中... ${Math.round(progress)}%` : '导出表格' }}
    </button>
    <div v-if="exporting" class="progress-container">
      <div class="progress-bar" :style="{ width: `${progress}%` }"></div>
    </div>
  </div>
</template>

<script>
import { ref, onMounted } from 'vue';
import init, { export_table_to_csv_with_progress } from './pkg/wasm_excel_exporter.js';

export default {
  name: 'TableExporter',
  props: {
    tableId: {
      type: String,
      required: true
    },
    filename: {
      type: String,
      default: 'table_export'
    }
  },
  setup(props) {
    const wasmReady = ref(false);
    const progress = ref(0);
    const exporting = ref(false);
    
    onMounted(async () => {
      await init();
      wasmReady.value = true;
    });
    
    const handleExport = async () => {
      if (!wasmReady.value) {
        alert('WASM 模块尚未加载完成');
        return;
      }
      
      exporting.value = true;
      progress.value = 0;
      
      try {
        await export_table_to_csv_with_progress(
          props.tableId,
          props.filename,
          (p) => { progress.value = p; }
        );
        alert('导出成功！');
      } catch (error) {
        alert('导出失败: ' + error);
      } finally {
        exporting.value = false;
        progress.value = 0;
      }
    };
    
    return {
      wasmReady,
      progress,
      exporting,
      handleExport
    };
  }
};
</script>

<style scoped>
.progress-container {
  width: 100%;
  height: 20px;
  background-color: #f0f0f0;
  margin-top: 10px;
}

.progress-bar {
  height: 100%;
  background-color: #4CAF50;
  transition: width 0.3s;
}
</style>
```

