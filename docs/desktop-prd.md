# CodeFart Desktop v2 PRD

## 目标
macOS 桌面应用，替代 CLI 的 hook 机制，提供系统通知 + 托盘常驻 + 设置面板。

## 功能

### 1. 系统托盘
- 菜单栏图标常驻
- 菜单项：Preferences / Preview / Quit
- 收到 hook 触发时，弹通知 + 播放声音

### 2. Hook 触发
- `codefart setup --desktop` 写入 `codefart notify`
- `codefart setup` 保持写入 `codefart play`（兼容旧行为）
- `codefart notify` 通过 Unix domain socket 发信号给托盘进程
- 托盘没运行则 fallback 自己播放声音（不丢事件）

### 3. 系统通知
- 标题/正文可配置，默认 "Claude 已完成"

### 4. 设置面板
- **主题选择**：classic / wet / tiny / squeaky / thunder，试听按钮
- **自定义声音**：上传本地音频文件
- **通知开关**：是否弹出系统通知
- **开机自启**：Login Item

### 5. 分发
- DMG 下载，GitHub Release
- 独立于 CLI 版本，用户可选装其中一个或两个

## 架构

```
Claude Hook → codefart notify (复用 CLI)
                    │
                    ▼ Unix domain socket
          ┌── codefart-desktop ──────┐
          │  通知 + 播放声音          │
          │  菜单栏图标               │
          │  设置面板 (Web UI)        │
          └──────────────────────────┘
                    │
          socket 不通时 CLI fallback
          自己播放声音（不丢事件）
```

## 技术选型

- **框架**：Tauri v2（复用 `codefart-core` Rust crate）
- **前端**：React + Vite（设置面板，单页）
- **插件**：
  - `tauri-plugin-notification`：系统通知
  - `tauri-plugin-single-instance`：防多开
  - `tauri-plugin-autostart`：开机自启
  - `tauri-plugin-dialog`：文件选择（自定义声音）
  - `tauri-plugin-shell`：从 hook 命令启动


