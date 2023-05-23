use serde::{Serialize, Deserialize};

/// redis配置
#[derive(Debug, Serialize, Deserialize)]
pub struct RedisConfig {
    host: String,
    port: u16,
    password: String,
}

impl RedisConfig {
    /// 返回redis的url
    pub fn get_url(&self, index: Option<i32>) -> String {
        format!(
            "redis://:{}@{}:{}/{}",
            self.password, self.host, self.port, index.unwrap_or(0)
        )
    }
}