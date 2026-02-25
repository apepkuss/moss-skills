---
name: moss-youtube-captions
description: 获取 YouTube 视频的字幕/转录文本。支持标准 URL 和短链接格式。当用户需要总结视频内容、回答视频相关问题、提取视频信息时使用此 skill。需要安装 yt-dlp。
metadata: {"moss":{"requires":{"bins":["yt-dlp"]},"install":[{"id":"brew","kind":"brew","formula":"yt-dlp","bins":["yt-dlp"],"label":"Install yt-dlp (brew)"},{"id":"pip","kind":"pip","package":"yt-dlp","bins":["yt-dlp"],"label":"Install yt-dlp (pip)"}]}}
---

# Moss YouTube Captions Skill

通过执行 `moss-youtube-captions` CLI 二进制文件获取 YouTube 视频字幕文本。

## 命令格式

```bash
<skill-base-dir>/scripts/moss-youtube-captions "<YOUTUBE_URL>"
```

> `<skill-base-dir>` 为 skill 加载时提供的 base directory 绝对路径。

| 参数 | 类型 | 必需 | 说明 |
|------|------|------|------|
| `YOUTUBE_URL` | string | 是 | YouTube 视频 URL |

### 支持的 URL 格式

| 格式 | 示例 |
|------|------|
| 标准链接 | `https://www.youtube.com/watch?v=dQw4w9WgXcQ` |
| 带额外参数 | `https://www.youtube.com/watch?v=dQw4w9WgXcQ&t=120` |
| 短链接 | `https://youtu.be/dQw4w9WgXcQ` |

## 执行步骤

1. **解析用户意图**：识别用户想获取哪个 YouTube 视频的内容
2. **获取视频 URL**：从用户消息中提取 YouTube 视频链接
3. **执行命令**：使用 Bash 工具，以 skill base directory 拼接完整路径执行二进制文件
4. **处理输出**：命令会将纯文本字幕输出到标准输出
5. **格式化回复**：根据用户需求对字幕内容进行总结、回答问题或提取信息

## 调用示例

**获取视频字幕：**
```bash
<skill-base-dir>/scripts/moss-youtube-captions "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
```

**使用短链接：**
```bash
<skill-base-dir>/scripts/moss-youtube-captions "https://youtu.be/dQw4w9WgXcQ"
```

## 返回信息

命令将视频的英文字幕以纯文本形式输出到标准输出，已清理 VTT 格式标记（时间戳、HTML 标签等），并去除连续重复行。

## 常见使用场景

| 场景 | 处理方式 |
|------|----------|
| 总结视频内容 | 获取字幕后进行摘要 |
| 回答视频相关问题 | 在字幕文本中搜索相关信息并回答 |
| 提取关键信息 | 从字幕中提取要点、数据、引用等 |
| 翻译视频内容 | 获取英文字幕后翻译为用户所需语言 |

## 错误处理

| 错误情况 | 处理方式 |
|----------|----------|
| 无效的 URL | 提示用户检查链接格式，需为 YouTube 视频链接 |
| 视频无字幕 | 提示用户该视频没有可用的字幕 |
| 网络连接失败 | 提示用户检查网络连接后重试 |
