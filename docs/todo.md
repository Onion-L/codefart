# CodeFart — TODO

## Phase 0: 验证 ✅

- [x] 手动往 `~/.claude/settings.json` 加 Stop hook，跑 Claude 确认触发
- [x] 确认 hook 是同步还是异步执行，是否阻塞 Claude
- [x] 确认 hook schema 精确结构（字段名、嵌套层级）
- [x] 占位合成音效已生成，后续替换为真实 fart-01/fart-02

## Phase 1: 核心 ✅

- [x] Rust 项目骨架：`audio.rs` / `config.rs` / `cli.rs` / `setup.rs` / `error.rs`
- [x] 6 个子命令：`play` / `list` / `theme` / `set-sound` / `reset` / `setup`
- [x] 嵌入音效 + rodio 播放
- [x] `~/.config/codefart/config.toml` 配置持久化
- [x] Claude Stop hook 注入 + 端到端验证

## Phase 2: 命令扩展 ✅

- [x] `codefart run -- <cmd>` 通用命令包装
- [x] `codefart remove` 清除自定义音效
- [x] `codefart update` 自更新（权限不够自动 sudo）
- [x] `codefart preview [name]` — 无参时箭头键交互试听，Enter 播放，Ctrl-C 退出
- [x] `codefart theme` — 无参时箭头键交互选择
- [x] 自定义音效复制到 `~/.config/codefart/sounds/` 统一管理
- [x] `~` 路径展开

## Phase 3: 发布 ✅

- [x] `Cargo.toml` metadata
- [x] GitHub Actions 编译 macOS arm64 + x86_64，打 tag 自动发布
- [x] `install.sh` / `uninstall.sh` curl 一键安装/卸载
- [x] README.md
- [ ] 15 秒 demo gif

## Phase 4: 后续

- [x] `codefart preview` 交互试听
- [x] 调研其他 AI 工具 hook 支持：Codex / Opencode / pi / Kimi Code 均无，仅 Claude Code 有
- [ ] 真实屁声音效补充（目前 2 个：classic / wet）
- [ ] demo gif
