# CodeFart — 待验证项

## 1. Claude Code Hook Schema 实测 ✅

**结论：Stop hook schema 正确，触发正常。**

测试配置：
```json
{
  "hooks": {
    "Stop": [
      {
        "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "echo $(date) HOOK_FIRED >> /tmp/test.txt && afplay /System/Library/Sounds/Ping.aiff"
          }
        ]
      }
    ]
  }
}
```

- [x] 响应完毕后是否触发 hook → **是**，`claude -p "say hi"` 返回 `hi` 后 `/tmp/test.txt` 写入成功
- [x] 是否能听到系统提示音 → **是**，Ping.aiff 播放正常
- [x] 配置结构是否被 Claude 接受 → **是**，无 JSON 校验报错
- [x] `matcher` 空字符串是否匹配所有 Stop 事件 → **是**（至少匹配了 `-p` 模式）

## 2. `codefart play` 作为 hook 命令的延迟

- [ ] Hook 命令执行到音效播完的总延迟是否在 1 秒以内
- [ ] 是否影响 Claude 显示响应文本的时机
- [ ] Hook 是同步还是异步？（从测试观察：Claude 打印响应文本后才执行 hook）

## 3. Rodio 在 macOS 上的表现

- [ ] `rodio` 默认输出设备选择是否正确
- [ ] 是否需要处理蓝牙耳机未连接时的回退
- [ ] 播放 WAV 内存数据的延迟和 CPU 开销

## 4. Claude Code 没有安装时的行为

- [ ] `codefart setup` 如果没有检测到 `~/.claude/` 目录的处理策略

## 5. 音效素材

- [ ] 5 个屁声音效的来源和版权确认
