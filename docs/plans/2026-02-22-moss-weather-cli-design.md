# moss-weather-cli 设计文档

**日期：** 2026-02-22
**状态：** 已审批

## 背景

`cardea-weather` 是一个 Rust 编写的 MCP 服务器，通过 OpenWeatherMap API 提供天气查询功能。本项目基于同样的 API 逻辑，构建一个独立的命令行工具 `moss-weather-cli`，供用户直接在终端查询天气，无需启动 MCP 服务器。

## 目标

在 `/Volumes/Dev/secondstate/me/skills/moss-weather-cli/` 创建一个 Rust CLI 二进制，支持按城市名查询当前天气，支持摄氏度和华氏度切换。

## 方案选择

采用**方案 A：独立 Rust CLI 二进制**。

- 自包含，无外部服务依赖
- 直接复用 cardea-weather 的 OpenWeatherMap 两步请求逻辑
- 可直接分发二进制

## 项目结构

```
skills/moss-weather-cli/
├── Cargo.toml
└── src/
    ├── main.rs      # CLI 入口、参数解析、输出
    ├── weather.rs   # OpenWeatherMap API 调用与数据格式化
    └── types.rs     # 请求/响应数据结构定义
```

## CLI 接口

```bash
# 基本用法（默认摄氏度）
moss-weather "Beijing"

# 指定华氏度
moss-weather "New York" --unit fahrenheit
moss-weather "New York" -u fahrenheit

# 帮助
moss-weather --help
```

参数：
- `CITY`：位置参数，城市名（必需）
- `--unit / -u`：温度单位，`celsius`（默认）或 `fahrenheit`
- `OPENWEATHERMAP_API_KEY`：从环境变量读取，未设置时打印友好错误后退出

## 数据流

```
User → main.rs (clap 解析参数)
  → weather.rs::get_weather(location, unit)
    → OWM Geocoding API (city → lat/lon)
    → OWM Weather API (lat/lon → weather data)
    → format_weather_info() → 格式化字符串
  → 打印到 stdout
```

## 依赖

| crate | 版本 | 用途 |
|---|---|---|
| `clap` | 4.x (derive) | CLI 参数解析 |
| `reqwest` | 0.12 (json) | HTTP 客户端 |
| `tokio` | 1.x (full) | 异步运行时 |
| `serde` + `serde_json` | 1.0 | JSON 反序列化 |
| `chrono` | 0.4 | 日出/日落时间格式化 |
| `anyhow` | 1.x | 统一错误处理 |

## 错误处理

使用 `anyhow::Result` 统一错误传播，在 `main` 中捕获并打印到 stderr，以非零退出码退出。常见错误场景：

- 未设置 `OPENWEATHERMAP_API_KEY` → 打印提示信息
- 城市名无法找到 → 打印找不到城市的提示
- 网络错误 → 打印网络连接错误
