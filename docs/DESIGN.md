# CodeFart Desktop — Design System

继承 landing page 的 scribble + comic 风格，适配 400×500 设置面板。

## 色彩

```css
--cream:    #fffbeb;   /* 底板 */
--paper:    #fffdf4;   /* 卡片/区块 */
--text:     #1a1410;   /* 主文字 */
--muted:    rgba(26,20,16,0.62);  /* 辅助文字 */
--orange:   #f05a28;   /* 主强调色，按钮/开关/选中态 */
--shadow:   rgba(89,46,21,0.14);  /* 硬投影色 */
--sticky:   #fef9c3;   /* 便签黄，输入框背景 */
--border:   rgba(26,20,16,0.12);  /* 细线分割 */
```

## 字体

| 用途 | 字体 |
|------|------|
| 标题 | Fredoka, 600-700 |
| 正文 | -apple-system (macOS 原生，Chalkboard 在小窗口下太花) |
| 代码 | SF Mono |

```css
.title { font-family: "Fredoka", sans-serif; font-weight: 700; }
.heading { font-family: "Fredoka", sans-serif; font-weight: 600; }
.body { font-family: -apple-system, sans-serif; }
.mono { font-family: "SF Mono", monospace; }
```

## 底板

- 背景色 `--cream`
- overlay: 点阵纹理（与 landing page 一致），opacity 0.12
- 4px 间距的点，12px grid

## 区块（Section Card）

每个设置分组是一个 scribble 卡片：

```css
.section {
    background: var(--paper);
    border: 2px solid var(--text);
    border-radius: 255px 15px 225px 15px / 15px 225px 15px 255px;
    padding: 16px;
    margin-bottom: 12px;
    box-shadow: 4px 4px 0 var(--shadow);
}
.section-title {
    font-family: "Fredoka", sans-serif;
    font-weight: 600;
    font-size: 14px;
    color: var(--muted);
    margin-bottom: 10px;
}
```

## 开关（Toggle & Check）

不使用系统原生控件，用 scribble 风格的 pill switch：

```css
.toggle {
    width: 44px;
    height: 24px;
    border-radius: 12px;
    border: 2px solid var(--text);
    background: #e0d8cb;
    transition: background 0.15s;
    box-shadow: 2px 2px 0 var(--shadow);
}
.toggle.active {
    background: var(--orange);
}
.toggle-knob {
    width: 18px; height: 18px;
    border-radius: 50%;
    background: var(--paper);
    border: 2px solid var(--text);
    transform: translateX(2px);
    transition: transform 0.15s;
}
.toggle.active .toggle-knob {
    transform: translateX(22px);
}
```

## 主题选择（ThemePicker）

每行一个主题，scribble-sm 边框，hover 翘起：

```css
.theme-row {
    display: flex; align-items: center; gap: 10px;
    padding: 8px 12px;
    margin-bottom: 6px;
    border: 2px solid var(--border);
    border-radius: 255px 15px 225px 15px / 15px 225px 15px 255px;
    background: var(--paper);
    cursor: pointer;
    transition: transform 0.12s, box-shadow 0.12s;
}
.theme-row:hover {
    transform: translateY(-2px) rotate(0.5deg);
    box-shadow: 3px 3px 0 var(--shadow);
}
.theme-row.selected {
    border-color: var(--orange);
    background: #fff5f0;
}
.theme-dot {
    width: 10px; height: 10px;
    border-radius: 50%;
    background: var(--muted);
}
.theme-row.selected .theme-dot {
    background: var(--orange);
}
.preview-btn {
    background: none; border: none; cursor: pointer;
    font-size: 16px; /* 🔊 emoji */
}
```

## 输入框（通知文案）

便签风格，仿手写体底色：

```css
.input {
    width: 100%;
    padding: 8px 12px;
    background: var(--sticky);
    border: 2px solid rgba(89,46,21,0.18);
    border-radius: 12px 4px 12px 4px;
    font-family: -apple-system, sans-serif;
    font-size: 14px;
    color: var(--text);
    box-shadow: 2px 3px 0 var(--shadow);
    transform: rotate(-0.3deg);
}
.input:focus {
    outline: none;
    border-color: var(--orange);
    transform: rotate(0deg);
}
```

## 按钮

两档：

**主按钮（试听、上传）：**

```css
.btn-primary {
    background: var(--orange);
    color: #fff;
    border: 2px solid var(--text);
    padding: 6px 16px;
    border-radius: 99px;
    font-family: "Fredoka", sans-serif;
    font-weight: 700;
    font-size: 14px;
    cursor: pointer;
    box-shadow: 3px 3px 0 var(--shadow);
    transform: rotate(0.5deg);
    transition: transform 0.12s, box-shadow 0.12s;
}
.btn-primary:hover {
    transform: rotate(-0.5deg) scale(1.04);
}
.btn-primary:active {
    transform: scale(0.95);
    box-shadow: none;
}
```

**次按钮（清除）：**

```css
.btn-ghost {
    background: transparent;
    border: none;
    color: var(--muted);
    font-size: 13px;
    cursor: pointer;
    text-decoration: underline;
    text-underline-offset: 2px;
}
.btn-ghost:hover { color: var(--orange); }
```

## 布局

```
┌──────────────────────────────┐
│  🔔 通知                       │
│  开关: [on]                 │  ← 通知开关
│  标题: [________]           │  ← 便签输入框
│  正文: [________]           │
├──────────────────────────────┤
│  🔊 声音                       │
│  ● classic          🔊      │  ← 主题行
│  ○ wet              🔊      │
│  ○ tiny             🔊      │
│  ...                        │
│  [上传自定义...] 清除        │
├──────────────────────────────┤
│  ⚡ 开机自启                  │
│  开关: [on]                 │
└──────────────────────────────┘
```

窗口尺寸 400×480，不可缩放。内边距 16px。区块间距 12px。

## 交互

- 主题选中：即时 `invoke("set_theme")`
- 试听：`invoke("preview_theme")`，点击后图标短暂变成 ⏸
- 通知文案：防抖 300ms 后保存
- 开关：即时切换
- 上传：`invoke("upload_sound")` → 打开原生文件对话框 → 复制到 sounds_dir → 刷新列表
