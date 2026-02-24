# Skills for MOSS

## `moss-weather` Skill

一个使用 Rust 编写的命令行天气查询工具，作为 MOSS AI 系统的 Skill 运行。当用户询问天气、气温、是否下雨等问题时，MOSS 会自动调用此工具获取实时数据并给出自然语言回复。

### moss-weather 项目结构

```text
moss-weather/
└── SKILL.md                         # Skill 定义文档（随发布包分发）

moss-weather-cli/                    # 源代码
├── Cargo.toml
├── Cargo.lock
└── src/
    ├── main.rs                      # CLI 入口（clap 参数解析）
    ├── types.rs                     # 数据结构定义
    └── weather.rs                   # 天气 API 调用与格式化
```

### moss-weather 功能特性

- 查询全球任意城市的实时天气（基于 [OpenWeatherMap API](https://openweathermap.org/)）
- 支持摄氏度 / 华氏度切换
- 自动地理编码（城市名 → 经纬度）
- 返回温度、体感温度、湿度、气压、风速风向、云量、能见度、日出日落、降水量等完整数据

### moss-weather 使用方法

```bash
# 前置条件：设置 API Key
export OPENWEATHERMAP_API_KEY="your-api-key"

# 查询北京天气（默认摄氏度）
<skill-base-dir>/scripts/moss-weather "Beijing"

# 查询纽约天气（华氏度）
<skill-base-dir>/scripts/moss-weather "New York" --unit fahrenheit
```

| 参数 | 类型 | 必需 | 说明 |
| --- | --- | --- | --- |
| `CITY` | string | 是 | 城市名称（英文），如 `Beijing`、`New York`、`Paris,FR` |
| `--unit` / `-u` | string | 否 | 温度单位：`celsius`（默认）或 `fahrenheit` |

### moss-weather 输出示例

```text
Weather for Beijing, CN
Condition  : Clear (clear sky)
Temperature: 20.0°C (feels like 19.0°C)
Range      : 17.0°C ~ 23.0°C
Humidity   : 50%
Pressure   : 1013 hPa
Wind       : 3.0 m/s S (180°)
Clouds     : 10%
Visibility : 10.0 km
Sunrise    : 06:23  Sunset: 17:21
```

---

## `moss-convert` Skill

一个使用 Rust 编写的 CSV / JSON 格式互转命令行工具，作为 MOSS AI 系统的 Skill 运行。当用户需要转换数据格式、处理 CSV/JSON 文件时，MOSS 会自动调用此工具完成转换。

### moss-convert 项目结构

```text
moss-convert/
└── SKILL.md                         # Skill 定义文档（随发布包分发）

moss-convert-cli/                    # 源代码
├── Cargo.toml
├── Cargo.lock
└── src/
    ├── main.rs                      # CLI 入口（clap 参数解析）
    └── convert.rs                   # CSV/JSON 转换核心逻辑
```

### moss-convert 功能特性

- CSV 转 JSON：自动识别数值类型，输出格式化 JSON
- JSON 转 CSV：支持嵌套对象自动展平（以 `.` 分隔），数组字段保留为 JSON 字符串
- 自定义分隔符（逗号、Tab 等）
- 支持输出到文件或标准输出

### moss-convert 使用方法

```bash
# CSV 转 JSON
<skill-base-dir>/scripts/moss-convert csv2json /path/to/data.csv /path/to/data.json

# JSON 转 CSV
<skill-base-dir>/scripts/moss-convert json2csv /path/to/data.json /path/to/data.csv

# 使用 Tab 分隔符
<skill-base-dir>/scripts/moss-convert csv2json /path/to/data.tsv /path/to/data.json --delimiter "\t"

# 输出到标准输出
<skill-base-dir>/scripts/moss-convert csv2json /path/to/data.csv
```

| 参数 | 类型 | 必需 | 说明 |
| --- | --- | --- | --- |
| `command` | string | 是 | 转换命令：`csv2json` 或 `json2csv` |
| `input` | string | 是 | 输入文件的绝对路径 |
| `output` | string | 否 | 输出文件的绝对路径（省略则输出到标准输出） |
| `--delimiter` / `-d` | string | 否 | CSV 分隔符（默认逗号） |
| `--encoding` / `-e` | string | 否 | 文件编码（默认 UTF-8） |

---

## 构建与发布

通过 GitHub Actions 自动构建，推送 `v*` 标签触发。支持以下平台：

| 平台                | Target                       |
| ------------------- | ---------------------------- |
| macOS Apple Silicon | `aarch64-apple-darwin`       |
| Linux x86_64        | `x86_64-unknown-linux-gnu`   |
| Linux ARM64         | `aarch64-unknown-linux-gnu`  |

发布产物包含 `SKILL.md` 和编译好的二进制文件：

- `moss-weather-{VERSION}-{TARGET}.tar.gz`
- `moss-convert-{VERSION}-{TARGET}.tar.gz`
