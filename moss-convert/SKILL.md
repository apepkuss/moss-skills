---
name: moss-convert
description: |
  CSV 和 JSON 数据格式互转工具。
  支持 CSV 转 JSON、JSON 转 CSV，可处理嵌套结构和自定义分隔符。
  当用户需要转换数据格式、处理 CSV/JSON 文件时使用此 skill。
  此 skill 只需文件绝对路径作为输入，内部自行读取和处理文件，无需预先读取文件内容。
---

# Moss Convert Skill

通过执行 `moss-convert` CLI 二进制文件实现 CSV 与 JSON 格式互转。

## 命令格式

```bash
<skill-base-dir>/scripts/moss-convert <command> <input_path> [output_path] [--delimiter <sep>] [--encoding <enc>]
```

> `<skill-base-dir>` 为 skill 加载时提供的 base directory 绝对路径。

| 参数 | 类型 | 必需 | 说明 |
|------|------|------|------|
| `command` | string | 是 | 转换命令：`csv2json` 或 `json2csv` |
| `input_path` | string | 是 | 输入文件的绝对路径 |
| `output_path` | string | 否 | 输出文件的绝对路径（省略则输出到标准输出） |
| `--delimiter / -d` | string | 否 | CSV 分隔符（默认逗号） |
| `--encoding / -e` | string | 否 | 文件编码（默认 UTF-8） |

## 执行步骤

1. **解析用户意图**：识别用户需要的转换方向（CSV→JSON 或 JSON→CSV）
2. **获取文件路径**：从用户消息或附件信息中获取输入文件的**绝对路径**
3. **确定转换方向**：根据输入文件扩展名确定转换命令（`.csv` → csv2json，`.json` → json2csv）
4. **推导输出路径**：输出文件应与输入文件在同一目录，仅更换扩展名
5. **执行命令**：使用 Bash 工具，以 skill base directory 拼接完整路径执行二进制文件
6. **报告结果**：告知用户转换结果和输出文件位置

## 调用示例

**CSV 转 JSON：**
```bash
<skill-base-dir>/scripts/moss-convert csv2json /Users/sam/Desktop/data.csv /Users/sam/Desktop/data.json
```

**JSON 转 CSV：**
```bash
<skill-base-dir>/scripts/moss-convert json2csv /home/user/data.json /home/user/data.csv
```

**使用自定义分隔符（Tab 分隔）：**
```bash
<skill-base-dir>/scripts/moss-convert csv2json /Users/sam/data.tsv /Users/sam/data.json --delimiter "\t"
```

**输出到标准输出（省略 output_path）：**
```bash
<skill-base-dir>/scripts/moss-convert csv2json /Users/sam/data.csv
```

## 输出路径推导规则

输出文件应与输入文件在同一目录，仅更换扩展名：

| 输入路径 | 输出路径 |
|----------|----------|
| `/Users/sam/Desktop/data.csv` | `/Users/sam/Desktop/data.json` |
| `/home/user/reports/users.json` | `/home/user/reports/users.csv` |

## 数据格式

### CSV
- 第一行为表头（列名）
- 支持带引号的字段和自定义分隔符
- 数值字段自动转换为 JSON 数字类型

### JSON
- 必须是对象数组 `[{...}, {...}]`
- 嵌套对象会自动展平（如 `address.city`）
- 数组字段转为 JSON 字符串保留

## 错误处理

| 错误情况 | 处理方式 |
|----------|----------|
| 文件不存在 | 检查路径是否为绝对路径 |
| JSON 格式错误 | 显示具体解析错误信息 |
| JSON 非数组格式 | 提示用户 JSON 必须是对象数组 |
| JSON 数组为空 | 提示用户数组不能为空 |
| 编码错误 | 提示用户检查文件编码设置 |
