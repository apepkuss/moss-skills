#!/usr/bin/env python3
"""
CSV <-> JSON 数据格式转换工具

Usage:
    python convert.py csv2json <input.csv> [output.json] [--delimiter=,] [--encoding=utf-8]
    python convert.py json2csv <input.json> [output.csv] [--delimiter=,] [--encoding=utf-8]
"""

import argparse
import csv
import json
import sys
from pathlib import Path
from typing import Any


def flatten_dict(d: dict, parent_key: str = "", sep: str = ".") -> dict:
    """展平嵌套字典"""
    items = []
    for k, v in d.items():
        new_key = f"{parent_key}{sep}{k}" if parent_key else k
        if isinstance(v, dict):
            items.extend(flatten_dict(v, new_key, sep).items())
        elif isinstance(v, list):
            # 将列表转换为 JSON 字符串
            items.append((new_key, json.dumps(v, ensure_ascii=False)))
        else:
            items.append((new_key, v))
    return dict(items)


def unflatten_dict(d: dict, sep: str = ".") -> dict:
    """还原展平的字典"""
    result = {}
    for key, value in d.items():
        parts = key.split(sep)
        current = result
        for part in parts[:-1]:
            if part not in current:
                current[part] = {}
            current = current[part]

        # 尝试解析 JSON 字符串（可能是数组）
        if isinstance(value, str):
            try:
                if value.startswith("[") or value.startswith("{"):
                    value = json.loads(value)
            except json.JSONDecodeError:
                pass

        current[parts[-1]] = value
    return result


def csv_to_json(
    input_path: str,
    output_path: str | None = None,
    delimiter: str = ",",
    encoding: str = "utf-8",
) -> list[dict]:
    """将 CSV 文件转换为 JSON"""
    input_file = Path(input_path)

    if not input_file.exists():
        raise FileNotFoundError(f"文件不存在: {input_path}")

    with open(input_file, "r", encoding=encoding, newline="") as f:
        reader = csv.DictReader(f, delimiter=delimiter)
        data = list(reader)

    # 转换数值类型
    for row in data:
        for key, value in row.items():
            if value is None:
                continue
            # 尝试转换为数字
            try:
                if "." in value:
                    row[key] = float(value)
                else:
                    row[key] = int(value)
            except ValueError:
                # 保持字符串
                pass

    if output_path:
        output_file = Path(output_path)
        with open(output_file, "w", encoding=encoding) as f:
            json.dump(data, f, ensure_ascii=False, indent=2)
        print(f"✓ 已转换: {input_path} -> {output_path}", file=sys.stderr)
        print(f"  共 {len(data)} 条记录", file=sys.stderr)
    else:
        # 输出到标准输出
        print(json.dumps(data, ensure_ascii=False, indent=2))

    return data


def json_to_csv(
    input_path: str,
    output_path: str | None = None,
    delimiter: str = ",",
    encoding: str = "utf-8",
) -> list[dict]:
    """将 JSON 文件转换为 CSV"""
    input_file = Path(input_path)

    if not input_file.exists():
        raise FileNotFoundError(f"文件不存在: {input_path}")

    with open(input_file, "r", encoding=encoding) as f:
        data = json.load(f)

    if not isinstance(data, list):
        raise ValueError("JSON 必须是数组格式")

    if len(data) == 0:
        raise ValueError("JSON 数组为空")

    # 展平嵌套结构
    flattened_data = [flatten_dict(row) if isinstance(row, dict) else {"value": row} for row in data]

    # 收集所有可能的字段名
    fieldnames = []
    for row in flattened_data:
        for key in row.keys():
            if key not in fieldnames:
                fieldnames.append(key)

    if output_path:
        output_file = Path(output_path)
        with open(output_file, "w", encoding=encoding, newline="") as f:
            writer = csv.DictWriter(f, fieldnames=fieldnames, delimiter=delimiter)
            writer.writeheader()
            writer.writerows(flattened_data)
        print(f"✓ 已转换: {input_path} -> {output_path}", file=sys.stderr)
        print(f"  共 {len(data)} 条记录, {len(fieldnames)} 列", file=sys.stderr)
    else:
        # 输出到标准输出
        writer = csv.DictWriter(sys.stdout, fieldnames=fieldnames, delimiter=delimiter)
        writer.writeheader()
        writer.writerows(flattened_data)

    return flattened_data


def main():
    parser = argparse.ArgumentParser(
        description="CSV <-> JSON 数据格式转换工具",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
示例:
  %(prog)s csv2json data.csv data.json
  %(prog)s csv2json data.csv --delimiter=";"
  %(prog)s json2csv users.json users.csv
  %(prog)s json2csv data.json --delimiter=$'\\t'
        """,
    )

    subparsers = parser.add_subparsers(dest="command", help="转换命令")

    # csv2json 子命令
    csv2json_parser = subparsers.add_parser("csv2json", help="CSV 转 JSON")
    csv2json_parser.add_argument("input", help="输入 CSV 文件路径")
    csv2json_parser.add_argument("output", nargs="?", help="输出 JSON 文件路径（可选）")
    csv2json_parser.add_argument("--delimiter", "-d", default=",", help="CSV 分隔符（默认逗号）")
    csv2json_parser.add_argument("--encoding", "-e", default="utf-8", help="文件编码（默认 UTF-8）")

    # json2csv 子命令
    json2csv_parser = subparsers.add_parser("json2csv", help="JSON 转 CSV")
    json2csv_parser.add_argument("input", help="输入 JSON 文件路径")
    json2csv_parser.add_argument("output", nargs="?", help="输出 CSV 文件路径（可选）")
    json2csv_parser.add_argument("--delimiter", "-d", default=",", help="CSV 分隔符（默认逗号）")
    json2csv_parser.add_argument("--encoding", "-e", default="utf-8", help="文件编码（默认 UTF-8）")

    args = parser.parse_args()

    if not args.command:
        parser.print_help()
        sys.exit(1)

    try:
        if args.command == "csv2json":
            csv_to_json(args.input, args.output, args.delimiter, args.encoding)
        elif args.command == "json2csv":
            json_to_csv(args.input, args.output, args.delimiter, args.encoding)
    except FileNotFoundError as e:
        print(f"错误: {e}", file=sys.stderr)
        sys.exit(1)
    except json.JSONDecodeError as e:
        print(f"JSON 解析错误: {e}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"错误: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
