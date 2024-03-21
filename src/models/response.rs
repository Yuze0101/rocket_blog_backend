use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MyResponse<T> {
    code: i32,
    success: bool,
    message: Option<String>, // 或者使用一个枚举来表示不同的错误信息
    result: Option<T>,
}

impl<T> MyResponse<T> {
    pub fn new(code: i32, success: bool, message: Option<String>, result: Option<T>) -> Self {
        Self {
            code,
            success,
            message,
            result,
        }
    }
}
