---
name: moss-weather
description: |
  通过 moss-weather CLI 工具查询城市当前天气信息。
  支持全球城市的实时天气查询，包括温度、湿度、风速、日出日落等信息。
  当用户询问天气、气温、是否下雨、需要穿什么衣服等问题时使用此 skill。
  需要设置 OPENWEATHERMAP_API_KEY 环境变量。
---

# Moss Weather Skill

通过执行 `moss-weather` CLI 二进制文件获取实时天气信息。

## 命令格式

```bash
<skill-base-dir>/scripts/moss-weather "<CITY>" [--unit celsius|fahrenheit]
```

> `<skill-base-dir>` 为 skill 加载时提供的 base directory 绝对路径。

| 参数 | 类型 | 必需 | 说明 |
|------|------|------|------|
| `CITY` | string | 是 | 城市名称（英文），如 "Beijing"、"New York" |
| `--unit / -u` | string | 否 | 温度单位："celsius"（默认）或 "fahrenheit" |

## 执行步骤

1. **解析用户意图**：识别用户想查询哪个城市的天气
2. **转换城市名**：如果用户用中文，转换为对应英文名
3. **确定温度单位**：中文语境默认摄氏度；用户明确要求时切换华氏度
4. **执行命令**：使用 Bash 工具，以 skill base directory 拼接完整路径执行二进制文件
5. **格式化回复**：结合天气数据给出自然语言回复和生活建议

## 调用示例

**查询北京天气（摄氏度）：**
```bash
<skill-base-dir>/scripts/moss-weather "Beijing"
```

**查询纽约天气（华氏度）：**
```bash
<skill-base-dir>/scripts/moss-weather "New York" --unit fahrenheit
```

## 返回信息

命令输出包含以下天气数据：

| 类别 | 包含内容 |
|------|----------|
| **位置** | 城市名、国家代码 |
| **天气状况** | 主要状况（晴/雨/阴等）、详细描述 |
| **温度** | 当前温度、体感温度、最高/最低温度 |
| **大气** | 气压（hPa）、湿度百分比 |
| **风** | 风速(m/s)、风向、阵风速度 |
| **其他** | 云量百分比、能见度(km) |
| **时间** | 日出时间、日落时间（当地时区） |
| **降水** | 降雨量、降雪量（如有） |

## 城市名称格式

使用英文城市名，支持以下格式：

| 格式 | 示例 |
|------|------|
| 城市名 | `Beijing`, `Tokyo`, `London` |
| 城市,国家 | `Paris,FR`, `Sydney,AU` |

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

## 回复建议

根据天气情况，可额外提供生活建议：

| 天气状况 | 建议 |
|----------|------|
| 高温 (>35°C) | 注意防暑，多喝水 |
| 低温 (<5°C) | 注意保暖，穿厚外套 |
| 下雨 | 记得带伞 |
| 大风 (>10 m/s) | 注意防风 |
| 高湿度 (>80%) | 天气闷热，注意通风 |
| 低能见度 (<1 km) | 开车注意安全 |

## 错误处理

| 错误情况 | 处理方式 |
|----------|----------|
| 未设置 `OPENWEATHERMAP_API_KEY` | 提示用户设置环境变量，申请地址：https://openweathermap.org/ |
| 城市名无法识别 | 提示用户检查拼写或提供英文城市名 |
| 网络连接失败 | 提示用户检查网络连接后重试 |
