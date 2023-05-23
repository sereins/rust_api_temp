use serde::{Serialize,de::DeserializeOwned,Deserialize};
use axum::body::Body;
use axum::response::Response;

/// 响应的封装

// 通用状态码
pub const CODE_SUCCESS: i16 = 0;
pub const CODE_FAIL: i16 = -1;
pub const CODE_INFO: i16 = 1101;
pub const CODE_WARING: i16 = 1102;
pub const CODE_DANGER: i16 = 1103;

// 特定的业务状态

// 未登陆
pub const CODE_NOT_LOGIN: i16 = 1000;
// 登陆失效(有token,redis中不存在信息)
pub const CODE_LOGIN_EXPIRE: i16 = 1001;

/// http 接口返回类型,基础的code，msg，error_level，data等json数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RespVo<'a, T> {
    pub code: i16,
    pub msg: &'a str,
    pub error_level: ErrorLevel,
    pub data: T,
}

/// 定义错误级别(前端页面展示不同通知类型)
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ErrorLevel {
    // 没有错误
    NONE,
    // 提示信息:比如参数填写错误..
    INFO,
    // 警告级别
    WARING,
    // 危险级别
    DANGER,
}

/// data 为空字符串的实现
impl<'a> RespVo<'a, String> {
    pub fn fail_info(msg: &'a str) -> Self {
        Self {
            code: CODE_INFO,
            msg,
            error_level: ErrorLevel::INFO,
            data: "".to_owned(),
        }
    }
    pub fn fail_waring(msg: &'a str) -> Self {
        Self {
            code: CODE_WARING,
            msg,
            error_level: ErrorLevel::WARING,
            data: "".to_owned(),
        }
    }
}

impl<'a, T> RespVo<'a, T>
    where T: Serialize + DeserializeOwned + Clone
{
    /// 返回
    pub fn from(code: i16, msg: &'a str, error_level: ErrorLevel, data: T) -> Self {
        Self { code, msg, error_level, data }
    }

    /// 成功返回
    pub fn success(data: T) -> Self {
        Self {
            code: CODE_SUCCESS,
            msg: "success",
            error_level: ErrorLevel::NONE,
            data,
        }
    }

    /// 失败返回
    pub fn fail(data: T) -> Self {
        Self {
            code: CODE_FAIL,
            msg: "",
            error_level: ErrorLevel::DANGER,
            data: data.clone(),
        }
    }

    pub fn waring_with_code(code: i16, msg: &'a str, data: T) -> Self {
        Self {
            code,
            msg,
            error_level: ErrorLevel::WARING,
            data: data.clone(),
        }
    }

    pub fn fail_danger(msg: &'a str, data: T) -> Self {
        Self {
            code: CODE_DANGER,
            msg,
            error_level: ErrorLevel::DANGER,
            data: data.clone(),
        }
    }

    /// 返回json响应
    pub fn resp_json(&self) -> Response<Body> {
        Response::builder()
            .extension(|| {})
            .header("Access-Control-Allow-Origin", "*")
            .header("Cache-Control", "no-cache")
            .header("Content-Type", "text/json;charset=UTF-8")
            .body(Body::from(self.to_string()))
            .unwrap()
    }
}

impl<'a, T> ToString for RespVo<'a, T>
    where
        T: Serialize + DeserializeOwned + Clone,
{
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}