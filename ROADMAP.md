# Roadmap

## 当前阶段

右侧 Agent 面板第一版已实现并通过 macOS 验证：显式选择工作目录，通过本机 Codex App Server 对话。

## 已完成

- 可折叠面板、目录选择与重启后的目录记忆；折叠保留会话。
- AI 面板左边缘支持手工拖拽宽度，交互沿用左侧栏；默认 380，最小 300，最大 800 并随窗口限制。松开后独立保存宽度，折叠展开和重启恢复，窗口缩小时收紧显示宽度。
- 仅 AI 回复使用只读 Markdown 渲染，用户问题保留原文；支持标题、行内样式、链接、嵌套列表、引用、分隔线、代码块和表格。代码可复制并横向滚动，宽表格可横向滚动，流式未闭合围栏可显示；复用现有解析器，无新增依赖，不创建编辑器或笔记记录。
- 工作目录入口位于输入框下方左侧，与发送／停止按钮同一行；显示文件夹图标和目录名，长名称截断，悬停显示完整路径，生成期间不可切换。
- 模型选择位于会话输入框上方，读取本机模型列表并支持分页，保留跟随配置；切换对下一条消息生效且沿用当前线程，生成期间禁用，新会话恢复默认选择。
- 展开 Agent 侧栏时自动引用当前打开的文档，按文档 ID 去重，输入框上方可移除；切换文档、新会话和选择目录保留引用。发送时导出最新 Markdown 正文，优先读取当前及其他窗口的未保存缓冲区，未打开的文档从文档库读取；读取失败保留输入并阻止发送。添加引用本身不发送内容，首页／设置不自动引用，图片和附件不随正文发送。
- 右侧入口改为右上角的展开／收起图标；折叠后不保留整列 AI 入口或面板宽度，正文填满剩余空间。顶栏按钮和主页搜索为该入口留位。
- Codex 多轮会话、流式消息、停止、新会话、命令和文件修改审批；未知交互明确拒绝。
- 新会话、切换目录、窗口关闭及应用退出时清理本面板的 Codex 子进程。
- `src/agent.rs` 负责 GPUI 面板，`src/agent/codex.rs` 负责子进程与协议；文档内核不承载 Agent 逻辑。
- 补充使用说明、架构说明、协议回归测试和模拟 Agent 的界面回放。
- 上一轮本地 Apple Silicon DMG：`dist-release.local-233b3_rl/quiet-md-0.1.11-agent-preview-macos-arm64.dmg`，应用名为 Quiet.md Preview，临时签名、未公证，关闭 `devtools`；尚不含右上角入口调整。
- 最新 Preview Apple Silicon DMG：`dist-release.local-c08rodqb/quiet-md-0.1.11-agent-preview-macos-arm64.dmg`，包含仅 AI 回复 Markdown 渲染、面板手工调宽和当前文档自动引用；模型选择位于输入框上方，目录与发送／停止位于输入框下方，临时签名、未公证，关闭 `devtools`。
- 最新本地 Release DMG：`dist-release.local-1t7xu6vx/quiet-md-0.1.11-release-macos-arm64.dmg`，版本 0.1.11，应用名 `Quiet.md.app`，使用正式图标和 `app.quietmd.mac` 标识，包含当前工作区功能；Release 优化构建，关闭 `devtools`，临时签名、未公证，未公开发布。

## 进行中

- 暂无。

## 待办

- 接入 Claude Code、Pi，分别保留会话上下文。
- Agent 修改当前笔记时的保存与外部变更协调，待单独设计。
- Windows、Linux 实机验证；跨重启恢复聊天记录尚未实现，当前只记忆工作目录和面板宽度。
- 重拍指南桌面截图：`shared-mock/guides/shots/shots.sh` 本机运行失败，现有截图未修改。

## 阻塞

- 本机仅有 Command Line Tools，缺少 `metal` 编译器，默认预编译着色器的构建检查受阻。已使用 GPUI 自带的 `gpui/runtime_shaders` 完成构建与验证，不改变默认构建配置。
- 指南重拍脚本的桌面捕获返回 `CGWindowListCreateImage returned null`，且系统返回的壁纸路径已失效；需要可用的系统屏幕捕获环境。面板本身已使用应用内部截图回放验证。

## 最近验证

- 本机 Codex CLI 为 `0.169.0`，支持 App Server 和协议 schema 导出。
- `cargo test --release --locked --workspace --features gpui/runtime_shaders,devtools`：210 项通过，5 项按原约定忽略，含需要单独运行的真实 Agent 测试；覆盖模型分页、过滤、恢复配置模型、失败重试、文档引用 Markdown 导出和消息边界，以及回复 Markdown 结构／样式、有序编号和逐字符流式前缀。
- `live_codex_round_trip -- --ignored`：本机已登录 Codex 读取合成文档并在下一轮保留上下文，两轮真实对话通过，未发送用户笔记。
- `python3 tools/harness/agent_panel.py`：多轮对话、折叠保留、审批允许/拒绝、停止、新会话、含空格目录、目录切换、重启记忆及子进程清理通过。
- Release Clippy：普通和 `devtools` 两种配置均通过 `-D warnings`，均附加 `gpui/runtime_shaders`；`vendor/xattr` 保留原有 5 条依赖弃用警告。
- 内置指南 XML 原样往返通过，6 个指南资源与源文件一致，`git diff --check` 通过。
- 本地 DMG：20,377,171 字节；完整性校验、只读挂载、Applications 入口、Info.plist、应用签名、thin arm64 架构及挂载后二进制与签名源逐字节一致性验证通过。校验摘要和详细结果保存在产物目录的 `.sha256` 与 `verification.json`。
- 右上角入口调整：`agent_panel.py` 新增折叠后无侧栏背景列的像素断言及主页顶栏回放，全部通过；原对话、审批、停止、会话和目录回归继续通过。Release Clippy（`gpui/runtime_shaders,devtools`，`-D warnings`）及 `git diff --check` 通过。
- 最新 Release DMG：20,494,505 字节，SHA-256 为 `c2b3fb4b3badd524d69d488dd55eebac8029037919e3416fba6212f19930944f`；完整性、只读挂载、Applications 入口、Info.plist、应用临时签名、arm64 架构及签名后二进制内容比对均通过。
- 目录入口下移：`agent_panel.py` 已从输入框下方选择和切换目录，长目录名截断及完整路径提示经截图确认；多轮对话、审批、停止、目录记忆和进程清理回归通过。Release Clippy（`gpui/runtime_shaders,devtools`，`-D warnings`）通过。
- 模型选择：本机真实读取 37 个模型并完成两轮对话；模拟 Agent 界面回放验证分页列表、切换、恢复配置模型、生成期间禁用、读取失败后的继续对话和 Esc 关闭菜单，且消息没有写入笔记。Release Clippy（`gpui/runtime_shaders,devtools`，`-D warnings`）及 `git diff --check` 通过。
- 模型选择框上移：已确认位于输入框上方；下方目录与发送按钮同排。新版界面回放覆盖模型选择、恢复默认、禁用、失败回退及目录／会话回归，全部通过；截图检查、Release Clippy（`gpui/runtime_shaders,devtools`，`-D warnings`）通过。
- 文档引用：模拟 Agent 实际收到的正文经核对，覆盖添加后再次编辑、未保存内容、跨文档保留、关闭／重新打开后的最新正文、跨窗口未保存内容、去重、移除、首页／设置不自动添加、读取失败保留输入。界面截图检查及原模型、目录、会话回归通过。
- AI 面板调宽：208 项 release 测试及普通／devtools Release Clippy 通过。界面回放核对拉宽／拉窄、最小／最大宽度、窗口缩小约束、折叠重开、重启恢复和左右栏独立保存；原对话、模型、引用回归通过。
- AI 回复 Markdown：完整界面回放通过；截图核对 AI 样式与用户原文、未闭合代码块、表格，并以像素差异验证长代码和宽表格实际横向滚动。复制经模拟剪贴板粘回输入框后，核对 Agent 收到原始代码；通用输入框接入已有模拟剪贴板，测试不依赖系统剪贴板。原模型、目录、引用及宽度回归通过，普通／devtools Release Clippy 通过。
- 本地 Release 打包：`cargo build --release --locked --features gpui/runtime_shaders`、无 `devtools` 的 workspace 测试（210 项通过、5 项忽略）和 Clippy（`--all-targets -- -D warnings`）通过。钥匙串查询为 0 个有效签名身份，未执行 Developer ID 签名或公证；沿用运行时着色器构建以适配本机 CLT 环境。
