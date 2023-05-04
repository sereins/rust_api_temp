use serde::{Deserialize, Serialize};

// 服务的配置
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


#[derive(Debug, Serialize, Deserialize)]
// 应用配置
pub struct ApplicationConfig {
    // 调试模式
    pub debug: bool,
    // 日志级别
    pub log_level: String,
    // 服务配置
    pub server_config: ServerConfig,
    pub database: DatabaseConfig,
}

impl ApplicationConfig {
    // 实例化配置
    pub fn new(config: &str) -> Self {
        match toml::from_str(config) {
            Ok(e) => e,
            Err(e) => panic!("{}", e)
        }
    }
}