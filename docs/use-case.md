# CodeFart — Use Case

## 用户的一天

### 安装（一辈子一次）

```bash
$ brew install codefart
$ codefart setup
✓ Added Stop hook to ~/.claude/settings.json
✓ Done. Next Claude session will notify you with a sound.
```

### 日常使用（什么都不用做）

```bash
$ claude "帮我写个 N 皇后问题的解"
# ... Claude 正常交互，你切到 Twitter 刷了两条 ...
# 💨 PUUUU  ← 屁响了，Claude 响应完毕
$ # 切回来，继续干活
```

用户完全感知不到 codefart 的存在。不 alias、不 shell 配置、不包装任何命令。是 Claude 自己调用 codefart 播放音效。

### 换音效（30 秒）

```bash
$ codefart list
  classic   ← current
  wet
  tiny
  squeaky
  thunder

$ codefart theme thunder
Theme set to "thunder"
# 下次 Claude 返回 → 雷声 ⚡
```

### 用自己的音效

```bash
$ codefart set-sound ~/Downloads/vine-boom.mp3
Custom sound set.

$ codefart reset   # 不喜欢？恢复默认
Reset to classic.
```

---

## 为什么没有安全顾虑

codefart 不包装 claude，不拦截任何东西。

看一眼 `~/.claude/settings.json` 就全透明：

```json
{
  "hooks": {
    "Stop": [{
      "hooks": [{
        "type": "command",
        "command": "codefart play"
      }]
    }]
  }
}
```

Claude 响应完毕 → Claude 自己说"该通知了" → 执行 `codefart play` → 放屁。
