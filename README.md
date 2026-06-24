# aichat

终端 AI 聊天工具，基于 Rust + [ratatui](https://github.com/ratatui/ratatui) 构建，支持 TUI 和轻量纯文本两种模式。

## 运行模式

### TUI 模式（默认）

```bash
cargo run
# 或
./aichat
```

全屏终端界面，状态栏 + 聊天区 + 输入区三段布局，支持消息滚动、模型切换、主题切换、中英文双语。

### --simple 模式

```bash
cargo run -- --simple [model]
# 例如
cargo run -- --simple deepseek
cargo run -- --simple qwenturbo
cargo run -- --simple custom:MyModel
```

不使用 ratatui，直接在终端中输入输出，适合 fbterm 等非标准终端环境。

---

## 键盘快捷键

### TUI 模式

| 按键 | 说明 |
|---|---|
| `←` `→` | 切换 AI 模型 |
| `↑` `↓` | 滚动聊天历史 |
| `PageUp` `PageDown` | 快速翻页（10 行） |
| `Home` `End` | 跳到顶部 / 底部 |
| `i` | 进入编辑模式 |
| `Enter` | 发送消息 |
| `C` / `c` | 切换中文 |
| `E` / `e` | 切换英文 |
| `1` `2` `3` `4` | 切换主题 |
| `F1` | 显示 / 隐藏帮助 |
| `Q` / `q` | 退出 |

编辑模式下：

| 按键 | 说明 |
|---|---|
| `Enter` | 发送消息并退出编辑模式 |
| `Esc` | 清空输入并退出编辑模式 |
| `Backspace` | 删除最后一个字符 |
| `Delete` | 清空整行 |

### --simple 模式

| 按键 | 说明 |
|---|---|
| `Enter` | 发送消息 |
| `Alt+Enter` | 换行（多行输入） |
| `Backspace` | 删除上一个字符 |
| `Esc` | 退出 |
| `Ctrl+D` | 输入为空时退出 |

---

## 支持模型

### 真实 API（需配置 Key）

| 模型 | 环境变量 | 说明 |
|---|---|---|
| DeepSeek | `DEEPSEEK_API_KEY` | 支持 128K 上下文 |
| 通义千问-Turbo | `ALIYUN_API_KEY` | 轻量版，响应快 |
| 通义千问-Plus | `ALIYUN_API_KEY` | 增强版，复杂任务 |
| 通义千问-Max | `ALIYUN_API_KEY` | 最强版，专业需求 |
| 通义千问-长文本 | `ALIYUN_API_KEY` | 128K 长文本 |

### 模拟回复（无需 Key，预留接口）

OpenAI GPT、Claude、Gemini、LocalLLM、Custom（自定义名称）

---

## 主题

`1` `2` `3` `4` 快速切换：

| 编号 | 名称 | 主色调 |
|---|---|---|
| 1 | 深蓝海洋 | 蓝 |
| 2 | 森林绿 | 绿 |
| 3 | 日落橙 | 橙 |
| 4 | 霓虹赛博 | 紫/青 |

---

## 构建

```bash
cargo build --release
```

发布构建已启用 `opt-level="z"`、`lto`、`strip`、`panic="abort"` 等尺寸优化。

### 交叉编译 armhf

```bash
rustup target add armv7-unknown-linux-gnueabihf
cargo build --release --target armv7-unknown-linux-gnueabihf
```

---

## 配置

在项目根目录创建 `.env` 文件：

```env
DEEPSEEK_API_KEY=sk-xxxxxxxx
ALIYUN_API_KEY=sk-xxxxxxxx
```

或通过系统环境变量设置。

---

## 依赖

- [ratatui](https://crates.io/crates/ratatui) + [crossterm](https://crates.io/crates/crossterm) — TUI 框架
- [tokio](https://crates.io/crates/tokio) — 异步运行时
- [reqwest](https://crates.io/crates/reqwest) — HTTP 客户端（rustls）
- [pulldown-cmark](https://crates.io/crates/pulldown-cmark) — Markdown 渲染（--simple 模式）

## License

MIT
