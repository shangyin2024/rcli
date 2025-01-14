use anyhow::Result;
use csv::Reader;
use serde_json::Value;
use std::fs;

use crate::opts::OutputFormat;

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = vec![];
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;

        // 1. headers.iter() 和 record.iter() 分别创建标题行和数据行的迭代器
        // 2. zip() 将两个迭代器配对组合，生成 (header, value) 的键值对
        // 3. collect::<Value>() 将键值对集合转换为 serde_json::Value 类型
        // 这样就把 CSV 的一行数据转换成了 JSON 对象，其中列名作为键，对应的数据作为值
        let value = headers.iter().zip(record.iter()).collect::<Value>();

        ret.push(value);
    }
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
        // OutputFormat::Toml => toml::to_string(&ret)?,
    };
    fs::write(output, content)?;
    Ok(())
}
