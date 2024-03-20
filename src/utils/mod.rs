pub fn read_json_from_str(json_str: &str) -> Result<serde_json::Value, serde_json::Error> {
    // 使用serde_json库解析JSON字符串
    let value: serde_json::Value = serde_json::from_str(json_str)?;
    Ok(value)
}
