<h1 align="center">Quiet.md</h1>

<p align="center">
  <b>速くて静かな、あなたのコンピューターに置くノート。</b><br>
  Rust + GPUI でネイティブに描画するノートアプリです。本物のフォルダ、入力するそばから整う Markdown、そしてノートの中のホワイトボード。無料のオープンソースです。
</p>

<p align="center">
  <a href="README.md">English</a> · <a href="README.zh-CN.md">简体中文</a> · <a href="README.zh-TW.md">繁體中文</a> · 日本語 · <a href="README.ko.md">한국어</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a>
</p>

<p align="center">
  <a href="https://github.com/s1dashu/quiet-md/releases/latest"><img src="https://img.shields.io/github/v/release/s1dashu/quiet-md?label=release&color=2f6feb" alt="最新リリース"></a>
  <a href="https://github.com/s1dashu/quiet-md/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/s1dashu/quiet-md/ci.yml?branch=main&event=schedule&label=CI" alt="CI のステータス"></a>
  <img src="https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey" alt="macOS、Windows、Linux">
  <img src="https://img.shields.io/badge/built%20with-Rust%20%2B%20GPUI-dea584?logo=rust" alt="Rust と GPUI で構築">
  <a href="LICENSE"><img src="https://img.shields.io/github/license/s1dashu/quiet-md?color=green" alt="MIT ライセンス"></a>
  <img src="https://img.shields.io/badge/status-beta-orange" alt="ベータ版">
</p>

<p align="center">
  <a href="https://download.quiet.md/latest/quiet-md-macos-arm64.dmg"><img src="docs/images/download-macos.svg" height="56" alt="macOS 版をダウンロード"></a>
  &nbsp;
  <a href="https://download.quiet.md/latest/quiet-md-windows-x64-setup.exe"><img src="docs/images/download-windows.svg" height="56" alt="Windows 版をダウンロード"></a>
  &nbsp;
  <a href="https://download.quiet.md/latest/quiet-md-x86_64.AppImage"><img src="docs/images/download-linux.svg" height="56" alt="Linux 版をダウンロード"></a>
</p>

<p align="center">
  <a href="https://quiet.md/?lang=en"><b>Web サイト</b></a> ·
  <a href="#ダウンロード"><b>ダウンロード</b></a> ·
  <a href="#機能">機能</a> ·
  <a href="#パフォーマンス">パフォーマンス</a> ·
  <a href="CONTRIBUTING.md">コントリビュート</a>
</p>

<p align="center">
  <img src="docs/images/readme-home.webp" width="100%" alt="ドット絵の草原を背景にした Quiet.md。左のサイドバーにフォルダが並び、最上位フォルダのページでは各フォルダが画像入りのノートカードの束として表示されています。">
</p>

> [!NOTE]
> Quiet.md は**ベータ版**で、急速に変化しています。バグを見つけたり、アイデアがあったりしたら、[issue を作成](https://github.com/s1dashu/quiet-md/issues)してください。プルリクエストを送ってもらえると、もっとうれしいです。

## Quiet.md を選ぶ理由

**📁 ノートはただのファイル。** アカウントもクラウドも不要で、既定ではネットワークに何も送りません（任意のアップデート確認、アプリとコマンドラインツール `quiet` が 1 日 1 回送る匿名の使用回数（アプリを開いた回数などの回数だけで、「設定 › 一般 › 匿名の使用状況統計」でオフにできます）、ホワイトボードで初めて中国語・日本語・韓国語の手書きフォントが必要になったとき、またはダウンロード可能なドキュメントフォントを選んだときのフォントのダウンロードを除く）。サイドバーのフォルダはどれもディスク上の本物のフォルダで、ノートはそれぞれタイトル名のファイルです。Markdown は `.md`、リッチなドキュメントは `.xml`。iCloud でも Git でも、好きな方法で同期できます。Quiet.md を使わなくなっても、ファイルはそのまま残ります。

**⚡ ネイティブで、速い。** インターフェース全体を Rust と [GPUI](https://www.gpui.rs/) が GPU で描画します。Electron も WebView も使いません。100 万文字のノートでも、入力は 1 フレームあたり約 5 ms。120 Hz のディスプレイが 1 フレームに使える時間は 8.3 ms です。

**✍️ 書くことと描くことを、ひとつの場所で。** `#`、`-`、`[]`、`>` と打てば Markdown がその場で整います。言葉で足りないときは、ノートにそのままホワイトボードを置けます。リッチなノートは Feishu（Lark）の DocxXML 形式で保存されるので、Feishu や Web ページから貼り付けても構造が保たれます。

## 機能

### 書く

<p align="center">
  <img src="docs/images/readme-write.webp" width="100%" alt="Quiet.md で開いた長いノート。見出し、インラインの数式、イオン濃度の表があります。">
</p>

見出し、リスト、ToDo、引用、コールアウト、段組み、セル結合できる表、シンタックスハイライト付きのコード、LaTeX 数式、画像、ファイル、カバー。書きながら Markdown のショートカットが使え、それ以外は `/` メニューにあります。別のノートをメンションするとリンクになり、相手のノートにはバックリンクが表示されます。データベースは単一選択、日付、数値、数式、ルックアップ、リンクなどのフィールドで記録を管理し、ビューで絞り込み・並べ替え・グループ化ができます。「メモ」列のセルはそれぞれ本物のノートです。「…」メニューには集中モード、タイプライターモード、段落フォーカスがあり、「表示」›「拡大」は本文だけを大きくします。

### 描く

<p align="center">
  <img src="docs/images/readme-board.webp" width="100%" alt="Quiet.md のボード。画像や地図が手書きのラベル、ボックス、つながった矢印とともに並んでいます。">
</p>

ボードは Excalidraw から移植したホワイトボードです。図形、つながったままの矢印、フリーハンド、テキスト、付箋、フレーム、画像。ノートの中に置くことも、excalidraw.com で開ける `.excalidraw` ファイルとして保存することもできます。トラックパッドのピンチでズームでき、10,000 個の図形があってもなめらかに動きます。

### 本物のフォルダで整理する

サイドバーはトップレベルのフォルダごとにまとまります。フォルダを開くと、そのページにサブフォルダが中身の見える紙の束として並び、リスト表示では好きな列で並べ替えられます。コンピューター上のどのフォルダも「フォルダを追加」でその場のまま使えます。ノートをフォルダの上でしばらく止めるとフォルダが開き、フォルダの中にさらにフォルダを入れられます。ページやフォルダは別のトップレベルフォルダへも移動でき、移動はいつでも取り消せます。ノートはサイドバーの一番上にピン留めでき、検索アイコンか `⌘ J` ですべてのタイトルと本文を検索、ノート内では検索と置換ができ、正規表現も使えます。削除したノートはまずゴミ箱へ入り、何日残すかは自分で決められます。単独で開いた Markdown ファイルは専用のウインドウでその場で編集できます。

### ノートを連れてくる

「他のアプリから読み込む…」（「ファイル」メニュー）は、このコンピュータにある Obsidian、Logseq、iA Writer、Typora のライブラリと SiYuan のワークスペースを探してコピーします。元のファイルには手を触れません。Obsidian の `[[ウィキリンク]]` もそのまま使え、クリックするとリンク先のノートが開きます。あとからでも、Markdown ファイルやフォルダをフォルダのページに、あるいはサイドバーのフォルダに直接ドラッグできます。ノートは Markdown や PDF に書き出すことも、そのまま印刷することもできます。

### ほかにも

- **バージョン履歴**：ノートは保存のたびにスナップショットを残します（既定で 20 件、最大 30 件）。開くと変更箇所がその場でわかり、ワンクリックで復元でき、名前も付けられます。
- **クイックノート**：macOS では、グローバルショートカット（既定は `⌥ Space`）でどのアプリの上にも小さなノートウィンドウを開けます。
- **見た目**：Craft、Feishu、Notion、Obsidian の 4 つの組み込みスタイルに、それぞれダークモード付き。CSS で自作もできます（[docs/THEMES.md](docs/THEMES.md)）。インターフェースとドキュメントのフォントも選べます。
- **キー**：`⌘ /` ですべてのショートカットを表示でき、どれでも割り当てを変えたり消したりできます。Windows と Linux では ⌘ の代わりに Ctrl、⌥ の代わりに Alt を使います。
- **コマンドラインとエージェント**：アプリに同梱のコマンドラインツール `quiet` で、Claude Code や Codex などの AI エージェントがアプリと同じエンジンでライブラリを読み書きできます。「設定 › エージェント連携」にはエージェントに渡す一文があり、エージェントが自分でセットアップします。macOS ではコマンドラインツールをワンクリックでインストールできます。詳しくは[英語版 README](README.md#work-with-agents) へ。
- **言語**：英語、簡体字中国語、繁体字中国語、日本語、韓国語、スペイン語、フランス語、アラビア語、トルコ語。アラビア語は文字がつながり、右から左に表示されます。
- **お掃除**：「設定 › ライブラリ」で、どのノートにも使われなくなった画像やファイルを見つけ、確認してから移動します。
- **アップデート**：新しいバージョンは署名を検証したうえでアプリ内からインストールされます。自動確認はオフにできます。

## ロードマップ

- [x] リッチなドキュメント：見出し、リスト、コールアウト、表、段組み、コード、数式、画像、ファイル
- [x] Markdown ファイルを、その場で開いてそのまま編集
- [x] ボード：手描き風の図やスケッチを、ノートの中にも単独でも
- [x] どのフォルダもその場のまま開き、サイドバーに並べて表示
- [x] Obsidian、Logseq、SiYuan、Typora、iA Writer、任意の Markdown フォルダからの読み込み
- [x] 執筆モード：集中モード、タイプライターモード、段落フォーカス
- [x] テーマ、ダークモード、9 つの言語
- [x] AI エージェント向けの `quiet` CLI とスキル
- [ ] データベース（進行中）
- [ ] Mermaid 図（進行中）
- [ ] モバイル対応：iPhone、iPad、Android（進行中）
- [ ] デバイス間の同期（進行中）
- [ ] ドキュメントのプロパティ
- [ ] Web クリッパー

## ダウンロード

<p align="center">
  <a href="https://download.quiet.md/latest/quiet-md-macos-arm64.dmg"><img src="docs/images/download-macos.svg" height="56" alt="macOS 版をダウンロード"></a>
  &nbsp;
  <a href="https://download.quiet.md/latest/quiet-md-windows-x64-setup.exe"><img src="docs/images/download-windows.svg" height="56" alt="Windows 版をダウンロード"></a>
  &nbsp;
  <a href="https://download.quiet.md/latest/quiet-md-x86_64.AppImage"><img src="docs/images/download-linux.svg" height="56" alt="Linux 版をダウンロード"></a>
</p>

| プラットフォーム | パッケージ | 要件 |
|---|---|---|
| **macOS**（Apple シリコン） | [quiet-md-macos-arm64.dmg](https://download.quiet.md/latest/quiet-md-macos-arm64.dmg) | macOS 13 以降 |
| **Windows**（x64） | [quiet-md-windows-x64-setup.exe](https://download.quiet.md/latest/quiet-md-windows-x64-setup.exe) | Windows 10 バージョン 1809 以降 |
| **Linux**（x86_64、試験的） | [quiet-md-x86_64.AppImage](https://download.quiet.md/latest/quiet-md-x86_64.AppImage) | glibc 2.35 以降（Ubuntu 22.04 以降） |
| **macOS**（Intel） | [quiet-md-macos-x86_64.dmg](https://download.quiet.md/latest/quiet-md-macos-x86_64.dmg) | macOS 13 以降 |

- **macOS**: `.dmg` を開き、`quiet.md.app` を隣の「アプリケーション」へのショートカットにドラッグして、アプリケーションフォルダーから開きます。アプリとディスクイメージは Apple によって署名・公証されています。Releases ページの `.zip` は同じアプリで、アプリ内アップデートがダウンロードするものです。
- **Windows**: インストーラーを実行します。デフォルトでは現在のユーザーだけにインストールされ、アンインストールしてもノートが削除されることはありません。初回の起動時に SmartScreen が表示された場合は、**詳細情報 → 実行** をクリックしてください。
- **Linux**（試験的）: `chmod +x quiet-md-*.AppImage` を実行してから起動します。FUSE がない場合は `--appimage-extract-and-run` を付けてください。AppImage 0.1.10 以降はアプリ内更新に対応します。以前の Linux 版からは一度手動でインストールしてください。現在の対話的なテストは Wayland 上の Omarchy のみです。詳しくは [Linux の説明](CONTRIBUTING.md#linux-experimental)を参照してください。

[Quiet.md](https://quiet.md/?lang=en) からもダウンロードできます。すべてのリリースとその SHA-256 チェックサムは [GitHub Releases](https://github.com/s1dashu/quiet-md/releases) にあります。

初回起動では、書類フォルダの **Quiet Notes** フォルダ（以前のバージョンが作った `Quiet.md` フォルダがあればそれ）がそのまま開き、**Getting started** ガイドが入っています。主な機能を 10 分で紹介するツアーです。既存のノートが上書きされることはありません。ほかのフォルダの追加や他のアプリからの読み込みは、いつでも「ファイル」メニューからできます。

## パフォーマンス

**100 万文字**、549 章の混在したドキュメントで、100 回連続して操作したときのフレームの処理時間：

| 操作 | P50 | P95 |
|---|---:|---:|
| ⌨️ 入力 | **5.06 ms** | 5.78 ms |
| ⌫ Backspace | **4.99 ms** | 5.55 ms |
| 🖱️ スクロール | **2.08 ms** | 4.14 ms |

120 Hz のディスプレイでは、1 フレームに 8.3 ms が与えられます。100 万文字のドキュメントの P95 でも、まだ余裕があります。

| | |
|---|---|
| macOS 版のダウンロード | 約 18 MB |
| Windows 版のインストーラー | 約 14 MB |
| ランタイム | WebView なし、Electron なし、バックグラウンドサービスなし |

<sub>Apple M5 Pro / 48 GiB、macOS 26.5、リリースビルド。測定方法と再現用のスクリプト：[docs/PERFORMANCE.md](docs/PERFORMANCE.md)。</sub>

## コントリビューター

Quiet.md の開発に協力してくださったすべての方に感謝します。issue とプルリクエストを歓迎します。ビルド、テスト、コードの規約については [CONTRIBUTING.md](CONTRIBUTING.md) をご覧ください。

[![コントリビューター](https://raw.githubusercontent.com/s1dashu/quiet-md/contributors-assets/docs/images/contributors.svg?v=1)](https://github.com/s1dashu/quiet-md/graphs/contributors)

## コミュニティ

WeChat をお使いの方は、下の QR コードからユーザーグループに参加できます。Discord サーバーも近日公開予定です。

<img src="docs/images/wechat-group.webp" alt="WeChat ユーザーグループ" width="200">

## 最近の更新

- **0.2.0**（11 月 28 日）: ひとつのデザインシステムに基づくよりシンプルな画面、誰でも使えるデータベース、3 つの執筆モード、そして AI エージェントが使えるコマンドライン。
  - **フォルダーが最上位に**: 独立したホームはなくなりました。サイドバーはトップレベルのフォルダーを折りたためる見出しの下にまとめ、各見出しの「…」メニューには「一括管理」「Finder で表示」「サイドバーから外す」があります。「フォルダを追加」はフォルダーを元の場所のまま使います。「マイドキュメント」と「このコンピューターのファイル」はなくなりました。単独で開いた Markdown ファイルは専用のウインドウで開き、以前に開いたフォルダーはトップレベルのフォルダーになります。ページやフォルダーは別のトップレベルのフォルダーへドラッグするか「移動先」で移せます。画像、リンク、バージョン履歴も一緒に移り、取り消すこともできます。お気に入りは「ピン留め」に統合されました。
  - **ひとつのデザインシステム**: メニュー、ボタン、ダイアログ、スイッチ、ツールチップ、スクロールバーを shadcn/ui に倣った一組のコンポーネントに統一。macOS では連続曲線の角（スクワークル）、ポップオーバーには細いリングと柔らかい影、影の段階はひとつに揃え、アイコンはすべて Nucleo に。収まらないメニューはスクロールでき、サブメニューへ向かう途中で通過した行が反応しなくなりました。ヘッダーはどのページでも「+ · 検索 · …」（フォルダーでは「+ · 表示 · 検索 · …」）で、パンくずは大文字の高さでウインドウボタンと揃います。
  - **データベース**: 「新規」メニュー、`/` メニュー、CSV ファイルから作成できます。フィールドはテキスト、単一選択、複数選択、日付、数値、数式、参照、リンクなど。ビューではルールに従って行を絞り込み、並べ替え、グループ化、色分けでき、行の高さは「内容に合わせる」も選べます。「メモ」列の各セルは、ライブラリ内の本物のノートです。
  - **執筆モード**（書類の「…」メニュー）: 「集中モード」（⌘.）はウインドウに書類だけを残します。「タイプライターモード」（⌘⇧T）は書いている行を中央に保ちます。「段落フォーカス」（⌘⇧F）は今の段落をくっきり見せ、ほかを薄くします。「表示」›「拡大 / 縮小」（⌘+ / ⌘−）はノートの本文だけを拡大縮小し、見出し、表、コード、数式も一緒に変わります。
  - **コマンドラインとエージェント**: 新しい `quiet` ツールで、ターミナルからノートやデータベースの一覧表示、検索、閲覧、編集、サイドバーのフォルダーの追加と削除、指定したブロックで書類を開くことができます。「設定 › エージェント連携」で Claude Code、Codex などのエージェントをコマンドひとつで設定できます。`quiet` はアプリに同梱され、PATH に追加されます（macOS は「コマンドをインストール」、Windows はインストーラー）。
  - **見やすくなったダークモード**: リンク、文字色、ハイライト、リマインダーの日付、カウントダウン、ボタンのラベルがすべて読みやすいコントラストになり、サイドバーも明るくなりました。⌃⌘D でダークとライトを切り替えます。
  - **そのほか**: 子ページが復活。Windows と Linux でも Word、ブラウザー、Feishu から貼り付けた書式を保持。SiYuan のワークスペースを読み込めるようになり、SiYuan、Obsidian、Typora、Markdown フォルダーを「他のアプリから読み込む…」ひとつにまとめました。ゴミ箱は 1〜30 日後に自動で空になります。どのショートカットも解除できます。「新しいウインドウで開く」はメインウインドウのサイズとサイドバーに合わせます。フォルダーの名前変更や移動はすぐに反映され、ファイルはバックグラウンドで追従します。初回起動では「書類 › Quiet Notes」と整理し直したガイドがすぐに開き、Markdown、ボード、データベースのサンプルファイルが付きます。匿名で日ごとの利用回数を集計し、「設定 › 一般」でオフにできます。
  - **ライブラリ形式**: ライブラリを開くと FORMAT 6 にアップグレードされ、以後 0.1.14 以前では開けなくなります。iCloud などの同期フォルダーにあるライブラリは、当面アップグレードしません。
- **0.1.14**（11 月 22 日）: 正規表現による検索と置換、Obsidian の `[[ウィキリンク]]` からノートを開く、トルコ語と右から左に表示するアラビア語のインターフェース、インターフェースと書類のフォントの選択、使われていない画像とファイルの整理、大きなライブラリでの検索とまとめて削除の高速化。
- **0.1.13**（11 月 17 日）: ボード（Excalidraw から移植したホワイトボード）をノートの中でも `.excalidraw` ファイルとしても。すべてのノートにバージョン履歴、フランス語とアラビア語のインターフェース、Markdown としてコピー、Web から貼り付けた画像はライブラリに保存。
- **0.1.12**（11 月 14 日）: Finder からサイドバーのフォルダーへ直接ドラッグ、ホームのカードを連続曲線の角に、「Finder で開く」、ノートの領域を確保するウインドウの最小幅。
- **0.1.11**（11 月 13 日）: ライブラリがタイトル名のファイルを持つ本物のフォルダーに（バックアップを取って自動移行）、Obsidian からの読み込みを備えた新しい初回起動、日本語・スペイン語・韓国語のインターフェース、Markdown のディスプレイ数式、より速い起動と少ないメモリー使用量。
- **0.1.10**（11 月 10 日）: 「新規」メニューで書類の種類を選択、幅の広い表の横スクロール、Markdown の列揃えの保持。
- **0.1.9**（11 月 8 日）: macOS の Cmd+Q、Windows のウインドウボタン、ハイライトで選択範囲が見えない問題を修正。
- **0.1.8**（11 月 9 日）: ローカル Markdown ファイルをその場で編集、ドラッグと取り消しに対応した階層フォルダー、画像のメモリー削減、実験的な Linux AppImage。
- **0.1.7**（11 月 6 日）: より安全な複数ウインドウでの保存と競合からの復元、Markdown エクスポートでのインライン添付ファイルの保持。
- **0.1.6**（11 月 6 日）: より滑らかな入力とウインドウ切り替え、より広い表、macOS では ⌥ Space でクイックノート、Windows で画像の貼り付け・PDF エクスポート・印刷。
- **0.1.5**（11 月 5 日）: 名前を Quiet.md に変更、複数の書類をまとめて管理、ゴミ箱をワンステップで空に。
- **0.1.4**（11 月 5 日）: 署名で検証されるアプリ内アップデート、すべてのテーマにダークモード。
- **0.1.3**（11 月 4 日）: macOS のダークモードでもサイドバーの文字が読みやすく。
- **0.1.2**（11 月 4 日）: macOS 標準のメニューとショートカット、Dock アイコンでウインドウを再び開く。
- **0.1.1**（11 月 3 日）: 初の Windows x64 リリース。

各バージョンの詳しいリリースノートは [Releases](https://github.com/s1dashu/quiet-md/releases) にあります。

## ライセンス

コードは [MIT](LICENSE) のもとでライセンスされています。サードパーティーのコード、フォント、アセットは [THIRD_PARTY.md](THIRD_PARTY.md) に記載しています。

次のものは MIT ライセンスの対象外で、フォークでは置き換える必要があります。

- `assets/icons/` の 82 個と `assets/editor-icons/` の 24 個の Nucleo アイコンは、Nucleo の商用ライセンスのもとにあります。単独で取り出したり再配布したりしないでください。
- 「Quiet.md」という名前と、アプリのアイコン。

## Star History

<a href="https://www.star-history.com/#s1dashu/quiet-md&type=date">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=s1dashu/quiet-md&type=date&theme=dark" />
    <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=s1dashu/quiet-md&type=date" />
  </picture>
</a>
