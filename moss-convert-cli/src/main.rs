mod convert;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "moss-convert",
    about = "CSV <-> JSON 数据格式转换工具",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// CSV 转 JSON
    Csv2json {
        /// 输入 CSV 文件路径
        input: String,

        /// 输出 JSON 文件路径（可选，省略则输出到标准输出）
        output: Option<String>,

        /// CSV 分隔符（默认逗号）
        #[arg(short, long, default_value = ",")]
        delimiter: String,

        /// 文件编码（默认 UTF-8）
        #[arg(short, long, default_value = "utf-8")]
        encoding: String,
    },
    /// JSON 转 CSV
    Json2csv {
        /// 输入 JSON 文件路径
        input: String,

        /// 输出 CSV 文件路径（可选，省略则输出到标准输出）
        output: Option<String>,

        /// CSV 分隔符（默认逗号）
        #[arg(short, long, default_value = ",")]
        delimiter: String,

        /// 文件编码（默认 UTF-8）
        #[arg(short, long, default_value = "utf-8")]
        encoding: String,
    },
}

fn parse_delimiter(s: &str) -> u8 {
    match s {
        "\\t" | "\t" => b'\t',
        s if s.len() == 1 => s.as_bytes()[0],
        _ => b',',
    }
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Csv2json {
            input,
            output,
            delimiter,
            encoding,
        } => {
            let delim = parse_delimiter(&delimiter);
            convert::csv_to_json(&input, output.as_deref(), delim, &encoding)
        }
        Commands::Json2csv {
            input,
            output,
            delimiter,
            encoding,
        } => {
            let delim = parse_delimiter(&delimiter);
            convert::json_to_csv(&input, output.as_deref(), delim, &encoding)
        }
    };

    if let Err(e) = result {
        eprintln!("错误: {}", e);
        std::process::exit(1);
    }
}
