# Skills for MOSS

## `moss-weather` Skill

一个使用 Rust 编写的命令行天气查询工具，作为 MOSS AI 系统的 Skill 运行。当用户询问天气、气温、是否下雨等问题时，MOSS 会自动调用此工具获取实时数据并给出自然语言回复。

### 项目结构

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

### 功能特性

- 查询全球任意城市的实时天气（基于 [OpenWeatherMap API](https://openweathermap.org/)）
- 支持摄氏度 / 华氏度切换
- 自动地理编码（城市名 → 经纬度）
- 返回温度、体感温度、湿度、气压、风速风向、云量、能见度、日出日落、降水量等完整数据

### 使用方法

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

### 输出示例

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

### 技术实现

- **API 数据源**：OpenWeatherMap — 地理编码 API (`/geo/1.0/direct`) + 当前天气 API (`/data/2.5/weather`)
- **依赖库**：`clap`（CLI 解析）、`reqwest`（HTTP）、`tokio`（异步运行时）、`serde`（JSON 序列化）、`chrono`（时间处理）、`anyhow`（错误处理）
- **URL 编码**：自定义实现，支持非 ASCII 字符（如中文城市名）
- **风向转换**：角度 → 16 方位（N/NE/E/SE/S/SW/W/NW）

### 构建与发布

通过 GitHub Actions 自动构建，推送 `v*` 标签触发。支持以下平台：

| 平台                | Target                       |
| ------------------- | ---------------------------- |
| macOS Apple Silicon | `aarch64-apple-darwin`       |
| Linux x86_64        | `x86_64-unknown-linux-gnu`   |
| Linux ARM64         | `aarch64-unknown-linux-gnu`  |

发布产物为 `moss-weather-{VERSION}-{TARGET}.tar.gz`，包含 `SKILL.md` 和编译好的二进制文件。
