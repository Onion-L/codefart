# CodeFart Desktop v2 — Tech Spec

## 项目结构

```
codefart-app/
├── desktop/
│   ├── src-tauri/               ← Tauri Rust 端
│   │   ├── Cargo.toml
│   │   ├── tauri.conf.json
│   │   ├── capabilities/
│   │   │   └── default.json     ← 权限声明
│   │   ├── icons/
│   │   └── src/
│   │       ├── main.rs          ← 入口：解析 notify 子命令或启动托盘
│   │       ├── lib.rs           ← Tauri Builder 配置
│   │       ├── tray.rs          ← 系统托盘
│   │       └── commands.rs      ← Tauri commands
│   ├── src/                     ← React 前端
│   │   ├── main.tsx
│   │   ├── App.tsx
│   │   ├── App.css
│   │   ├── components/
│   │   │   ├── ThemePicker.tsx
│   │   │   ├── SoundUpload.tsx
│   │   │   ├── NotificationPrefs.tsx
│   │   │   └── AutostartToggle.tsx
│   │   └── hooks/
│   │       └── useConfig.ts
│   ├── index.html
│   ├── package.json
│   ├── tsconfig.json
│   └── vite.config.ts
└── crates/
    ├── codefart-core/           ← 复用
    └── codefart-cli/            ← + notify.rs, + setup --desktop
```

## 二进制职责

| 二进制 | 类型 | 说明 |
|--------|------|------|
| `codefart` | CLI | 所有命令行功能：`play`、`notify`、`setup`、`theme` 等 |
| `codefart-desktop` | 托盘进程 | 无 CLI。常驻 menu bar，通过 socket 接收事件。包含设置面板 Web UI |

分开的原因是 Tauri app 需要 WebView、是 `.app` bundle，不适合当 CLI 用。
反过来 CLI 二进制也做不了托盘。

### CLI 改动（`crates/codefart-cli`）

新增 `notify` 子命令：尝试 socket 通知托盘进程，连不上则 fallback 自己播放。

`setup` 加 `--desktop` flag：控制 hook 命令写 `codefart notify` 还是 `codefart play`。

### IPC 机制

macOS 上用 Unix domain socket，路径 `~/.config/codefart/notify.sock`。
CLI 连上后发一条字节，托盘收到后执行通知 + 播放。

## Rust 端 (src-tauri)

### Cargo.toml

```toml
[package]
name = "codefart-desktop"
version.workspace = true
edition.workspace = true

[build-dependencies]
tauri-build = { version = "2", features = [] }

[dependencies]
tauri = { version = "2", features = ["tray-icon"] }
tauri-plugin-notification = "2"
tauri-plugin-single-instance = "2"
tauri-plugin-autostart = "2"
tauri-plugin-dialog = "2"
tauri-plugin-shell = "2"
codefart-core = { path = "../../crates/codefart-core" }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

### commands.rs — Tauri Commands

| Command | 参数 | 说明 |
|---------|------|------|
| `get_config` | — | 返回 Config（复用 core） |
| `set_theme` | `theme: String` | 切换主题，写 config |
| `preview_theme` | `theme: String` | 试听主题（复用 core::audio） |
| `upload_sound` | — | 打开文件对话框 → 复制到 sounds_dir → 写 config |
| `clear_custom_sound` | — | 清除自定义声音 |
| `set_notification_enabled` | `enabled: bool` | 通知开关 |
| `set_notification_text` | `title, body: String` | 通知文案 |
| `set_autostart` | `enabled: bool` | 开机自启 |
| `play_sound` | — | 播放当前配置的声音 |

### tray.rs

- 图标：16x16 菜单栏 icon（PNG，模板色）
- 菜单：
  ```
  ┌─────────────────┐
  │ 🔔 Preferences  │ → 显示设置窗口
  │ 🔊 Preview      │ → 播放当前声音
  │ ─────────────── │
  │   Quit          │ → 退出
  └─────────────────┘
  ```
- 后台线程从 mpsc channel 接收 IPC 事件：
  1. 通知开关开启 → 弹系统通知
  2. 播放当前配置的声音

### ipc.rs — Unix Domain Socket 服务端

```rust
use std::os::unix::net::UnixListener;
use std::thread;

pub fn start_socket(tx: std::sync::mpsc::Sender<()>) {
    let sock_path = dirs::home_dir()
        .unwrap()
        .join(".config/codefart/notify.sock");
    let _ = std::fs::remove_file(&sock_path);
    let listener = UnixListener::bind(&sock_path).unwrap();

    thread::spawn(move || {
        for stream in listener.incoming() {
            if stream.is_ok() {
                let _ = tx.send(());
            }
        }
    });
}
```

### lib.rs — Tauri Builder

```rust
fn main() {
    let (tx, rx) = std::sync::mpsc::channel::<()>();

    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))  // 仅防多开
        .plugin(tauri_plugin_autostart::init(...))
        .plugin(tauri_plugin_dialog::init())
        .setup(move |app| {
            tray::build(app, rx)?;
            ipc::start_socket(tx);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![...])
        .run(tauri::generate_context!())
        .expect("error running tauri");
}
```

## 前端 (React)

### 组件树

```
App
├── ThemePicker        ← 主题列表 + 试听按钮
├── SoundUpload        ← 上传自定义声音 / 清除
├── NotificationPrefs  ← 通知开关 + 标题/正文字段
└── AutostartToggle    ← 开机自启开关
```

### 数据流

- 所有状态通过 Tauri `invoke` 读写，前端不直接操作文件
- `useConfig` hook 封装 `invoke("get_config")` 调用
- 设置变更即时 invoke 到后端保存

### 视觉

- 单窗口，固定 400×500
- 适配 macOS 外观（浅色/深色）
- 设置项分组，每组一个 section 标题
- 待定：设计风格（macOS 原生 / 暗色极简 / 玩味）

## CLI 改动

### notify.rs（新增）

```rust
use std::os::unix::net::UnixStream;
use codefart_core::config::Config;
use codefart_core::audio;

pub fn notify_desktop() -> Result<(), CodefartError> {
    let sock_path = Config::notify_socket_path();
    match UnixStream::connect(&sock_path) {
        Ok(mut stream) => {
            // 托盘在运行，发信号让它处理
            stream.write_all(&[1]).ok();
            Ok(())
        }
        Err(_) => {
            // 托盘没跑，fallback 自己播放
            let config = Config::load().unwrap_or_default();
            audio::play_sound(&config)
        }
    }
}
```

### main.rs 新增命令

```rust
Commands::Notify => notify::notify_desktop(),
```

### cli.rs 新增

```rust
/// Notify the desktop app to show notification + play sound (falls back to local playback)
Notify,
```

`setup` 新增 `--desktop` flag，控制写入 `codefart notify` 还是 `codefart play`。

## Hook 安装

`codefart setup --desktop` 写入：

```json
{
  "hooks": {
    "Stop": [{
      "matcher": "",
      "hooks": [{
        "type": "command",
        "command": "codefart notify"
      }]
    }]
  }
}
```

## 构建与分发

### 本地构建

```bash
cd desktop
npm install
npm run tauri build         # → src-tauri/target/release/bundle/dmg/*.dmg
```

### CI

- `macos-latest` runner
- 步骤：checkout → Node.js → npm install → cargo tauri build
- artifact：`.dmg` 文件
- 上传到 GitHub Release

### Release workflow 改动

在现有 `release.yml` 加一个 `desktop` job，仅在 macOS 上构建，产出 `.dmg`。

## Config 扩展

现有 `Config` 结构需要扩展：

```toml
# ~/.config/codefart/config.toml
theme = "classic"
# custom_sound = "/path/to/sound.wav"

[desktop]
notification_enabled = true
notification_title = "Claude"
notification_body = "已完成"
autostart = false
```

`codefart-core` 的 `Config` 统一管理，CLI 仅读写 theme/custom_sound，忽略 `[desktop]`。

`sounds_dir()` 旁新增 `notify_socket_path()` → `~/.config/codefart/notify.sock`。

## 实现顺序

1. Tauri 脚手架 + `codefart-core` 集成 → 能编译
2. 系统托盘 + IPC 通知 → 能收到 hook 并响应
3. Config 扩展 + commands → 前后端数据通
4. 设置面板 UI
5. DMG 打包 + CI
