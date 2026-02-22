---
name: data-convert
description: |
  CSV 和 JSON 数据格式互转工具。
  支持 CSV 转 JSON、JSON 转 CSV，可处理嵌套结构和自定义分隔符。
  当用户需要转换数据格式、处理 CSV/JSON 文件时使用此 skill。
  此 skill 只需文件绝对路径作为输入，内部自行读取和处理文件，无需预先读取文件内容。
---

# Data Convert Skill

CSV 与 JSON 格式互转工具，通过 `convert.py` 脚本实现。

## 脚本参数格式

脚本使用**位置参数**（非 flag 参数），格式如下：

```
convert.py <command> <input_path> [output_path] [--delimiter=,]
```

- `command`: 转换命令，`csv2json` 或 `json2csv`（必需）
- `input_path`: 输入文件的**绝对路径**（必需）
- `output_path`: 输出文件的**绝对路径**（可选，省略则输出到标准输出）
- `--delimiter`: CSV 分隔符（可选，默认逗号）

**输出路径推导规则**：输出文件应与输入文件在同一目录，仅更换扩展名。例如：
- `/Users/sam/Desktop/data.csv` → `/Users/sam/Desktop/data.json`
- `/home/user/reports/users.json` → `/home/user/reports/users.csv`

## 调用参数示例

`args` 数组的元素对应上述位置参数：

**CSV 转 JSON：**
```
args: ["csv2json", "/Users/sam/Desktop/data.csv", "/Users/sam/Desktop/data.json"]
```

**JSON 转 CSV：**
```
args: ["json2csv", "/home/user/data.json", "/home/user/data.csv"]
```

**使用自定义分隔符：**
```
args: ["csv2json", "/Users/sam/data.tsv", "/Users/sam/data.json", "--delimiter=\t"]
```

**输出到标准输出（省略 output_path）：**
```
args: ["csv2json", "/Users/sam/data.csv"]
```

## 执行步骤

1. 从用户消息或附件信息中获取输入文件的**绝对路径**
2. 根据输入文件扩展名确定转换方向（`.csv` → csv2json，`.json` → json2csv）
3. 基于输入路径推导输出路径（同目录，换扩展名）
4. 调用 `convert.py` 脚本执行转换

## 数据格式

### CSV
- 第一行为表头（列名）
- 支持带引号的字段和自定义分隔符

### JSON
- 必须是对象数组 `[{...}, {...}]`
- 嵌套对象会自动展平（如 `address.city`）
- 数组字段转为 JSON 字符串

## 错误处理

| 错误情况 | 处理方式 |
|----------|----------|
| 文件不存在 | 检查路径是否为绝对路径 |
| JSON 格式错误 | 显示具体解析错误位置 |
| CSV 格式错误 | 显示问题行号 |
