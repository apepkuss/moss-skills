---
name: cardea-weather
description: |
  调用 Cardea Weather MCP Server 获取城市天气信息。
  支持全球城市的实时天气查询，包括温度、湿度、风速、日出日落等信息。
  当用户询问天气、气温、是否下雨、需要穿什么衣服等问题时使用此 skill。
allowed-tools: mcp__cardea-weather__get_current_weather
---

# Cardea Weather MCP Skill

通过 MCP 协议调用 Cardea Weather 服务获取实时天气信息。

## 可用工具

| 工具名 | 功能 |
|--------|------|
| `mcp__cardea-weather__get_current_weather` | 获取指定城市的当前天气 |

## 参数说明

### get_current_weather

| 参数 | 类型 | 必需 | 说明 |
|------|------|------|------|
| `location` | string | 是 | 城市名称，如 "Beijing"、"New York"、"Tokyo" |
| `unit` | string | 否 | 温度单位："celsius"（摄氏度，默认）或 "fahrenheit"（华氏度） |

## 使用指令

当用户询问天气相关问题时，调用 `mcp__cardea-weather__get_current_weather` 工具。

### 基本查询

```
工具: mcp__cardea-weather__get_current_weather
参数:
  location: "城市名（英文）"
  unit: "celsius"  # 可选，默认摄氏度
```

### 示例调用

**查询北京天气（摄氏度）：**
```
mcp__cardea-weather__get_current_weather(location="Beijing")
```

**查询纽约天气（华氏度）：**
```
mcp__cardea-weather__get_current_weather(location="New York", unit="fahrenheit")
```

## 返回信息

工具返回的天气信息包括：

| 类别 | 包含内容 |
|------|----------|
| **位置** | 城市名、国家代码、经纬度、时区 |
| **天气状况** | 主要状况（晴/雨/阴等）、详细描述 |
| **温度** | 当前温度、体感温度、最高/最低温度 |
| **大气** | 气压（地面/海平面）、湿度百分比 |
| **风** | 风速(m/s)、风向(度)、阵风速度 |
| **其他** | 云量百分比、能见度(米) |
| **时间** | 日出时间、日落时间（当地时区） |
| **降水** | 降雨量、降雪量（如有） |

## 城市名称格式

使用英文城市名，支持以下格式：

| 格式 | 示例 |
|------|------|
| 城市名 | `Beijing`, `Tokyo`, `London` |
| 城市,国家 | `Paris,FR`, `Sydney,AU` |
| 城市,州,国家 | `Los Angeles,CA,US` |

### 常用城市对照

| 中文 | 英文 |
|------|------|
| 北京 | Beijing |
| 上海 | Shanghai |
| 广州 | Guangzhou |
| 深圳 | Shenzhen |
| 香港 | Hong Kong |
| 台北 | Taipei |
| 东京 | Tokyo |
| 首尔 | Seoul |
| 新加坡 | Singapore |
| 纽约 | New York |
| 洛杉矶 | Los Angeles |
| 伦敦 | London |
| 巴黎 | Paris |

## 执行流程

1. **解析用户意图**：识别用户想查询哪个城市的天气
2. **转换城市名**：如果用户用中文，转换为对应的英文名
3. **确定温度单位**：
   - 中国用户默认使用摄氏度
   - 美国用户可能偏好华氏度
   - 或根据用户明确要求选择
4. **调用工具**：使用正确的参数调用 MCP 工具
5. **格式化输出**：将返回的天气信息以友好方式呈现给用户

## 回复建议

根据天气情况，可以额外提供建议：

| 天气状况 | 建议 |
|----------|------|
| 高温 (>35°C) | 注意防暑，多喝水 |
| 低温 (<5°C) | 注意保暖，穿厚外套 |
| 下雨 | 记得带伞 |
| 大风 (>10m/s) | 注意防风，固定室外物品 |
| 高湿度 (>80%) | 天气闷热，注意通风 |
| 低能见度 (<1000m) | 开车注意安全 |

## 错误处理

| 错误情况 | 处理方式 |
|----------|----------|
| 城市名无法识别 | 提示用户检查拼写，或提供正确的英文名 |
| MCP 服务未连接 | 提示用户检查服务是否运行 |
| API 限流 | 稍后重试 |

## 使用示例

用户可以这样询问：

- "北京今天天气怎么样？"
- "东京现在多少度？"
- "纽约会下雨吗？"
- "我去伦敦需要带伞吗？"
- "深圳明天适合户外运动吗？"
- "查一下上海的天气"

## 服务信息

| 属性 | 值 |
|------|-----|
| 服务名称 | cardea-weather |
| 版本 | 0.10.0 |
| 默认端口 | 8002 |
| 数据源 | OpenWeatherMap API |
| 传输协议 | Streamable HTTP |
