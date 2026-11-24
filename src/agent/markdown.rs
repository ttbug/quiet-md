//! Read-only Markdown for assistant messages; no editor or document-library state.
use crate::model::{Block, BlockKind, RichText};
use crate::settings::tr;
use crate::theme::{self, Theme};
use crate::ui;
use gpui::{prelude::FluentBuilder, *};

pub(super) fn parse(text: &str) -> Vec<Block> {
    crate::paste::markdown_to_blocks(text, crate::paste::Off { file: true, alt: true, ..Default::default() })
}

fn link_allowed(url: &str) -> bool {
    url.starts_with("https://") || url.starts_with("http://") || url.starts_with("mailto:")
}

fn rich(text: &RichText, id: &str, bold: bool) -> AnyElement {
    let mut display = String::new();
    let mut runs = vec![];
    let mut links = vec![];
    for (range, marks) in text.spans() {
        let value = marks.inline.as_ref().and_then(|i| i.equation()).unwrap_or(&text.text[range]);
        let start = display.len();
        display.push_str(value);
        let style = TextStyle { font_family: if marks.code { theme::MONO_FONT } else { theme::ui_family() }.into(),
            font_weight: if bold || marks.bold { FontWeight::BOLD } else { FontWeight::NORMAL },
            font_style: if marks.italic { FontStyle::Italic } else { FontStyle::Normal },
            color: if marks.link.is_some() { Theme::link() } else { Theme::text() },
            background_color: marks.code.then(Theme::code_chip),
            underline: (marks.underline || marks.link.is_some()).then_some(UnderlineStyle { thickness: px(1.), color: None, wavy: false }),
            strikethrough: marks.strike.then_some(StrikethroughStyle { thickness: px(1.), color: None }), ..Default::default() };
        if marks.code { runs.push(style.to_run(value.len())); }
        else { runs.extend(ui::ui_runs(value, &style, &[], style.color)); }
        if let Some(url) = marks.link.as_ref().filter(|url| link_allowed(url)) { links.push((start..display.len(), url.clone())); }
    }
    let ranges = links.iter().map(|(range, _)| range.clone()).collect();
    InteractiveText::new(SharedString::from(format!("{id}-text")), StyledText::new(display).with_runs(runs))
        .on_click(ranges, move |i, _, cx| cx.open_url(&links[i].1)).into_any_element()
}

pub(super) fn render(blocks: &[Block], id: &str) -> AnyElement {
    div().w_full().min_w_0().flex().flex_col().gap(px(8.))
        .children(blocks.iter().enumerate().map(|(i, block)| render_block(block, &format!("{id}-{i}"), crate::editor::core::ordered_number(blocks, i))))
        .into_any_element()
}

fn render_block(block: &Block, id: &str, number: u32) -> AnyElement {
    let mut outer = div().id(SharedString::from(id.to_owned())).w_full().min_w_0().flex().flex_col().gap(px(6.));
    match &block.kind {
        BlockKind::Divider => return outer.my(px(4.)).h(px(1.)).bg(Theme::line_border()).into_any_element(),
        BlockKind::Code { lang, .. } => {
            let code = block.text.text.clone();
            return outer.rounded(px(6.)).border_1().border_color(Theme::code_border()).bg(Theme::code_bg())
                .child(div().px(px(8.)).py(px(4.)).flex().items_center().justify_between().text_size(px(11.)).text_color(Theme::text_caption())
                    .child(lang.clone())
                    .child(crate::design::Button::outline("copy").sized(crate::design::ButtonSize::Sm).label(tr("复制", "Copy")).on_click(move |_, _, cx| {
                        let mut fake = crate::clipboard::FAKE_CLIPBOARD.lock().unwrap();
                        if let Some(clipboard) = fake.as_mut() {
                            clipboard.text = code.clone(); clipboard.html = None; clipboard.images.clear();
                        } else { cx.write_to_clipboard(ClipboardItem::new_string(code.clone())); }
                    })))
                .child(div().id(SharedString::from(format!("{id}-scroll"))).w_full().min_w_0().flex().items_start().overflow_x_scroll().p(px(8.))
                    .child(div().flex_none().font_family(theme::MONO_FONT).text_size(px(12.)).line_height(px(18.)).text_color(Theme::code_text())
                        .whitespace_nowrap().child(block.text.text.clone())))
                .into_any_element();
        }
        BlockKind::Table(table) => {
            return outer.overflow_x_scroll().child(div().w(px(table.ncols() as f32 * 140.)).flex_none().flex().flex_col().border_t_1().border_l_1().border_color(Theme::line_border())
                .children(table.rows.iter().enumerate().map(|(r, row)| div().flex().children(row.iter().enumerate().map(|(c, cell)| {
                    div().id(SharedString::from(format!("{id}-{r}-{c}"))).w(px(140.)).flex_none().p(px(8.))
                        .border_r_1().border_b_1().border_color(Theme::line_border())
                        .when(r < table.header_rows, |d| d.bg(Theme::fill_hover()))
                        .children(cell.blocks.iter().enumerate().map(|(i, b)| {
                            let cell_id = format!("{id}-{r}-{c}-{i}");
                            if b.children.is_empty() && matches!(b.kind, BlockKind::Text) { rich(&b.text, &cell_id, r < table.header_rows) }
                            else { render_block(b, &cell_id, crate::editor::core::ordered_number(&cell.blocks, i)) }
                        }))
                })))))
                .into_any_element();
        }
        BlockKind::Quote | BlockKind::Callout { .. } => {
            outer = outer.border_l_2().border_color(Theme::line_border()).pl(px(10.));
            if !block.text.text.is_empty() { outer = outer.child(rich(&block.text, id, false)); }
            return outer.child(render(&block.children, id)).into_any_element();
        }
        BlockKind::Heading { level, .. } => {
            let size = match level { 1 => 22., 2 => 19., 3 => 16., _ => 14. };
            outer = outer.mt(px(4.)).text_size(px(size)).line_height(px(size + 8.)).child(rich(&block.text, id, true));
        }
        BlockKind::Bullet | BlockKind::Ordered { .. } | BlockKind::Todo { .. } => {
            let marker = match block.kind {
                BlockKind::Ordered { .. } => format!("{number}."),
                BlockKind::Todo { done } => if done { "☑" } else { "☐" }.into(),
                _ => "•".into(),
            };
            outer = outer.child(div().flex().gap(px(6.)).child(div().min_w(px(18.)).flex_none().child(marker))
                .child(div().flex_1().min_w_0().child(rich(&block.text, id, false))));
        }
        BlockKind::Image { src, caption, .. } => {
            outer = outer.child(ui::ui_para(format!("[{}] {src}", caption.as_deref().unwrap_or(tr("图片", "Image")))));
        }
        _ => outer = outer.child(rich(&block.text, id, false)),
    }
    if !block.children.is_empty() { outer = outer.child(div().pl(px(24.)).child(render(&block.children, id))); }
    outer.into_any_element()
}

#[cfg(test)]
mod tests {
    use super::{link_allowed, parse};
    use crate::model::BlockKind;

    #[test]
    fn assistant_markdown_keeps_structure_and_inline_styles() {
        let blocks = parse("## Heading\n\n**粗体** *italic* `code` [link](https://example.com)\n\n- item\n  - nested\n\n> quote\n\n---\n\n| A | B |\n| --- | --- |\n| 中 | 文 |\n");
        assert!(matches!(blocks[0].kind, BlockKind::Heading { level: 2, .. }));
        let text = &blocks[1].text;
        assert!(text.runs.iter().any(|r| r.marks.bold));
        assert!(text.runs.iter().any(|r| r.marks.italic));
        assert!(text.runs.iter().any(|r| r.marks.code));
        assert!(text.runs.iter().any(|r| r.marks.link.as_deref() == Some("https://example.com")));
        assert!(blocks.iter().any(|b| matches!(b.kind, BlockKind::Bullet) && !b.children.is_empty()));
        assert!(blocks.iter().any(|b| matches!(b.kind, BlockKind::Quote)));
        assert!(blocks.iter().any(|b| matches!(b.kind, BlockKind::Divider)));
        assert!(blocks.iter().any(|b| matches!(&b.kind, BlockKind::Table(t) if t.rows.len() == 2 && t.ncols() == 2)));
        let numbered = parse("3. three\n4. four\n");
        assert_eq!(crate::editor::core::ordered_number(&numbered, 0), 3);
        assert_eq!(crate::editor::core::ordered_number(&numbered, 1), 4);
    }

    #[test]
    fn streaming_prefixes_and_unclosed_code_are_readable() {
        let text = "中文 **bold**\n\n```rust\nlet x = 1;\n";
        for end in text.char_indices().map(|(i, _)| i).chain([text.len()]) { let _ = parse(&text[..end]); }
        let blocks = parse(text);
        assert!(matches!(blocks.last().unwrap().kind, BlockKind::Code { .. }));
        assert_eq!(blocks.last().unwrap().text.text, "let x = 1;");
        assert!(!link_allowed("javascript:alert(1)"));
        assert!(!link_allowed("file:///tmp/note"));
        assert!(link_allowed("https://example.com"));
    }
}
