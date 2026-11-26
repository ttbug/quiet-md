//! 打字机模式: the caret's line stays in the middle of the page while one
//! writes, as in iA Writer, Typora and Scrivener (docs/design/typewriter.md).
//! A switch in the "…" menu and ⌘⇧T (Ctrl+Shift+T), remembered for every
//! document. The scrolling itself is the editor's caret reveal
//! (`Editor::apply_reveal`); this holds the switch.
use crate::app::Workspace;
use gpui::*;

actions!(workspace, [ToggleTypewriter]);

/// The "…" menu row's glyph: a line held between two arrows.
pub const ICON: &str = "valign-middle";

pub fn bind_keys(cx: &mut App) {
    cx.bind_keys([KeyBinding::new("cmd-shift-t", ToggleTypewriter, None)]);
}

impl Workspace {
    /// The "…" menu's switch and ⌘⇧T; the shortcut says what it did, the
    /// menu shows it on its switch.
    pub fn set_typewriter(&mut self, on: bool, toast: bool, cx: &mut Context<Self>) {
        self.set_pref(|p| p.typewriter = on, cx);
        if let Some(ed) = &self.editor {
            ed.update(cx, |e, cx| e.typewriter_changed(cx));
        }
        if toast {
            self.show_toast(if on { crate::settings::tr("打字机模式已打开", "Typewriter mode on") } else { crate::settings::tr("打字机模式已关闭", "Typewriter mode off") }, cx);
        }
        cx.notify();
    }
}
