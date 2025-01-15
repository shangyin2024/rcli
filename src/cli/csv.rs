use super::verify_file;
use core::fmt;
use std::str::FromStr;

use clap::Args;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    // Toml,
}

#[derive(Args, Debug)]
pub struct CsvOpts {
    #[arg(short, long,value_parser = verify_file)]
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(long,value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

// 将字符串解析为 OutputFormat 枚举
// 这个函数实际上是调用了 FromStr trait 的实现
fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse() // 直接调用 parse() 方法，这会使用下面的 FromStr 实现
}

// 实现 From trait，用于将 OutputFormat 转换为字符串字面量
// 这允许我们将 OutputFormat 枚举值转换为对应的字符串表示
impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json", // 如果是 Json 格式，返回 "json"
            OutputFormat::Yaml => "yaml", // 如果是 Yaml 格式，返回 "yaml"
                                           // OutputFormat::Toml => "toml",   // 如果是 Toml 格式，返回 "toml"
        }
    }
}

// 实现 FromStr trait，允许从字符串解析为 OutputFormat
// 这使得我们可以使用 str.parse() 方法来创建 OutputFormat
impl FromStr for OutputFormat {
    type Err = anyhow::Error; // 使用 anyhow::Error 作为错误类型

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json), // 将 "json" 字符串转换为 Json 枚举值
            "yaml" => Ok(OutputFormat::Yaml), // 将 "yaml" 字符串转换为 Yaml 枚举值
            // "toml" => Ok(OutputFormat::Toml),   // 将 "toml" 字符串转换为 Toml 枚举值
            _ => Err(anyhow::anyhow!("Invalid format: {}", s)), // 其他情况返回错误
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
