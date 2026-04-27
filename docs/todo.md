# CodeFart — TODO

## Phase 0: 验证

- [x] 手动往 `~/.claude/settings.json` 加 Stop hook，跑 Claude 确认触发
- [x] 确认 hook 是同步还是异步执行，是否阻塞 Claude
- [x] 确认 hook schema 精确结构（字段名、嵌套层级）
- [x] 找齐 5 个屁声音效素材，确认版权可用（占位合成音效已生成，后续替换真实素材）

## Phase 1: 核心 ✅

- [x] `cargo init`，搭项目骨架
- [x] 嵌入 5 个 WAV 音效（`rust-embed`）
- [x] 实现 `audio.rs`：从内存解码 WAV 并用 rodio 播放
- [x] 实现 `config.rs`：读写 `~/.config/codefart/config.toml`
- [x] 实现 `cli.rs`：clap 子命令路由
  - [x] `play` — 读取配置，播放音效，exit 0
  - [x] `list` — 列出内置主题 + 标记当前
  - [x] `theme <name>` — 切换主题
  - [x] `set-sound <path>` — 设置自定义音效
  - [x] `reset` — 恢复默认
  - [x] `setup` — 注入 Claude hook
- [x] 实现 `setup.rs`：读写 `~/.claude/settings.json`，注入 Stop hook
- [x] Claude hook 集成端到端验证通过

## Phase 2: 附加 ✅

- [x] 实现 `runner.rs`：`codefart run -- <cmd>` 通用命令包装
- [x] 自定义音效文件的格式校验（存在 + 可读）
- [x] 路径 `~` 展开

## Phase 3: 发布

- [ ] `Cargo.toml` 填好 metadata（author, repo, description）
- [ ] GitHub Release 编译流水线（macOS arm64 + x86_64）
- [ ] Homebrew formula
- [ ] README.md
- [ ] 15 秒 demo gif

## Phase 4: 后续

- [ ] `codefart preview <name>` — 试听内置音效
- [ ] 支持更多 AI 工具的 hook（如果它们有类似机制）
- [ ] 安装统计埋点（可选，用于验证 PRD 的 success metrics）
