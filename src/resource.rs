/// 资源管理模块
///
/// 提供 RAII 风格的资源管理，确保 Web 资源的正确释放

use web_sys::Url;

/// RAII 风格的 URL 资源管理器
///
/// 确保在对象销毁时自动释放 Blob URL 资源
pub struct UrlGuard {
    url: String,
}

impl UrlGuard {
    /// 创建新的 URL 资源管理器
    ///
    /// # 参数
    /// * `url` - 需要管理的 URL 字符串
    ///
    /// # 返回值
    /// 返回 UrlGuard 实例
    pub fn new(url: &str) -> Self {
        Self { url: url.to_string() }
    }
}

impl Drop for UrlGuard {
    fn drop(&mut self) {
        // 确保在对象销毁时释放 URL 资源
        if let Err(e) = Url::revoke_object_url(&self.url) {
            // 记录错误但不阻止程序执行
            // 在实际应用中，这里可以使用 console.error 或其他日志机制
            wasm_bindgen::JsValue::from_str(&format!("释放 URL 资源失败: {:?}", e));
        }
    }
}