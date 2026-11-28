<h1 align="center">Quiet.md</h1>

<p align="center">
  <b>又快又安靜的筆記，就放在你自己的電腦上。</b><br>
  以 Rust + GPUI 原生繪製的筆記應用程式：真實的資料夾、輸入 Markdown 即自動排版，筆記裡還有一塊白板。免費且開源。
</p>

<p align="center">
  <a href="README.md">English</a> · <a href="README.zh-CN.md">简体中文</a> · 繁體中文 · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a>
</p>

<p align="center">
  <a href="https://github.com/s1dashu/quiet-md/releases/latest"><img src="https://img.shields.io/github/v/release/s1dashu/quiet-md?label=release&color=2f6feb" alt="最新版本"></a>
  <a href="https://github.com/s1dashu/quiet-md/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/s1dashu/quiet-md/ci.yml?branch=main&event=schedule&label=CI" alt="CI 狀態"></a>
  <img src="https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey" alt="macOS、Windows 與 Linux">
  <img src="https://img.shields.io/badge/built%20with-Rust%20%2B%20GPUI-dea584?logo=rust" alt="以 Rust 與 GPUI 打造">
  <a href="LICENSE"><img src="https://img.shields.io/github/license/s1dashu/quiet-md?color=green" alt="MIT 授權條款"></a>
  <img src="https://img.shields.io/badge/status-beta-orange" alt="Beta 測試版">
</p>

<p align="center">
  <a href="https://download.quiet.md/latest/quiet-md-macos-arm64.dmg"><img src="docs/images/download-macos.svg" height="56" alt="下載 macOS 版"></a>
  &nbsp;
  <a href="https://download.quiet.md/latest/quiet-md-windows-x64-setup.exe"><img src="docs/images/download-windows.svg" height="56" alt="下載 Windows 版"></a>
  &nbsp;
  <a href="https://download.quiet.md/latest/quiet-md-x86_64.AppImage"><img src="docs/images/download-linux.svg" height="56" alt="下載 Linux 版"></a>
</p>

<p align="center">
  <a href="https://quiet.md/?lang=zh-Hant"><b>官方網站</b></a> ·
  <a href="#下載"><b>下載</b></a> ·
  <a href="#主要功能">主要功能</a> ·
  <a href="#效能表現">效能表現</a> ·
  <a href="CONTRIBUTING.md">參與貢獻</a>
</p>

<p align="center">
  <img src="docs/images/readme-home.webp" width="100%" alt="像素風草坡背景上的 Quiet.md：左側側邊欄列出資料夾，頂層資料夾頁將每個資料夾顯示成一疊附圖的筆記卡片。">
</p>

> [!NOTE]
> Quiet.md 目前處於 **beta 測試階段**，版本迭代相當迅速。如果你遇到臭蟲或有任何想法，歡迎[回報 Issue](https://github.com/s1dashu/quiet-md/issues)，更歡迎直接送出 Pull Request。

## 為什麼選擇 Quiet.md

**📁 筆記就是一般檔案。** 不需要帳號，沒有雲端，預設不連網（以下除外：可選的更新檢查；應用程式和命令列工具 `quiet` 每天傳送一次匿名使用次數，只有計數，例如應用程式開了幾次，可在「設定 › 一般 › 匿名使用統計」關閉；白板第一次用到中日韓手寫字型、或者你選擇可下載的文件字型時，會下載對應的字型檔）。側邊欄裡的每個資料夾都是磁碟上真實的資料夾，每篇筆記都是以標題命名的檔案：Markdown 為 `.md`，富文字文件為 `.xml`。用 iCloud、Git 或任何同步工具都可以；哪天不再使用 Quiet.md，檔案也還留在原處。

**⚡ 原生，而且快。** 整個介面由 Rust 與 [GPUI](https://www.gpui.rs/) 直接以 GPU 繪製，不是 Electron，也不是網頁套殼。在一篇 100 萬字的筆記裡打字，每格畫面約 5 ms；120 Hz 螢幕給每格畫面的時間是 8.3 ms。

**✍️ 寫字與畫圖，在同一個地方。** 輸入 `#`、`-`、`[]` 或 `>`，Markdown 就會自動排版。文字說不清楚時，直接在筆記裡放一塊白板。富文字筆記以 Feishu（Lark）DocxXML 格式儲存，從 Feishu 或網頁貼上都能保留結構。

## 主要功能

### 書寫

<p align="center">
  <img src="docs/images/readme-write.webp" width="100%" alt="在 Quiet.md 開啟的一篇長筆記，包含標題、行內公式與一張離子濃度表。">
</p>

標題、清單、待辦事項、引文、提示框、分欄、可合併儲存格的表格、具語法醒目提示的程式碼、LaTeX 數學公式、圖片、附件與封面。邊寫邊用 Markdown 快速輸入，其餘的都在 `/` 選單裡。提及另一篇筆記即可建立連結，對方會列出反向引用。資料庫以單選、日期、數字、公式、查閱、關聯等欄位記錄資訊，檢視可以篩選、排序、分組，「筆記」欄的每一格都是一篇真實的筆記。「…」選單裡有專注模式、打字機模式和段落聚焦，「顯示」›「放大」只放大內文。

### 繪圖

<p align="center">
  <img src="docs/images/readme-board.webp" width="100%" alt="Quiet.md 裡的一塊白板：圖片與地圖排在一起，搭配手寫標註、方框與連接箭頭。">
</p>

白板移植自 Excalidraw：幾何圖形、始終相連的箭頭、手繪線條、文字、便利貼、畫框與圖片。可以放進筆記裡，也可以存成獨立的 `.excalidraw` 檔案，excalidraw.com 能直接開啟。用觸控式軌跡板雙指捏合即可縮放；一萬個圖形的白板依然流暢。

### 用真實的資料夾整理

側邊欄依頂層資料夾分組。打開一個資料夾，它的頁面會把子資料夾顯示成一疊紙，裡面有什麼一目了然；清單檢視可以依任一欄排序。電腦上的任何資料夾都能用「新增資料夾」加入，就地使用。把筆記拖到資料夾上稍候片刻，資料夾就會自動展開；資料夾裡還能再放資料夾，頁面和資料夾也能移到另一個頂層資料夾，每次移動都能復原。筆記可以釘選到側邊欄最上方；點搜尋圖示或按 `⌘ J` 搜尋所有標題與內文，在筆記裡尋找與取代，也可以使用規則運算式。刪除的筆記會先移到垃圾桶，保留幾天由你決定。單獨開啟的 Markdown 檔案在自己的視窗裡就地編輯。

### 帶上舊筆記

「從其他應用程式匯入…」（在「檔案」選單裡）會找到這台電腦上的 Obsidian、Logseq、iA Writer、Typora 筆記庫與思源筆記工作空間並複製進來，原始檔案不會被更動。Obsidian 的 `[[雙向連結]]` 照樣能用，點一下就會開啟它指向的筆記。之後也可以把 Markdown 檔案和資料夾拖到資料夾頁，或直接拖進側邊欄的某個資料夾。筆記也能匯出為 Markdown 或 PDF，或直接列印。

### 還有更多

- **版本紀錄**：每篇筆記在儲存時都會留下快照（預設 20 版，最多 30 版）。點開任一版就能在原處看到改了什麼，一鍵還原，也能為它命名。
- **快速筆記**：在 macOS 上，全域快速鍵（預設 `⌥ Space`）會在任何應用程式上方開啟一個小筆記視窗。
- **外觀**：內建 Craft、Feishu、Notion 與 Obsidian 四套樣式，每套皆有深色版本，也可以用 CSS 自行撰寫（見 [docs/THEMES.md](docs/THEMES.md)）。介面字型與文件字型都能選擇。
- **快速鍵**：按下 `⌘ /` 檢視所有快速鍵，每一組都能重新設定，也能清除。在 Windows 與 Linux 上，以 Ctrl 對應 ⌘、Alt 對應 ⌥。
- **命令列與 Agent**：隨應用程式附帶的命令列工具 `quiet` 讓 Claude Code、Codex 等 AI Agent 讀寫你的文件庫，和你用的是同一個引擎。「設定 › 連接 Agent」提供一句話傳給 Agent，它會自己安裝好；macOS 上一鍵安裝命令列工具。詳見[英文 README](README.md#work-with-agents)。
- **語言**：英文、簡體中文、繁體中文、日文、韓文、西班牙文、法文、阿拉伯文與土耳其文。阿拉伯文會正確連寫，由右至左排列。
- **清理**：「設定 › 文件」能找出不再有任何筆記使用的圖片與檔案，確認後才會移出。
- **更新**：新版本在應用程式內安裝，安裝前會驗證簽章。自動檢查可以關閉。

## 路線圖

- [x] 富文字文件：標題、清單、提示框、表格、分欄、程式碼、數學公式、圖片與附件
- [x] Markdown 檔案，就地開啟、就地編輯
- [x] 白板：手繪風格的圖解與草圖，可以放進筆記，也可以獨立存在
- [x] 就地開啟任何資料夾，在側邊欄裡並排顯示
- [x] 從 Obsidian、Logseq、思源筆記、Typora、iA Writer 與任何 Markdown 資料夾匯入
- [x] 寫作模式：專注模式、打字機模式和段落聚焦
- [x] 主題、深色模式與九種介面語言
- [x] 給 AI Agent 使用的 `quiet` 命令列工具與 skill
- [ ] 資料庫（進行中）
- [ ] Mermaid 圖表（進行中）
- [ ] 行動裝置：iPhone、iPad 與 Android（進行中）
- [ ] 跨裝置同步（進行中）
- [ ] 文件屬性
- [ ] 網頁剪輯

## 下載

<p align="center">
  <a href="https://download.quiet.md/latest/quiet-md-macos-arm64.dmg"><img src="docs/images/download-macos.svg" height="56" alt="下載 macOS 版"></a>
  &nbsp;
  <a href="https://download.quiet.md/latest/quiet-md-windows-x64-setup.exe"><img src="docs/images/download-windows.svg" height="56" alt="下載 Windows 版"></a>
  &nbsp;
  <a href="https://download.quiet.md/latest/quiet-md-x86_64.AppImage"><img src="docs/images/download-linux.svg" height="56" alt="下載 Linux 版"></a>
</p>

| 平台 | 安裝套件 | 系統需求 |
|---|---|---|
| **macOS**（Apple Silicon） | [quiet-md-macos-arm64.dmg](https://download.quiet.md/latest/quiet-md-macos-arm64.dmg) | macOS 13 以上版本 |
| **Windows**（x64） | [quiet-md-windows-x64-setup.exe](https://download.quiet.md/latest/quiet-md-windows-x64-setup.exe) | Windows 10 1809 以上版本 |
| **Linux**（x86_64，實驗性） | [quiet-md-x86_64.AppImage](https://download.quiet.md/latest/quiet-md-x86_64.AppImage) | glibc 2.35 以上版本（Ubuntu 22.04 及更新版本） |
| **macOS**（Intel） | [quiet-md-macos-x86_64.dmg](https://download.quiet.md/latest/quiet-md-macos-x86_64.dmg) | macOS 13 以上版本 |

- **macOS**：打開 `.dmg`，將 `quiet.md.app` 拖到旁邊的「應用程式」捷徑上，再從「應用程式」打開。應用程式與磁碟映像都已完成 Apple 簽章與公證。Releases 頁面上的 `.zip` 是同一個應用程式，供應用程式內更新下載。
- **Windows**：執行安裝程式，預設僅為目前使用者安裝，解除安裝時絕不會刪除你的筆記。若首次啟動出現 SmartScreen 提示，請點選**其他資訊 → 仍要執行**。
- **Linux**（實驗性）：執行 `chmod +x quiet-md-*.AppImage` 後即可啟動，若系統未安裝 FUSE 則加上 `--appimage-extract-and-run` 參數。AppImage 0.1.10 以上版本支援應用程式內更新，較舊的 Linux 版本需手動重新安裝一次，目前已在 Wayland 環境下的 Omarchy 完成互動實測，詳情請參閱 [Linux 說明](CONTRIBUTING.md#linux-experimental)。

你也可以前往 [Quiet.md](https://quiet.md/?lang=zh-Hant) 官方網站下載，所有發行版本與對應的 SHA-256 雜湊值皆公布於 [GitHub Releases](https://github.com/s1dashu/quiet-md/releases)。

首次啟動會直接開啟「文件」裡的 **Quiet Notes** 資料夾（先前版本建過 `Quiet.md` 資料夾的，就沿用它），裡面附有一份 **Getting started** 入門指南，帶你在十分鐘內逛完主要功能，且絕不會覆寫任何既有筆記。其他資料夾或其他應用程式裡的筆記，隨時都能從「檔案」選單加入或匯入。

## 效能表現

在包含 **100 萬字**與 549 個章節的混合內容文件中，連續執行 100 次操作的單格畫面處理耗時如下：

| 操作項目 | P50 | P95 |
|---|---:|---:|
| ⌨️ 打字輸入 | **5.06 ms** | 5.78 ms |
| ⌫ 倒退刪除 | **4.99 ms** | 5.55 ms |
| 🖱️ 畫面捲動 | **2.08 ms** | 4.14 ms |

120 Hz 螢幕每一格畫面的時間預算為 8.3 ms，即使面對百萬字長文的 P95 峰值，依然保有充裕餘裕。

| | |
|---|---|
| macOS 下載大小 | ~18 MB |
| Windows 安裝檔 | ~14 MB |
| 執行時期環境 | 無 WebView、無 Electron、無背景常駐服務 |

<sub>測試環境：Apple M5 Pro / 48 GiB，macOS 26.5，release 建置版本。測試方法與重現腳本請見 [docs/PERFORMANCE.md](docs/PERFORMANCE.md)。</sub>

## 參與貢獻

感謝每一位協助打造 Quiet.md 的貢獻者。歡迎提交 Issue 與 Pull Request，建置流程、測試方式與程式碼規範請參閱 [CONTRIBUTING.md](CONTRIBUTING.md)。

[![Contributors](https://raw.githubusercontent.com/s1dashu/quiet-md/contributors-assets/docs/images/contributors.svg?v=1)](https://github.com/s1dashu/quiet-md/graphs/contributors)

## 交流群

微信使用者可以掃描下方 QR Code，加入 Quiet.md 使用者群組。Discord 社群也即將推出。

<img src="docs/images/wechat-group.webp" alt="微信使用者群組" width="200">

## 近期更新

- **0.2.0**（11 月 28 日）：介面改用統一的設計系統並大幅簡化，資料庫正式開放，新增三種寫作模式，以及能讓 AI Agent 直接使用的命令列工具。
  - **資料夾就是最上層**：不再有獨立的首頁，側邊欄依頂層資料夾分組，標題可以收合，標題的「…」裡有管理、在 Finder 中顯示和從側邊欄移除。「新增資料夾」就地使用資料夾。「我的文件」和「本機檔案」不再單獨列出：單獨開啟的 Markdown 檔案會在自己的視窗裡編輯，以前開啟過的本機資料夾自動成為頂層資料夾。頁面和資料夾可以拖曳到、或用「移動至」移到另一個頂層資料夾，圖片、連結和版本紀錄會一起帶過去，也可以復原。收藏併入釘選。
  - **統一的設計系統**：選單、按鈕、對話框、開關、提示和捲軸改用參照 shadcn/ui 的同一套元件；macOS 上改為連續圓角（超橢圓），彈出層是細框線加柔和陰影，陰影統一成一套層級，圖示統一換成 Nucleo。選單放不下時可以捲動，滑鼠移向子選單時不會誤觸經過的列。每一頁右上角都是「+ · 搜尋 · …」（資料夾頁多一個「顯示」），麵包屑導覽依大寫字母的實際高度與紅綠燈對齊。
  - **資料庫**：可以從「新增」選單、`/` 選單或 CSV 建立；有文字、單選、多選、日期、數字、公式、查詢參照、關聯等欄位，可以篩選、排序、分組、依條件填色，列高可選「適應內容」。「筆記」欄的每個儲存格都是文件庫裡的一篇真實筆記。
  - **寫作模式**：都在文件的「…」選單裡。「專注模式」（⌘.）讓視窗裡只留下文件；「打字機模式」（⌘⇧T）讓正在寫的那一行停在視窗中間；「段落聚焦」（⌘⇧F）讓正在寫的段落保持清晰，其餘變淡。「顯示」裡的放大 / 縮小（⌘+ / ⌘−）只縮放內文，標題、表格、程式碼和方程式一起縮放。
  - **命令列與 Agent**：新增命令列工具 `quiet`，可以在終端機裡列出、搜尋、閱讀和編輯筆記與資料庫，增刪側邊欄裡的資料夾，還能在應用程式裡開啟文件並定位到某個區塊。在 設定 › 連接 Agent 一行指令就能為 Claude Code、Codex 等 Agent 完成設定；`quiet` 隨應用程式一起安裝並加入 PATH（macOS 上點「安裝命令列工具」，Windows 由安裝程式完成）。
  - **更清晰的深色模式**：連結、文字顏色、螢光標記、提醒日期、倒數計時和按鈕文字的對比度都調到清晰可讀，側邊欄更亮；⌃⌘D 切換深色與淺色。
  - **還有**：子頁面回來了；Windows 和 Linux 上從 Word、瀏覽器、飛書貼上也保留格式；可以匯入思源筆記，「從其他應用程式匯入…」一處列出思源、Obsidian、Typora 和 Markdown 資料夾；垃圾桶可以在 1 到 30 天後自動清空；每個快速鍵都可以清除；「在新視窗中開啟」跟隨主視窗的大小與側邊欄；重新命名、移動資料夾會立即顯示，檔案在背景寫入；首次啟動直接開啟「文件」裡的「Quiet Notes」和重新分組的入門指南，附帶 Markdown、白板和資料庫的範例檔案；匿名的每日使用次數統計，可以在 設定 › 一般 裡關閉。
  - **文件庫格式**：開啟文件庫時會升級到 FORMAT 6，升級後 0.1.14 及更早的版本無法再開啟它。放在 iCloud 等同步資料夾裡的文件庫暫不升級。
- **0.1.14**（11 月 22 日）：尋找與取代支援規則運算式；Obsidian 的 `[[雙向連結]]` 可以點擊跳轉；新增土耳其文介面，阿拉伯文由右至左排版；可以選擇介面與文件字型，清理沒有用到的圖片和檔案；大型文件庫的搜尋與批次刪除更快。
- **0.1.13**（11 月 17 日）：新增白板，移植自 Excalidraw，可以插在筆記裡或直接開啟 `.excalidraw` 檔案；每篇筆記都有版本紀錄；新增法文與阿拉伯文介面；複製為 Markdown；從網頁貼上的圖片會存進文件庫。
- **0.1.12**（11 月 14 日）：檔案可以從 Finder 直接拖進側邊欄的資料夾；首頁卡片改為連續圓角；新增「在 Finder 中開啟」；視窗最小寬度為筆記區保留足夠空間。
- **0.1.11**（11 月 13 日）：文件庫改為真實資料夾，檔案以標題命名，舊文件庫自動遷移並保留備份；全新的首次啟動，可從 Obsidian 匯入；新增日文、西班牙文與韓文介面；Markdown 支援區塊數學公式；啟動更快、記憶體更省。
- **0.1.10**（11 月 10 日）：「新增」選單可選擇文件類型；寬表格可水平捲動；Markdown 表格保留欄位對齊。
- **0.1.9**（11 月 8 日）：修正 macOS 上的 Cmd+Q、Windows 視窗按鈕，以及選取範圍被螢光筆底色遮蓋的問題。
- **0.1.8**（11 月 9 日）：原位編輯本機 Markdown 檔案；多層資料夾，支援拖曳與復原；圖片更省記憶體；實驗性 Linux AppImage。
- **0.1.7**（11 月 6 日）：強化多視窗儲存與衝突復原；匯出 Markdown 時保留行內附件。
- **0.1.6**（11 月 6 日）：打字與切換視窗更順暢，表格更寬，macOS 快速筆記改用 ⌥ Space；Windows 支援貼上圖片、匯出 PDF 與列印。
- **0.1.5**（11 月 5 日）：更名為 Quiet.md；一次管理多份文件；一鍵清空垃圾桶。
- **0.1.4**（11 月 5 日）：應用程式內更新，並驗證簽章；所有主題都有深色模式。
- **0.1.3**（11 月 4 日）：macOS 深色模式下側邊欄文字清晰可讀。
- **0.1.2**（11 月 4 日）：補齊標準 macOS 選單與快速鍵；點選 Dock 圖示可重新開啟視窗。
- **0.1.1**（11 月 3 日）：首個 Windows x64 發行版。

各版本完整更新紀錄請見 [Releases](https://github.com/s1dashu/quiet-md/releases)。

## 授權條款

程式碼以 [MIT](LICENSE) 授權條款釋出，第三方程式碼、字型與素材清單詳列於 [THIRD_PARTY.md](THIRD_PARTY.md)。

以下項目不包含在 MIT 授權範圍內，建立衍生分支（Fork）時請自行替換：

- `assets/icons/` 目錄下的 82 個和 `assets/editor-icons/` 目錄下的 24 個 Nucleo 圖示受商業 Nucleo 授權保護，請勿單獨抽離或重新散佈。
- 「Quiet.md」名稱與應用程式圖示。

## Star History

<a href="https://www.star-history.com/#s1dashu/quiet-md&type=date">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=s1dashu/quiet-md&type=date&theme=dark" />
    <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=s1dashu/quiet-md&type=date" />
  </picture>
</a>
