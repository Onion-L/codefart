# CodeFart — 待验证项

## 1. Claude Code Hook Schema 实测 ✅

Stop hook schema 正确，触发正常。

- [x] 响应完毕后触发 hook
- [x] 音效正常播放（系统 Ping.aiff）
- [x] hook JSON 结构被 Claude 接受
- [x] `matcher: ""` 匹配所有 Stop 事件

## 2. `codefart play` 作为 hook 命令 ✅

- [x] hook 执行 → codefart play → 音效播放，全链路端到端验证通过
- [x] Claude 响应文本先显示，hook 后执行（不阻塞输出）
- [ ] 长时间 Claude 会话中 hook 稳定性（需更多测试）

## 3. Rodio 在 macOS 上的表现 ✅

- [x] 播放正常，默认输出设备选择正确
- [x] 从内存解码 WAV 延迟可接受（< 100ms）
- [ ] 蓝牙耳机回退场景（待测试）

## 4. 音效素材

- [x] 占位合成音效已生成
- [ ] 替换为真实屁声

## 5. 已知问题

- macOS 上 `dirs::config_dir()` 返回 `~/Library/Application Support/`，已变更为固定使用 `~/.config/codefart/`
