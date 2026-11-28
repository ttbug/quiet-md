<h1 align="center">Quiet.md</h1>

<p align="center">
  <b>又快又安静的笔记，就放在你自己的电脑上。</b><br>
  用 Rust + GPUI 原生绘制的笔记应用：真实的文件夹，敲下 Markdown 自动排版，笔记里还有一块白板。免费开源。
</p>

<p align="center">
  <a href="README.md">English</a> · 简体中文 · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a>
</p>

<p align="center">
  <a href="https://github.com/s1dashu/quiet-md/releases/latest"><img src="https://img.shields.io/github/v/release/s1dashu/quiet-md?label=release&color=2f6feb" alt="最新版本"></a>
  <a href="https://github.com/s1dashu/quiet-md/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/s1dashu/quiet-md/ci.yml?branch=main&event=schedule&label=CI" alt="CI 状态"></a>
  <img src="https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey" alt="支持 macOS、Windows 和 Linux">
  <img src="https://img.shields.io/badge/built%20with-Rust%20%2B%20GPUI-dea584?logo=rust" alt="使用 Rust 和 GPUI 构建">
  <a href="LICENSE"><img src="https://img.shields.io/github/license/s1dashu/quiet-md?color=green" alt="MIT 许可"></a>
  <img src="https://img.shields.io/badge/status-beta-orange" alt="Beta">
</p>

<p align="center">
  <a href="https://download.quiet.md/latest/quiet-md-macos-arm64.dmg"><img src="docs/images/download-macos.svg" height="56" alt="下载 macOS 版"></a>
  &nbsp;
  <a href="https://download.quiet.md/latest/quiet-md-windows-x64-setup.exe"><img src="docs/images/download-windows.svg" height="56" alt="下载 Windows 版"></a>
  &nbsp;
  <a href="https://download.quiet.md/latest/quiet-md-x86_64.AppImage"><img src="docs/images/download-linux.svg" height="56" alt="下载 Linux 版"></a>
</p>

<p align="center">
  <a href="https://quiet.md"><b>官网</b></a> ·
  <a href="#下载"><b>下载</b></a> ·
  <a href="#功能">功能</a> ·
  <a href="#性能">性能</a> ·
  <a href="CONTRIBUTING.md">参与开发</a>
</p>

<p align="center">
  <img src="docs/images/readme-home.webp" width="100%" alt="像素风草坡背景上的 Quiet.md：左侧侧栏列出文件夹，顶层文件夹页把每个文件夹显示成一叠带配图的笔记卡片。">
</p>

> [!NOTE]
> Quiet.md 目前是 **Beta 版本**，还在快速迭代。遇到 bug 或有想法，欢迎[提 issue](https://github.com/s1dashu/quiet-md/issues)，更欢迎直接提 PR，一起把它打磨好。

## 为什么是 Quiet.md

**📁 笔记就是普通文件。** 没有账号，没有云端，默认不联网（以下除外：可选的更新检查；应用和命令行工具 `quiet` 每天发送一次匿名使用次数，只有计数，例如应用打开了几次，可在「设置 › 通用 › 匿名使用统计」关闭；白板第一次用到中日韩手写字体、或者你选择可下载的文档字体时，会下载对应的字体文件）。侧栏里的每个文件夹都是磁盘上真实的文件夹，每篇笔记都是一个以标题命名的文件：Markdown 是 `.md`，富文本文档是 `.xml`。用 iCloud、Git 或任何同步工具都行；哪天不用 Quiet.md 了，文件也还在原地。

**⚡ 原生，而且快。** 整个界面由 Rust 和 [GPUI](https://www.gpui.rs/) 直接用 GPU 绘制，不是 Electron，也不是网页套壳。在一篇 100 万字的笔记里打字，每帧约 5 ms；120 Hz 屏幕给每帧的时间是 8.3 ms。

**✍️ 写字和画图，在同一个地方。** 敲 `#`、`-`、`[]` 或 `>`，Markdown 自动排好版。文字说不清的时候，直接在笔记里放一块白板。富文本笔记以飞书文档的 DocxXML 格式保存，从飞书或网页粘贴都能保留结构。

## 功能

### 写

<p align="center">
  <img src="docs/images/readme-write.webp" width="100%" alt="Quiet.md 里打开的一篇长笔记，有标题、行内公式和一张离子浓度表。">
</p>

标题、列表、待办、引用、高亮块、分栏、可合并单元格的表格、带语法高亮的代码、LaTeX 公式、图片、附件和封面。边写边用 Markdown 快捷输入，其余的都在 `/` 菜单里。提及另一篇笔记就能链接过去，对方会列出反向引用。数据库用单选、日期、数字、公式、查找引用、关联等字段记录信息，视图可以筛选、排序、分组，「笔记」列的每一格都是一篇真实的笔记。「…」菜单里有专注模式、打字机模式和段落聚焦，「显示」›「放大」只放大正文。

### 画

<p align="center">
  <img src="docs/images/readme-board.webp" width="100%" alt="Quiet.md 里的一块画板：图片和地图排在一起，配着手写标注、方框和连线箭头。">
</p>

画板是按 Excalidraw 移植的白板：图形、始终连着的箭头、画笔、文字、便签、分区和图片。可以放进笔记里，也可以存成独立的 `.excalidraw` 文件，excalidraw.com 能直接打开。触控板双指捏合就能缩放；一万个图形的画板依然流畅。

### 用真实的文件夹整理

侧栏按顶层文件夹分组。打开一个文件夹，它的页面把子文件夹显示成一叠纸，里面有什么一眼就能看到；列表视图可以按任意一列排序。电脑上的任何文件夹都能用「添加文件夹」加进来，原地使用。把笔记拖到文件夹上停一下，文件夹会自己展开；文件夹里还能再放文件夹，页面和文件夹也能移到另一个顶层文件夹，每次移动都可以撤销。笔记可以置顶到侧栏最上面；点搜索图标或按 `⌘ J` 搜索所有标题和正文，在笔记里查找替换，也可以用正则表达式。删掉的笔记先进回收站，保留几天由你定。单独打开的 Markdown 文件在自己的窗口里原地编辑。

### 带上旧笔记

「从其他应用导入…」（在「文件」菜单里）会找到这台电脑上的 Obsidian、Logseq、iA Writer、Typora 笔记库和思源笔记工作空间并复制进来，原文件不会被改动。Obsidian 的 `[[双链]]` 照样能用，点一下就打开它指向的笔记。之后也可以把 Markdown 文件和文件夹拖到文件夹页，或者直接拖进侧栏的某个文件夹。笔记也能导出成 Markdown 或 PDF，或者直接打印。

### 还有

- **版本历史**：每篇笔记保存时都会留下快照（默认 20 份，最多 30 份）。点开一份就在原处看到改了什么，一键恢复，也可以给它起个名字。
- **快速笔记**：在 macOS 上，全局快捷键（默认 `⌥ Space`）在任何应用上方打开一个小笔记窗口。
- **外观**：内置 Craft、飞书、Notion、Obsidian 四种风格，每种都有由它的配色推算出的深色效果，也可以用 CSS 自己写（见 [docs/THEMES.md](docs/THEMES.md)）。界面字体和文档字体都能选。
- **快捷键**：按 `⌘ /` 查看全部快捷键，每一个都能改，也能清除。在 Windows 和 Linux 上，Ctrl 代替 ⌘，Alt 代替 ⌥。
- **命令行和 Agent**：随应用附带的命令行工具 `quiet` 让 Claude Code、Codex 等 AI Agent 读写你的文档库，和你用的是同一个引擎。「设置 › 连接 Agent」给出一句话发给 Agent，它会自己装好；macOS 上一键安装命令行工具。详见[英文 README](README.md#work-with-agents)。
- **语言**：英文、简体中文、繁体中文、日文、韩文、西班牙文、法文、阿拉伯文和土耳其文。阿拉伯文正确连写，从右往左排。
- **清理**：「设置 › 文档库」能找出已经没有笔记在用的图片和文件，确认后才移出。
- **更新**：新版本在应用内安装，安装前会校验签名。自动检查可以关闭。

## 路线图

- [x] 富文本文档：标题、列表、高亮块、表格、分栏、代码、公式、图片和附件
- [x] Markdown 文件，原地打开、原地编辑
- [x] 画板：手绘风格的图示和草图，可以放进笔记，也可以单独存在
- [x] 原地打开任意文件夹，在侧栏里并排显示
- [x] 从 Obsidian、Logseq、思源笔记、Typora、iA Writer 和任意 Markdown 文件夹导入
- [x] 写作模式：专注模式、打字机模式和段落聚焦
- [x] 主题、深色模式和九种界面语言
- [x] 给 AI Agent 用的 `quiet` 命令行工具和 skill
- [ ] 数据库（进行中）
- [ ] Mermaid 图表（进行中）
- [ ] 移动端：iPhone、iPad 和 Android（进行中）
- [ ] 多设备同步（进行中）
- [ ] 文档属性
- [ ] 网页剪藏

## 下载

<p align="center">
  <a href="https://download.quiet.md/latest/quiet-md-macos-arm64.dmg"><img src="docs/images/download-macos.svg" height="56" alt="下载 macOS 版"></a>
  &nbsp;
  <a href="https://download.quiet.md/latest/quiet-md-windows-x64-setup.exe"><img src="docs/images/download-windows.svg" height="56" alt="下载 Windows 版"></a>
  &nbsp;
  <a href="https://download.quiet.md/latest/quiet-md-x86_64.AppImage"><img src="docs/images/download-linux.svg" height="56" alt="下载 Linux 版"></a>
</p>

| 平台 | 安装包 | 系统要求 |
|---|---|---|
| **macOS**（Apple Silicon） | [quiet-md-macos-arm64.dmg](https://download.quiet.md/latest/quiet-md-macos-arm64.dmg) | macOS 13 及以上 |
| **Windows**（x64） | [quiet-md-windows-x64-setup.exe](https://download.quiet.md/latest/quiet-md-windows-x64-setup.exe) | Windows 10 1809 及以上 |
| **Linux**（x86_64，实验性） | [quiet-md-x86_64.AppImage](https://download.quiet.md/latest/quiet-md-x86_64.AppImage) | glibc 2.35 及以上（Ubuntu 22.04 及更新） |
| **macOS**（Intel） | [quiet-md-macos-x86_64.dmg](https://download.quiet.md/latest/quiet-md-macos-x86_64.dmg) | macOS 13 及以上 |

- **macOS**：打开 `.dmg`，把 `quiet.md.app` 拖到旁边的「应用程序」快捷方式上，再从「应用程序」打开。应用和磁盘映像都已签名并通过 Apple 公证。Releases 页面上的 `.zip` 是同一个应用，供应用内更新下载。
- **Windows**：运行安装程序，默认只装到当前用户目录，卸载不会删除笔记。如果首次运行出现 SmartScreen 提示，点「更多信息 → 仍要运行」即可。
- **Linux**（实验性）：`chmod +x quiet-md-*.AppImage` 后直接运行；没有 FUSE 时加 `--appimage-extract-and-run`。AppImage 从 0.1.10 起支持应用内更新；更早的 Linux 版本需先手动安装本版。目前只在 Omarchy（Wayland）上做过交互测试，详见 [Linux 说明](CONTRIBUTING.md#linux-experimental)。

也可以在官网 [Quiet.md](https://quiet.md) 下载。所有版本和 SHA-256 校验值见 [GitHub Releases](https://github.com/s1dashu/quiet-md/releases)。

首次启动会直接打开「文稿」里的 **Quiet Notes** 文件夹（以前的版本建过 `Quiet.md` 文件夹的，就用它），里面放着一套「开始指南」，十分钟带你过一遍主要功能；已有的笔记不会被覆盖。别的文件夹、其他应用里的笔记，随时可以从「文件」菜单添加或导入。

## 性能

在一篇 **100 万字**、549 章的混合文档里连续操作 100 次，统计每帧处理耗时：

| 操作 | P50 | P95 |
|---|---:|---:|
| ⌨️ 连续输入 | **5.06 ms** | 5.78 ms |
| ⌫ 连续退格 | **4.99 ms** | 5.55 ms |
| 🖱️ 连续滚动 | **2.08 ms** | 4.14 ms |

120 Hz 屏幕每帧的预算是 8.3 ms，百万字文档下的 P95 也留有余量。

| | |
|---|---|
| macOS 安装包 | 约 18 MB |
| Windows 安装包 | 约 14 MB |
| 运行时 | 无 WebView、无 Electron、无后台服务 |

<sub>Apple M5 Pro / 48 GiB，macOS 26.5，Release 构建。测试方法和复现脚本见 [docs/PERFORMANCE.md](docs/PERFORMANCE.md)。</sub>

## 贡献者

感谢每一位参与 Quiet.md 的人。欢迎提 issue 和 PR，构建、测试和代码约定见 [CONTRIBUTING.md](CONTRIBUTING.md)。

[![Contributors](https://raw.githubusercontent.com/s1dashu/quiet-md/contributors-assets/docs/images/contributors.svg?v=1)](https://github.com/s1dashu/quiet-md/graphs/contributors)

## 交流群

微信用户可以扫描下方二维码，加入 Quiet.md 用户群。Discord 社区也即将上线。

<img src="docs/images/wechat-group.webp" alt="微信用户群" width="200">

## 近期更新

- **0.2.0**（11 月 28 日）：界面换成统一的设计系统并大幅简化，数据库正式开放，新增三种写作模式，以及能让 AI Agent 直接使用的命令行工具。
  - **文件夹就是最上层**：不再有单独的主页，侧栏按顶层文件夹分组，标题可以折叠，标题的「…」里有批量管理、在访达中显示和从侧栏移除。「添加文件夹」就地使用文件夹。「我的文档」和「本机文件」不再单独列出：单独打开的 Markdown 文件在自己的窗口里编辑，以前打开过的本机文件夹自动成为顶层文件夹。页面和文件夹可以拖到、或用「移动到」移到另一个顶层文件夹，图片、链接和版本历史一起带过去，可以撤销。收藏并入置顶。
  - **统一的设计系统**：菜单、按钮、对话框、开关、提示和滚动条换成参照 shadcn/ui 的同一套组件；macOS 上改为连续圆角（超椭圆），弹出层是细环加柔和阴影，阴影统一成一套层级，图标统一换成 Nucleo。菜单放不下时可以滚动，鼠标移向子菜单时不会误触经过的行。每一页右上角都是「+ · 搜索 · …」（文件夹页多一个「显示」），面包屑按大写字母的实际高度和红绿灯对齐。
  - **数据库**：可以从新建菜单、`/` 菜单或 CSV 新建；有文本、单选、多选、日期、数字、公式、查找引用、关联等字段，可以筛选、排序、分组、按条件填色，行高可选「适应内容」。「笔记」列的每个单元格都是文档库里的一篇真实笔记。
  - **写作模式**：都在文档的「…」菜单里。「专注模式」（⌘.）让窗口里只留下文档；「打字机模式」（⌘⇧T）让正在写的一行停在窗口中间；「段落聚焦」（⌘⇧F）让正在写的段落保持清晰，其余变淡。「显示」里的放大 / 缩小（⌘+ / ⌘−）只缩放正文，标题、表格、代码和公式一起缩放。
  - **命令行和 Agent**：新增命令行工具 `quiet`，可以在终端里列出、搜索、阅读和编辑笔记和数据库，增删侧栏里的文件夹，还能在应用里打开文档并定位到某个块。设置 › 连接 Agent 一条命令就能让 Claude Code、Codex 等 Agent 配置好；`quiet` 随应用一起安装并加入 PATH（macOS 上点「安装命令行工具」，Windows 由安装程序完成）。
  - **更清晰的深色模式**：链接、文字颜色、高亮、提醒日期、倒计时和按钮文字的对比度都调到清晰可读，侧栏更亮；⌃⌘D 切换深色和浅色。
  - **还有**：子文档回来了；Windows 和 Linux 上从 Word、浏览器、飞书粘贴也保留格式；可以导入思源笔记，「从其他应用导入…」一处列出思源、Obsidian、Typora 和 Markdown 文件夹；回收站可以在 1 到 30 天后自动删除；每个快捷键都可以清除；「在新窗口中打开」跟随主窗口的大小和侧栏；重命名、移动文件夹立即显示，文件在后台写入；首次启动直接打开「文稿」里的「Quiet Notes」和重新分组的开始指南，附带 Markdown、画板和数据库的示例文件；匿名的每日使用计数，可以在 设置 › 通用 里关闭。
  - **文档库格式**：打开文档库时会升级到 FORMAT 6，升级后 0.1.14 及更早的版本不能再打开它。放在 iCloud 等同步文件夹里的文档库暂时不升级。
- **0.1.14**（11 月 22 日）：查找替换支持正则表达式；Obsidian 的 `[[双链]]` 可以点击跳转；新增土耳其语界面，阿拉伯语从右往左排版；可以选择界面和文档字体，清理没有用到的图片和文件；大文档库搜索和批量删除更快。
- **0.1.13**（11 月 17 日）：新增画板，按 Excalidraw 移植，可以插在文档里或直接打开 `.excalidraw` 文件；每篇文档都有版本历史；新增法语和阿拉伯语界面；复制为 Markdown；从网页粘贴的图片存进文档库。
- **0.1.12**（11 月 14 日）：文件可以从访达直接拖进侧栏的文件夹；主页卡片改为连续圆角；新增「在访达中打开」；窗口最小宽度给文档区留足空间。
- **0.1.11**（11 月 13 日）：文档库改为真实文件夹，文件以标题命名，旧文档库自动迁移并留有备份；全新的首次启动，可从 Obsidian 导入；新增日语、西班牙语和韩语界面；Markdown 支持块级公式；启动更快、内存更省。
- **0.1.10**（11 月 10 日）：新建菜单可选择文档类型；宽表格横向滚动；Markdown 表格保存列对齐。
- **0.1.9**（11 月 8 日）：修复 macOS ⌘Q、Windows 窗口按钮和背景色遮挡选区的问题。
- **0.1.8**（11 月 9 日）：本机 Markdown 原地编辑；多级文件夹与拖动、撤销；图片更省内存；实验性 Linux AppImage。
- **0.1.7**（11 月 6 日）：加固多窗口保存和冲突恢复；Markdown 导出保留行内附件。
- **0.1.6**（11 月 6 日）：输入和切换窗口更顺，表格更宽，macOS 快速笔记改为 ⌥ Space；Windows 支持粘贴图片、导出 PDF 和打印。
- **0.1.5**（11 月 5 日）：更名为 Quiet.md；批量管理多篇文档；回收站一键清空。
- **0.1.4**（11 月 5 日）：应用内自动更新，校验签名；所有主题都有深色模式。
- **0.1.3**（11 月 4 日）：macOS 深色模式下侧栏文字清晰可读。
- **0.1.2**（11 月 4 日）：补齐 macOS 标准菜单和快捷键；点 Dock 图标可重新打开窗口。
- **0.1.1**（11 月 3 日）：推出 Windows x64 版。

每个版本的完整说明见 [Releases](https://github.com/s1dashu/quiet-md/releases)。

## 许可

代码使用 [MIT](LICENSE) 许可。第三方代码、字体和素材见 [THIRD_PARTY.md](THIRD_PARTY.md)。

以下内容不在 MIT 范围内，fork 时请替换：

- `assets/icons/` 里的 82 个和 `assets/editor-icons/` 里的 24 个 Nucleo 图标使用 Nucleo 商业授权，请勿单独提取再分发；
- 「Quiet.md」这个名字和应用图标。

## Star History

<a href="https://www.star-history.com/#s1dashu/quiet-md&type=date">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=s1dashu/quiet-md&type=date&theme=dark" />
    <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=s1dashu/quiet-md&type=date" />
  </picture>
</a>
