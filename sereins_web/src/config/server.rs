use serde::{Serialize, Deserialize};

/// 服务的配置
#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    host: String,
    port: usize,
}
impl ServerConfig {
    /// 获取stock地址
    pub fn service_format(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
    // 返回服务地址
    pub fn http_addr(&self) -> String {
        format!("http://{}:{}/", self.host, self.port)
    }
}