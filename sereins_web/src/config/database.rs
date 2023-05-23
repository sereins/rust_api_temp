use serde::{Serialize, Deserialize};

/// 数据库配置
#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub name: String,
    // 数据库类型
    pub types: String,
    // 主机
    pub host: String,
    // 端口
    pub port: u16,
    // 数据库
    pub database: String,
    // 用户名
    pub username: String,
    // 密码
    pub password: String,
}

impl DatabaseConfig {
    /// 数据库链接url mysql://root:root@localhost:3306/test
    pub fn get_url(&self) -> String {
        format!(
            "{}://{}:{}@{}:{}/{}",
            self.types, self.username, self.password, self.host, self.port, self.database
        )
    }
}