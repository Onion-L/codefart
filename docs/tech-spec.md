# CodeFart — Technical Specification

**Version:** 0.2
**Status:** Draft
**Last Updated:** April 2026

---

## 1. Technology Stack

| Layer | Choice | Rationale |
|---|---|---|
| Language | Rust (stable) | 单二进制分发，无运行时依赖，`rodio`/`clap` 生态成熟 |
| CLI framework | `clap` v4 | 生态标准，derive API 简洁 |
| Audio playback | `rodio` | 纯 Rust，macOS 走 CoreAudio，支持 WAV/MP3/FLAC/Ogg |
| Asset embedding | `rust-embed` | 将 5 个屁声音效编译进二进制 |
| Config serialization | `serde` + `toml` | TOML 人类可读，适合 ~/.config 下的小文件 |
| Config path | `dirs` | 跨平台标准目录 (`~/.config/codefart/`) |
| Error handling | `anyhow` + `thiserror` | 库内 `thiserror`，main 层 `anyhow` |
| Logging | `tracing` + `tracing-subscriber` | 结构日志，生产可关闭 |

---

## 2. Architecture Overview

### 核心思路：不是包装 Claude，而是被 Claude 调用

CodeFart 不拦截任何命令、不修改任何 shell 配置、不代理任何进程。它只是一个**被 Claude Code 内置 hooks 系统调用的音效播放器**。

### 调用链

```
用户敲 claude "写个排序"
  → Claude Code 正常运行、交互
  → Claude 响应完毕
  → Claude 自身触发 Stop hook
  → 执行 codefart play
  → 播放音效
```

**信任模型**：codefart 是 Claude 的"插件"，由 Claude 自己调度。不碰 Claude 的 stdin/stdout/stderr/进程/网络。

### CLI 子命令

```
codefart play            ← 被 Claude hook 调用，播放当前音效
codefart list            ← 列出内置主题
codefart theme <name>    ← 切换主题
codefart set-sound <path>← 设置自定义音效
codefart reset           ← 恢复默认
codefart setup           ← 自动配置 Claude hook
```

---

## 3. Claude Code Hook 集成

### Hook 机制

Claude Code 提供 23 种 hook 事件，本次使用 `Stop`：

> **Stop** — fires at the end of every turn (including read-only ones)

Hook 通过 `~/.claude/settings.json` 配置：

```json
{
  "hooks": {
    "Stop": [
      {
        "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "codefart play"
          }
        ]
      }
    ]
  }
}
```

- `matcher: ""` — 匹配所有 Stop 事件（无需过滤）
- `type: "command"` — 执行 shell 命令
- 如果 codefart 未安装或出错，Claude 静默忽略，不影响 Claude 自身

### `codefart setup` 做的事

```
$ codefart setup
✓ Added Stop hook to ~/.claude/settings.json
✓ Done. Next Claude session will play a sound on response.
```

具体逻辑：
1. 读取 `~/.claude/settings.json`
2. 如果已有 codefart 的 Stop hook → 跳过，提示已安装
3. 否则在 `hooks.Stop` 数组中追加一条 hook
4. 保留文件中其他已有配置不变
5. 幂等：多次运行不会重复添加

如果没有 `~/.claude/settings.json`，创建一个最小文件。

---

## 4. `codefart play` — 核心子命令

```
1. 读取 ~/.config/codefart/config.toml
2. 解析音效来源：
     custom_sound 有值 → 播放指定文件
     theme 有值     → 播放对应内置音效
     都没有          → 播放 classic（默认）
3. 调用 rodio 播放
4. 退出
```

- 同步阻塞播放（0.2–1.5 秒），播放完才退出
- 音频错误静默吞掉，exit 0
- 无参数，无交互，专门为 hook 调用设计

---

## 5. CLI 兼容层：`codefart run`

保留 `codefart run -- <cmd>` 子命令作为**可选的通用包装器**。有些用户可能想给非 Claude 的命令也加音效（比如 `npm run build`、`sleep 10`）。

但这是附加功能，不是核心集成路径。setup 默认只配 Claude hook，不写 shell alias。

```
$ codefart run -- sleep 3
# 3 秒后音效播放
```

---

## 6. Sound Assets

### Embedding

```
sounds/
  classic.wav
  wet.wav
  tiny.wav
  squeaky.wav
  thunder.wav
```

通过 `rust-embed` 编译进二进制，运行时从内存解码。

### Spec

- 格式：WAV 16-bit PCM（rodio 原生支持，无需编解码器）
- 时长：0.2–1.5 秒
- 采样率：44.1kHz / 48kHz
- 单文件 < 200KB，5 个总 < 1MB

---

## 7. Configuration

### Path

```
~/.config/codefart/config.toml
```

### Schema

```toml
theme = "wet"                              # optional
custom_sound = "/Users/me/sounds/a.mp3"    # optional
```

### Priority

| custom_sound | theme | Result |
|---|---|---|
| set | any | 播放自定义文件 |
| none | "wet" | 播放内置 `wet` |
| none | none | 播放内置 `classic` |

---

## 8. Error Handling

与 PRD 一致：**音效失败不影响调用方**。

- 音频文件不存在 / 格式不支持 → 静默返回 exit 0
- 配置读取失败 → 回退默认主题，不崩溃
- 配置写入失败 → stderr 输出 warning，不阻塞
- `codefart play` 始终 exit 0（这样 Claude hook 不会报错中断）

---

## 9. Project Structure

```
codefart/
├── Cargo.toml
├── sounds/
│   ├── classic.wav
│   ├── wet.wav
│   ├── tiny.wav
│   ├── squeaky.wav
│   └── thunder.wav
├── src/
│   ├── main.rs               # 入口 + clap 路由
│   ├── cli.rs                 # clap 子命令定义
│   ├── audio.rs               # rodio 播放 + 嵌入资产
│   ├── config.rs              # ~/.config/codefart/config.toml 读写
│   ├── setup.rs               # Claude hook 注入 (~/.claude/settings.json)
│   ├── runner.rs              # run 子命令（通用命令包装，可选）
│   └── error.rs               # 错误类型
└── docs/
    ├── codefart-prd.md
    ├── tech-spec.md
    └── use-case.md
```

### Dependencies

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
rodio = "0.18"
rust-embed = "8"
serde = { version = "1", features = ["derive"] }
serde_json = "1"              # 读写 ~/.claude/settings.json
toml = "0.8"
dirs = "5"
anyhow = "1"
thiserror = "1"
tracing = "0.1"
tracing-subscriber = "0.3"
```

---

## 10. Build & Distribution

### Binary Size Target

- Release build (stripped): < 5 MB

### Homebrew

```ruby
class Codefart < Formula
  desc "Play a sound when your AI finishes thinking"
  homepage "https://github.com/xxx/codefart"
  url "..."
  def install
    bin.install "codefart"
  end
end
```

---

## 11. Open Decisions

| Item | Options |
|---|---|
| 音效素材来源 | 自录 / 免版税 / AI 生成 |
| `codefart play` 需不需要支持 `--theme` 参数直接指定？ | 暂不需要（hook 配置固定，切换通过 `codefart theme`） |
| Claude Code settings.json hook 字段精确 schema | 需要实测确认 nested key 结构 |
