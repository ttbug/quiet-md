# Roadmap

## 当前阶段

右侧 Agent 面板第一版已实现；当前合并 main（`47e203d`）的多文件夹工作区及存储更新，保留本机 Codex 面板。

## 已完成

- 2026-11-23 解决 main（`47e203d`）合并冲突：`Settings` 同时保留 Agent 工作目录、面板宽度和每个文件夹的外观设置，新增旧配置兼容及保存不丢字段回归；Markdown 刷新回归适配多文件夹存储的 `with` 接口。Release workspace 测试、三种配置 Clippy、指南及 Agent／Markdown 界面回归通过，尚未创建合并提交。
- 2026-11-22 解决 main（`0f408c4`，0.1.14）合并冲突：保留 main 的新版 README 结构并迁入 Local Codex 说明；主页搜索同时为三个操作按钮及 AI 入口留位。AI 回复改用可配置界面字体接口，Markdown 刷新回归按新签名传入当前窗口 ID，移除被 main 的窗口枚举实现替代的冗余字段。Release 测试、三种配置 Clippy、指南及界面回归通过，尚未创建合并提交。
- 2026-11-19 修复恢复 Markdown 文档后窗口激活闪退：`Workspace` 保存所属窗口 ID，刷新、接收外部版本及保存后的兄弟窗口同步均在读取实体前排除当前窗口，避免 GPUI 的窗口重入 panic。新增 `tools/harness/markdown_refresh.py`，不激活窗口，仅操作临时文档库。
- 2026-11-19 按当前源码 `11fcd6e` 重新构建 0.1.13 本地 Apple Silicon DMG：`dist-release.local-wrf3kk5d/quiet-md-0.1.13-release-macos-arm64.dmg`。使用 dist 优化及 `gpui_platform/runtime_shaders`，关闭 `devtools`；临时签名、未公证，未公开发布，保留旧包、已安装应用和真实文档库。
- 2026-11-18 合并 main（`159e3d4`）：解决 CONTRIBUTING.md 冲突，保留 AI 面板使用说明及 main 的 `vendor/zed` 维护流程；适配新版 GPUI 的焦点、滚动边界、菜单锚点和异步测试回调 API。构建说明改用 `gpui_platform/runtime_shaders`，未修改 vendor 源码。
- 2026-11-15 合并 main（`672a1ad`）：解决 README 功能列表与窗口渲染冲突，保留版本历史、拖拽导入和 AI 面板；标题栏／搜索按实际 AI 面板宽度布局。历史预览期间，当前及其他窗口的 AI 引用仍读取当前版本的正文和标题。
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

- main（`47e203d`）合并到 `feat/own`：冲突及自动合并后的接口兼容问题已解决并验证，已暂存，保留合并状态，等待创建合并提交。

## 待办

- 接入 Claude Code、Pi，分别保留会话上下文。
- Agent 修改当前笔记时的保存与外部变更协调，待单独设计。
- Windows、Linux 实机验证；跨重启恢复聊天记录尚未实现，当前只记忆工作目录和面板宽度。
- 重拍指南桌面截图：`shared-mock/guides/shots/shots.sh` 本机运行失败，现有截图未修改。

## 阻塞

- 本机仅有 Command Line Tools，缺少 `metal` 编译器，默认预编译着色器的构建检查受阻。已使用 GPUI 平台层的 `gpui_platform/runtime_shaders` 完成构建与验证，不改变默认构建配置。
- 指南重拍脚本的桌面捕获返回 `CGWindowListCreateImage returned null`，且系统返回的壁纸路径已失效；需要可用的系统屏幕捕获环境。面板本身已使用应用内部截图回放验证。

## 最近验证

- 2026-11-23 合并验证：Release workspace 测试（`devtools`）1121 项通过、8 项忽略；普通、`devtools`、`devtools,datatable` 三种配置 Release Clippy（`--all-targets -- -D warnings`）通过，均使用 `gpui_platform/runtime_shaders` 和 `MACOSX_DEPLOYMENT_TARGET=13.0`。指南 XML 往返、6 个指南资源一致性、完整 Agent 面板回放及 Markdown 刷新防闪退回归通过。测试仅使用临时文档库；首轮恢复副本用例因沙箱权限失败，沙箱外完整重跑通过。无未解决索引项，本次修改文件空白检查通过；全量暂存 diff 的 10 处空白提示来自与 main 完全一致的两个 vendor 补丁，保留原样。
- 2026-11-22 合并验证：Release workspace 测试（`devtools`）972 项通过、8 项忽略；普通、`devtools`、`devtools,datatable` 三种配置 Release Clippy（`--all-targets -- -D warnings`）通过；本机均启用 `gpui_platform/runtime_shaders`，设置 `MACOSX_DEPLOYMENT_TARGET=13.0`。指南 XML 往返及 6 个指南资源一致性通过。完整模拟 Agent 面板回放和 Markdown 刷新防闪退回归通过；主页截图确认搜索、三个操作按钮及 AI 入口无重叠。测试仅使用临时文档库，恢复用例与图形回放在获得沙箱外权限后通过。本次修改文件空白检查通过；完整暂存 diff 的 9 处空白提示来自与 main 完全一致的两个 vendor 补丁，保留原样。
- 2026-11-19 修复版 DMG：`dist-release.local-prz7w6rp/quiet-md-0.1.13-release-macos-arm64.dmg`，30,586,936 字节，SHA-256 `15487d7b044af9ff8b2334f039f8169506516d13fc0aff51b4df760462ccdb3e`。dist 构建、镜像 CRC、只读挂载、签名、arm64、包内二进制及 dSYM UUID 匹配通过，验证挂载已卸载。成品在临时库启动 15 秒无 panic，核对本机打开记录确认实际打开已有 `Startup.md`，随后仅结束本次测试进程。临时签名、未公证，关闭 `devtools`，未公开发布或替换已安装应用；验证文件与调试符号保留在产物目录。
- 2026-11-19 Markdown 闪退回归：修复前在窗口回调栈内刷新稳定复现 `attempted to read a window that is already on the stack`；修复后，同一回归覆盖回调内刷新、外部版本同步到两个窗口、未保存内容保护及镜像保存状态，全部通过。Release workspace 测试 852 项通过、6 项忽略，普通／devtools 两种 Release Clippy（`--all-targets -- -D warnings`）、指南 XML 往返、指南资源一致性及 `git diff --check` 通过；本机均使用 `gpui_platform/runtime_shaders`，未修改文档内核或用户数据。
- 2026-11-19 本地 DMG 重建：dist 构建、Release workspace 测试（852 项通过、6 项忽略）、普通／devtools 两种 Release Clippy（`--all-targets -- -D warnings`）、指南 XML 原样往返及 6 个指南资源一致性通过。恢复文件测试首轮受沙箱权限阻止，沙箱外完整重跑通过。DMG CRC、只读挂载、Applications 入口、版本、签名、thin arm64、包内二进制一致性及 dSYM UUID 检查通过，验证挂载已卸载。新包在临时文档库启动 15 秒仍存活、无 panic，随后仅结束本次测试进程；不代表已定位或修复此前报告的闪退。DMG 为 30,586,334 字节，SHA-256 `147a17ca29cb4cf9b527ac4a6677e6ad29898ca013b0f07b980e0b9e0a6d4865`，验证记录和调试符号位于产物目录。
- 2026-11-18 合并验证：普通／devtools Release workspace 测试均为 847 项通过、6 项忽略，两种配置的 Clippy（`--all-targets -- -D warnings`）通过；本机命令均设置 `MACOSX_DEPLOYMENT_TARGET=13.0` 并启用 `gpui_platform/runtime_shaders`。2000 个随机种子的编辑测试、指南 XML 往返和 6 个指南资源一致性通过。完整 Agent 界面回放及模型菜单截图检查通过，覆盖对话、折叠、审批、停止、模型／目录切换、目录及宽度记忆、Markdown／代码复制和滚动、跨窗口与历史预览引用、读取失败保留输入及子进程清理。
- 本轮冲突标记及未解决索引项已清除，本次修改的 6 个文件空白检查通过。全量暂存 diff 有 95 处上游空白提示，涉及 30 个 vendor／许可证文件；逐个核对内容哈希均与 `MERGE_HEAD` 一致，未修改第三方源码或补丁上下文。
- 2026-11-15 合并验证：普通／devtools Release workspace 测试均为 239 项通过、5 项忽略；两种配置的 Clippy（`--all-targets -- -D warnings`）通过，均附加 `gpui/runtime_shaders`。2000 个随机种子的编辑测试、指南 XML 往返、6 个指南资源一致性及完整 AI 面板界面回放通过；回放新增本窗口和其他窗口历史预览时读取当前正文、预览标题与当前标题隔离的场景，测试配置从 Cargo.toml 读取当前版本以避免更新弹窗干扰。冲突标记、未解决索引项及 diff 空白检查通过。
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
