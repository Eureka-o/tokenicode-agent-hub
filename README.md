# TOKENICODE DeepSeek Alpha

一个面向 DeepSeek / CC Switch 使用场景的 TOKENICODE 魔改版。它保留 TOKENICODE 的桌面 GUI、会话管理、文件浏览和 Claude Code CLI 工作流，同时把模型显示、第三方 API、Skills 管理和翻译体验做了更适合本地使用的改造。

> 本项目基于 [TOKENICODE](https://github.com/yiliqi78/TOKENICODE) 修改而来。感谢原作者 TinyZ / yiliqi78 以及 TOKENICODE 项目的开源工作。本仓库保留原项目 Apache License 2.0 授权与 attribution，详见 [LICENSE](LICENSE) 和 [NOTICE](NOTICE)。

## 功能亮点

- **DeepSeek / CC Switch 适配**
  - 界面模型名改为 `DeepseekV4Pro` / `DeepseekV4Flash`
  - 实际 API 请求自动映射为 `deepseek-v4-pro` / `deepseek-v4-flash`
  - 兼容旧的 Claude/Fable/Opus/Sonnet/Haiku 显示名，自动归一到 DeepSeek 模型

- **独立 Provider 配置**
  - 支持 Anthropic 格式和 OpenAI 兼容格式
  - 支持自定义 Base URL、API Key、模型映射和代理
  - 适合接入 CC Switch、DeepSeek 兼容代理、第三方模型网关

- **Codex Skills 面板**
  - 自动扫描本机已安装的 Codex/Agent skills
  - 支持 `.codex/skills`、`.agents/skills`、`.claude/skills` 和插件缓存
  - 自动去重，避免插件缓存和本地目录重复显示
  - 可查看、编辑、启用/禁用、复制、定位 skill 文件

- **Skills 翻译**
  - 技能列表名称和简介可调用独立翻译 API 翻译为中文
  - `SKILL.md` 预览正文也可一键翻译
  - 翻译只影响预览，不会修改原始 skill 文件
  - 翻译结果本地缓存，减少重复 API 调用

- **内置网页预览**
  - 右侧边栏新增 Preview 面板，可直接打开网页、localhost 和本地预览地址
  - 支持后退、前进、刷新、外部浏览器打开
  - 提供 Tauri 预览控制命令，方便后续接入 AI/MCP 工具调用

- **界面主题与字体优化**
  - 新增 VS Code Dark 黑暗界面，接近 VS Code 的深色工作区观感
  - 新增纯白简约风格，适合喜欢干净白底界面的用户
  - 支持界面字体切换，可选择微软雅黑、系统字体、思源黑体、霞鹜文楷和等宽字体
  - 默认让路径、模型名、小标签等原本等宽字体区域跟随界面字体，减少中英文/拼音混排时的割裂感

- **更顺手的新对话流程**
  - 记住上一次选择的项目文件夹，作为新对话的默认工作目录
  - 点击“新任务”不再强制回到欢迎页重新选择文件夹
  - 输入框底部新增文件夹选择按钮，可以像 Codex 一样在对话区快速切换项目目录

- **桌面工作流**
  - Tauri 2 + React 19 桌面应用
  - 内置文件浏览、Markdown/HTML/SVG/PDF/图片预览
  - CodeMirror 编辑器，支持多种语言语法高亮
  - 会话历史、归档、置顶、导出、AI 标题生成
  - 支持计划模式、权限模式、回退和文件恢复

## 相比原版 TOKENICODE 多了什么

这个仓库不是简单改名版，而是围绕 DeepSeek、CC Switch 和 Codex skills 工作流做了一组定向魔改：

| 方向 | 原版 TOKENICODE | DeepSeek Alpha 魔改版 |
| --- | --- | --- |
| 模型显示 | 主要沿用 Claude / Opus / Sonnet / Haiku 等命名 | 显示为 `DeepseekV4Pro`、`DeepseekV4Flash`，并自动映射到真实 API model |
| CC Switch 适配 | 需要用户自己理解 Claude 名称和代理映射关系 | 内置 DeepSeek V4 Pro / Flash 显示与请求映射，减少填错模型名的问题 |
| Provider 配置 | 更偏原始 Claude Code 使用方式 | 增加独立 Provider 管理，支持 Anthropic / OpenAI 兼容格式、Base URL、API Key、模型映射、代理 |
| Skills 面板 | 原版没有面向 Codex skills 的完整管理面板 | 可以扫描、去重、查看、编辑、启用/禁用本机 Codex/Agent skills |
| Skills 翻译 | 需要自己读英文 `SKILL.md` | 可以调用独立翻译 API 翻译技能列表和 `SKILL.md` 预览正文，且只改预览不改原文件 |
| Preview 工具 | 没有内置网页预览控制面板 | 右侧新增 Preview，可打开网页、localhost、本地页面，并为后续 AI 控制预留命令 |
| 主题 | 以原有浅/深色和背景为主 | 新增 VS Code Dark、纯白简约等更偏工作流的界面风格 |
| 字体 | 字体跟随范围有限，部分小标签仍固定等宽 | 增加字体选择，并让小标签/路径/模型名等区域默认跟随界面字体 |
| 新对话目录 | 新建对话时容易回到重新选文件夹流程 | 记住上次项目目录，新任务直接进入默认文件夹，并可在输入框下方快速切换 |
| 发布包 | 需要自行构建或使用原项目发布 | Release 提供 Windows x64 exe / zip 和 SHA256 校验文件 |

## 下载

请到 GitHub Releases 下载 Windows 便携版：

- `tokenicode-deepseek-alpha-windows-x64.exe`

下载后双击运行即可。首次运行时请按需要配置 CC Switch / DeepSeek API。

## 快速开始

1. 下载 release 里的 Windows exe。
2. 打开 TOKENICODE。
3. 在设置里配置 API Provider，或在 Skills 面板里单独配置翻译 API。
4. 选择项目文件夹，开始对话。

### DeepSeek / CC Switch 模型建议

如果你的网关支持本项目当前的 DeepSeek V4 命名：

| 用途 | 显示名 | 实际 API model |
| --- | --- | --- |
| 高质量/复杂任务 | DeepseekV4Pro | `deepseek-v4-pro` |
| 快速/翻译/轻任务 | DeepseekV4Flash | `deepseek-v4-flash` |

如果你使用 DeepSeek 官方 OpenAI 兼容接口，请以官方实际支持的模型名为准，并在配置里选择 OpenAI 格式。

## Skills 翻译 API 配置

打开右侧 **技能** 面板，点击右上角齿轮按钮：

- `Anthropic` / `OpenAI`：选择接口格式
- `Base URL`：填写 API 地址，例如 CC Switch 或 DeepSeek 网关地址
- `API Key`：填写密钥
- `Model`：建议填写快速模型，例如 `deepseek-v4-flash`
- `Proxy URL`：可选，通常留空；仅需要代理时填写 `http://127.0.0.1:7890` 这类地址

配置后点击 `译` 即可翻译技能列表。打开 `SKILL.md` 预览时，也可以点击右上角 `译` 翻译正文。

## 本地开发

环境要求：

- Node.js
- pnpm
- Rust
- Tauri 2 构建环境
- Windows 打包需要 MSVC Build Tools

常用命令：

```powershell
pnpm install
pnpm build
pnpm tauri build
```

在本机使用 MSVC 环境构建的示例：

```powershell
cmd /c "call C:\BuildTools\VC\Auxiliary\Build\vcvars64.bat && set PATH=C:\Users\Administrator\.cargo\bin;%PATH% && cd /d D:\TOKENICODE\TOKENICODE-src && pnpm tauri build"
```

如果没有 Tauri 签名私钥，安装包签名阶段可能失败，但 `src-tauri\target\release\tokenicode.exe` 仍会生成。

## 与原 TOKENICODE 的关系

这是 TOKENICODE 的个人魔改版，主要目标是让本机 DeepSeek / CC Switch / Codex skills 工作流更顺手。核心桌面框架、Claude Code GUI 思路和大量基础功能来自原 TOKENICODE 项目。

本项目会在源码和文档中保留原项目许可声明。若你需要原版功能、跨平台安装包或官方更新，请优先参考原项目：

- 原项目仓库：[https://github.com/yiliqi78/TOKENICODE](https://github.com/yiliqi78/TOKENICODE)

## 许可证

本项目沿用原项目的 **Apache License 2.0**。

请阅读：

- [LICENSE](LICENSE)
- [NOTICE](NOTICE)

## 致谢

- [TOKENICODE](https://github.com/yiliqi78/TOKENICODE)：本项目的基础来源
- TinyZ / yiliqi78：TOKENICODE 原作者
- [Tauri](https://tauri.app)：桌面应用框架
- [React](https://react.dev)：前端 UI 框架
- Claude Code / Codex skills 生态：提供 agent 工作流基础
