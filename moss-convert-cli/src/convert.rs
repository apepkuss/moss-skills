use anyhow::{bail, Context, Result};
use csv::{ReaderBuilder, WriterBuilder};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

/// 展平嵌套字典，用 `.` 作为分隔符
fn flatten_dict(value: &Value, parent_key: &str, sep: &str) -> BTreeMap<String, String> {
    let mut items = BTreeMap::new();

    if let Value::Object(map) = value {
        for (k, v) in map {
            let new_key = if parent_key.is_empty() {
                k.clone()
            } else {
                format!("{}{}{}", parent_key, sep, k)
            };

            match v {
                Value::Object(_) => {
                    items.extend(flatten_dict(v, &new_key, sep));
                }
                Value::Array(_) => {
                    items.insert(new_key, serde_json::to_string(v).unwrap_or_default());
                }
                Value::Null => {
                    items.insert(new_key, String::new());
                }
                Value::Bool(b) => {
                    items.insert(new_key, b.to_string());
                }
                Value::Number(n) => {
                    items.insert(new_key, n.to_string());
                }
                Value::String(s) => {
                    items.insert(new_key, s.clone());
                }
            }
        }
    }

    items
}

/// 还原展平的字典
#[allow(dead_code)]
pub fn unflatten_dict(flat: &BTreeMap<String, String>, sep: &str) -> Value {
    let mut result = serde_json::Map::new();

    for (key, value) in flat {
        let parts: Vec<&str> = key.split(sep).collect();
        let mut current = &mut result;

        for part in &parts[..parts.len() - 1] {
            let part_str = (*part).to_string();
            if !current.contains_key(&part_str) {
                current.insert(part_str.clone(), Value::Object(serde_json::Map::new()));
            }
            current = current.get_mut(&part_str).unwrap().as_object_mut().unwrap();
        }

        let last_key = parts[parts.len() - 1].to_string();
        let parsed_value = try_parse_json_value(value);
        current.insert(last_key, parsed_value);
    }

    Value::Object(result)
}

/// 尝试将字符串解析为 JSON 值（数组或对象），否则保持字符串
#[allow(dead_code)]
pub fn try_parse_json_value(s: &str) -> Value {
    if s.starts_with('[') || s.starts_with('{') {
        if let Ok(v) = serde_json::from_str::<Value>(s) {
            return v;
        }
    }
    Value::String(s.to_string())
}

/// 尝试将字符串转换为数值类型
fn try_parse_number(s: &str) -> Value {
    if s.is_empty() {
        return Value::String(String::new());
    }
    if s.contains('.') {
        if let Ok(f) = s.parse::<f64>() {
            return serde_json::Number::from_f64(f)
                .map(Value::Number)
                .unwrap_or_else(|| Value::String(s.to_string()));
        }
    } else if let Ok(i) = s.parse::<i64>() {
        return Value::Number(i.into());
    }
    Value::String(s.to_string())
}

/// CSV 转 JSON
pub fn csv_to_json(
    input_path: &str,
    output_path: Option<&str>,
    delimiter: u8,
    encoding: &str,
) -> Result<()> {
    let input = Path::new(input_path);
    if !input.exists() {
        bail!("文件不存在: {}", input_path);
    }

    let content = if encoding.eq_ignore_ascii_case("utf-8") || encoding.eq_ignore_ascii_case("utf8")
    {
        fs::read_to_string(input).with_context(|| format!("无法读取文件: {}", input_path))?
    } else {
        read_file_with_encoding(input_path, encoding)?
    };

    let mut reader = ReaderBuilder::new()
        .delimiter(delimiter)
        .from_reader(content.as_bytes());

    let headers = reader.headers()?.clone();
    let mut data: Vec<Value> = Vec::new();

    for result in reader.records() {
        let record = result?;
        let mut row = serde_json::Map::new();

        for (i, field) in record.iter().enumerate() {
            if let Some(header) = headers.get(i) {
                row.insert(header.to_string(), try_parse_number(field));
            }
        }

        data.push(Value::Object(row));
    }

    let json_output = serde_json::to_string_pretty(&data)?;

    if let Some(out_path) = output_path {
        fs::write(out_path, &json_output).with_context(|| format!("无法写入文件: {}", out_path))?;
        eprintln!("✓ 已转换: {} -> {}", input_path, out_path);
        eprintln!("  共 {} 条记录", data.len());
    } else {
        io::stdout().write_all(json_output.as_bytes())?;
        println!();
    }

    Ok(())
}

/// JSON 转 CSV
pub fn json_to_csv(
    input_path: &str,
    output_path: Option<&str>,
    delimiter: u8,
    encoding: &str,
) -> Result<()> {
    let input = Path::new(input_path);
    if !input.exists() {
        bail!("文件不存在: {}", input_path);
    }

    let content = if encoding.eq_ignore_ascii_case("utf-8") || encoding.eq_ignore_ascii_case("utf8")
    {
        fs::read_to_string(input).with_context(|| format!("无法读取文件: {}", input_path))?
    } else {
        read_file_with_encoding(input_path, encoding)?
    };

    let data: Value = serde_json::from_str(&content).with_context(|| "JSON 解析错误")?;

    let array = data
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("JSON 必须是数组格式"))?;

    if array.is_empty() {
        bail!("JSON 数组为空");
    }

    // 展平嵌套结构并收集所有字段名
    let mut flattened_data: Vec<BTreeMap<String, String>> = Vec::new();
    let mut fieldnames: Vec<String> = Vec::new();

    for item in array {
        let flat = if item.is_object() {
            flatten_dict(item, "", ".")
        } else {
            let mut m = BTreeMap::new();
            m.insert(
                "value".to_string(),
                match item {
                    Value::String(s) => s.clone(),
                    other => other.to_string(),
                },
            );
            m
        };

        for key in flat.keys() {
            if !fieldnames.contains(key) {
                fieldnames.push(key.clone());
            }
        }

        flattened_data.push(flat);
    }

    let csv_output = build_csv(&fieldnames, &flattened_data, delimiter)?;

    if let Some(out_path) = output_path {
        fs::write(out_path, &csv_output).with_context(|| format!("无法写入文件: {}", out_path))?;
        eprintln!("✓ 已转换: {} -> {}", input_path, out_path);
        eprintln!(
            "  共 {} 条记录, {} 列",
            flattened_data.len(),
            fieldnames.len()
        );
    } else {
        io::stdout().write_all(csv_output.as_bytes())?;
    }

    Ok(())
}

fn build_csv(
    fieldnames: &[String],
    data: &[BTreeMap<String, String>],
    delimiter: u8,
) -> Result<String> {
    let mut wtr = WriterBuilder::new()
        .delimiter(delimiter)
        .from_writer(Vec::new());

    wtr.write_record(fieldnames)?;

    for row in data {
        let record: Vec<&str> = fieldnames
            .iter()
            .map(|f| row.get(f).map(|s| s.as_str()).unwrap_or(""))
            .collect();
        wtr.write_record(&record)?;
    }

    let bytes = wtr.into_inner()?;
    Ok(String::from_utf8(bytes)?)
}

fn read_file_with_encoding(path: &str, encoding: &str) -> Result<String> {
    // 对于非 UTF-8 编码，先读取字节再尝试转换
    let bytes = fs::read(path).with_context(|| format!("无法读取文件: {}", path))?;

    match encoding.to_lowercase().as_str() {
        "latin-1" | "latin1" | "iso-8859-1" => Ok(bytes.iter().map(|&b| b as char).collect()),
        _ => {
            // 尝试作为 UTF-8 解码
            String::from_utf8(bytes).with_context(|| format!("文件编码不是有效的 {}", encoding))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_temp_csv(content: &str) -> NamedTempFile {
        let mut f = NamedTempFile::new().unwrap();
        f.write_all(content.as_bytes()).unwrap();
        f.flush().unwrap();
        f
    }

    fn create_temp_json(content: &str) -> NamedTempFile {
        let mut f = NamedTempFile::new().unwrap();
        f.write_all(content.as_bytes()).unwrap();
        f.flush().unwrap();
        f
    }

    #[test]
    fn test_flatten_dict_simple() {
        let json: Value = serde_json::json!({"name": "Alice", "age": 30});
        let flat = flatten_dict(&json, "", ".");
        assert_eq!(flat.get("name").unwrap(), "Alice");
        assert_eq!(flat.get("age").unwrap(), "30");
    }

    #[test]
    fn test_flatten_dict_nested() {
        let json: Value = serde_json::json!({
            "name": "Alice",
            "address": {"city": "Beijing", "zip": "100000"}
        });
        let flat = flatten_dict(&json, "", ".");
        assert_eq!(flat.get("address.city").unwrap(), "Beijing");
        assert_eq!(flat.get("address.zip").unwrap(), "100000");
    }

    #[test]
    fn test_flatten_dict_array() {
        let json: Value = serde_json::json!({"tags": ["a", "b"]});
        let flat = flatten_dict(&json, "", ".");
        assert_eq!(flat.get("tags").unwrap(), r#"["a","b"]"#);
    }

    #[test]
    fn test_unflatten_dict() {
        let mut flat = BTreeMap::new();
        flat.insert("name".to_string(), "Alice".to_string());
        flat.insert("address.city".to_string(), "Beijing".to_string());
        let result = unflatten_dict(&flat, ".");
        assert_eq!(result["name"], "Alice");
        assert_eq!(result["address"]["city"], "Beijing");
    }

    #[test]
    fn test_unflatten_dict_with_array() {
        let mut flat = BTreeMap::new();
        flat.insert("tags".to_string(), r#"["a","b"]"#.to_string());
        let result = unflatten_dict(&flat, ".");
        assert!(result["tags"].is_array());
    }

    #[test]
    fn test_try_parse_number_int() {
        assert_eq!(try_parse_number("42"), Value::Number(42.into()));
    }

    #[test]
    fn test_try_parse_number_float() {
        let v = try_parse_number("3.14");
        assert!(v.is_number());
    }

    #[test]
    fn test_try_parse_number_string() {
        assert_eq!(
            try_parse_number("hello"),
            Value::String("hello".to_string())
        );
    }

    #[test]
    fn test_csv_to_json_basic() {
        let csv_file = create_temp_csv("name,age\nAlice,30\nBob,25\n");
        let json_out = NamedTempFile::new().unwrap();
        let out_path = json_out.path().to_str().unwrap().to_string();

        csv_to_json(
            csv_file.path().to_str().unwrap(),
            Some(&out_path),
            b',',
            "utf-8",
        )
        .unwrap();

        let content = fs::read_to_string(&out_path).unwrap();
        let data: Vec<Value> = serde_json::from_str(&content).unwrap();
        assert_eq!(data.len(), 2);
        assert_eq!(data[0]["name"], "Alice");
        assert_eq!(data[0]["age"], 30);
    }

    #[test]
    fn test_json_to_csv_basic() {
        let json_file = create_temp_json(r#"[{"name":"Alice","age":30},{"name":"Bob","age":25}]"#);
        let csv_out = NamedTempFile::new().unwrap();
        let out_path = csv_out.path().to_str().unwrap().to_string();

        json_to_csv(
            json_file.path().to_str().unwrap(),
            Some(&out_path),
            b',',
            "utf-8",
        )
        .unwrap();

        let content = fs::read_to_string(&out_path).unwrap();
        assert!(content.contains("name"));
        assert!(content.contains("Alice"));
    }

    #[test]
    fn test_json_to_csv_nested() {
        let json_file =
            create_temp_json(r#"[{"name":"Alice","address":{"city":"Beijing","zip":"100000"}}]"#);
        let csv_out = NamedTempFile::new().unwrap();
        let out_path = csv_out.path().to_str().unwrap().to_string();

        json_to_csv(
            json_file.path().to_str().unwrap(),
            Some(&out_path),
            b',',
            "utf-8",
        )
        .unwrap();

        let content = fs::read_to_string(&out_path).unwrap();
        assert!(content.contains("address.city"));
        assert!(content.contains("Beijing"));
    }

    #[test]
    fn test_csv_to_json_file_not_found() {
        let result = csv_to_json("/nonexistent/file.csv", None, b',', "utf-8");
        assert!(result.is_err());
    }

    #[test]
    fn test_json_to_csv_not_array() {
        let json_file = create_temp_json(r#"{"key": "value"}"#);
        let result = json_to_csv(json_file.path().to_str().unwrap(), None, b',', "utf-8");
        assert!(result.is_err());
    }

    #[test]
    fn test_json_to_csv_empty_array() {
        let json_file = create_temp_json("[]");
        let result = json_to_csv(json_file.path().to_str().unwrap(), None, b',', "utf-8");
        assert!(result.is_err());
    }

    #[test]
    fn test_csv_to_json_custom_delimiter() {
        let csv_file = create_temp_csv("name\tage\nAlice\t30\n");
        let json_out = NamedTempFile::new().unwrap();
        let out_path = json_out.path().to_str().unwrap().to_string();

        csv_to_json(
            csv_file.path().to_str().unwrap(),
            Some(&out_path),
            b'\t',
            "utf-8",
        )
        .unwrap();

        let content = fs::read_to_string(&out_path).unwrap();
        let data: Vec<Value> = serde_json::from_str(&content).unwrap();
        assert_eq!(data[0]["name"], "Alice");
    }
}
