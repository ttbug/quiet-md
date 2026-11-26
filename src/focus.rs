//! 专注模式: the open document alone in its window. The sidebar folds away and the header's
//! crumbs and buttons go; on macOS the traffic lights go too (Windows: the caption buttons).
//! A strip along the window's top brings them back while the pointer is on it, with 退出专注模式
//! beside them. That button, ⌘. (Ctrl+Shift+Enter on Windows) or Esc leave it, and the
//! sidebar comes back as it was. Page width, size and 阅读模式 are the document's, untouched.
//! Not remembered, like 窗口保持在最前 and iA Writer's or Typora's own: a reopened window shows
//! everything. See docs/design/FOCUS_MODE.md.
use crate::app::{HEADER_H, Workspace};
use crate::ui::{TipExt, TitleBarExt};
use gpui::{prelude::FluentBuilder, *};
use std::time::{Duration, Instant};

actions!(workspace, [ToggleFocus]);

/// ⌘. on macOS, Ulysses' 隐藏界面 and Craft's Focus Mode. Windows takes Ctrl+Shift+Enter:
/// Ctrl+. is Microsoft Pinyin's 中英文标点 switch, and F11 is already full screen there.
pub const KEY: &str = if cfg!(target_os = "macos") { "cmd-." } else { "ctrl-shift-enter" };
/// The key as the UI writes it.
pub fn hint() -> &'static str { crate::shortcuts::hint("workspace::ToggleFocus", if cfg!(target_os = "macos") { "⌘ + ." } else { "Ctrl + Shift + Enter" }) }

pub fn bind_keys(cx: &mut App) {
    // Bound as written on each platform (`pc_binding` would read ctrl-shift-enter as Cocoa's).
    cx.bind_keys([KeyBinding::new(KEY, ToggleFocus, None)]);
}

/// The menu row's glyph: the four corners of a page filling its frame (Nucleo expand).
pub const ICON: &str = "expand";
/// How long the pointer may be off the strip before its controls fade: crossing it on the
/// way to the menu bar, or overshooting the button, leaves them.
const LINGER: Duration = Duration::from_millis(400);
const FADE_IN: f32 = 0.15;
const FADE_OUT: f32 = 0.25;

/// What the window showed before, to put back on leaving.
struct Saved { sidebar_open: bool, sidebar_auto_hidden: bool }

/// A window's 专注模式 and its top strip's fade.
pub struct Focus {
    saved: Option<Saved>,
    reveal: bool,
    /// The strip's opacity when `reveal` last changed, and when its fade starts.
    from: f32,
    since: Instant,
    /// The traffic lights' alpha last given to AppKit.
    lights: std::cell::Cell<f32>,
}
impl Default for Focus {
    fn default() -> Self { Focus { saved: None, reveal: false, from: 0., since: Instant::now(), lights: std::cell::Cell::new(1.) } }
}
impl Focus {
    pub fn on(&self) -> bool { self.saved.is_some() }
    /// The strip's controls, 0 (gone) to 1. 动画 › 丰富 fades them; 克制 shows and hides at once.
    pub fn opacity(&self) -> f32 {
        let target = if self.reveal { 1. } else { 0. };
        let now = Instant::now();
        if now < self.since { return self.from }
        if !crate::settings::motion_high() { return target }
        let t = ((now - self.since).as_secs_f32() / if self.reveal { FADE_IN } else { FADE_OUT }).min(1.);
        let e = 1. - (1. - t) * (1. - t);
        self.from + (target - self.from) * e
    }
    fn moving(&self) -> bool {
        let fade = if crate::settings::motion_high() { if self.reveal { FADE_IN } else { FADE_OUT } } else { 0. };
        Instant::now() < self.since + Duration::from_secs_f32(fade)
    }
    pub fn set_reveal(&mut self, on: bool) {
        if on == self.reveal { return }
        self.from = self.opacity();
        self.reveal = on;
        self.since = if on { Instant::now() } else { Instant::now() + LINGER };
    }
}

/// Give the traffic lights `alpha` (macOS; elsewhere the app draws its own controls). At 0
/// they are hidden as well, so an unseen 关闭 takes no click.
fn lights(window: &Window, alpha: f32) {
    #[cfg(target_os = "macos")]
    unsafe {
        use objc::runtime::{NO, Object, YES};
        use objc::{msg_send, sel, sel_impl};
        let handle = crate::printing::parent_of(window);
        if handle == 0 { return }
        let win: *mut Object = msg_send![handle as *mut Object, window];
        if win.is_null() { return }
        // NSWindowCloseButton, NSWindowMiniaturizeButton, NSWindowZoomButton
        for kind in 0..3usize {
            let b: *mut Object = msg_send![win, standardWindowButton: kind];
            if b.is_null() { continue }
            let _: () = msg_send![b, setHidden: if alpha <= 0. { YES } else { NO }];
            let _: () = msg_send![b, setAlphaValue: alpha as f64];
        }
    }
    #[cfg(not(target_os = "macos"))]
    let _ = (window, alpha);
}

/// 退出专注模式: the button the strip shows beside the traffic lights.
pub fn exit_button(id: impl Into<ElementId>) -> crate::design::Button {
    crate::design::Button::ghost(id).sized(crate::design::ButtonSize::Sm).icon(ICON).label(crate::settings::tr("退出专注模式", "Exit Focus Mode"))
}

impl Workspace {
    /// Whether 专注模式 can start here: a document's own page (not a 数据表 or 画板 file).
    pub fn can_focus(&self, cx: &App) -> bool {
        self.view == crate::app::View::Doc && self.current.is_some()
            && self.editor.as_ref().is_none_or(|e| { let e = e.read(cx); !e.table_file && !e.board_file })
    }

    pub fn toggle_focus(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.focus_mode.on() { self.leave_focus(window, cx) } else { self.enter_focus(window, cx) }
    }

    pub fn enter_focus(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if self.focus_mode.on() || !self.can_focus(cx) { return }
        self.doc_menu = false;
        self.doc_sub = None;
        self.focus_mode.saved = Some(Saved { sidebar_open: self.sidebar_open, sidebar_auto_hidden: self.sidebar_auto_hidden });
        self.focus_mode.reveal = false;
        self.focus_mode.from = 0.;
        self.sidebar_open = false;
        self.sidebar_auto_hidden = false;
        if let Some(ed) = &self.editor { ed.update(cx, |e, cx| { e.focus_mode = true; cx.notify(); }); }
        self.show_toast_kind(crate::ui::ToastKind::Info, crate::settings::tr("按 Esc 退出专注模式", "Press Esc to exit Focus Mode"), cx);
        cx.notify();
    }

    pub fn leave_focus(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(saved) = self.focus_mode.saved.take() else { return };
        self.sidebar_open = saved.sidebar_open;
        self.sidebar_auto_hidden = saved.sidebar_auto_hidden;
        self.focus_mode.reveal = false;
        self.focus_mode.lights.set(1.);
        lights(window, 1.);
        if let Some(ed) = &self.editor { ed.update(cx, |e, cx| { e.focus_mode = false; cx.notify(); }); }
        cx.notify();
    }

    /// Each frame: leave when the page is no longer a document's, and keep the traffic lights
    /// with the strip. In full screen the system has the title bar, and they are left alone.
    pub(crate) fn sync_focus(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.focus_mode.on() {
            return;
        }
        if !self.can_focus(cx) { return self.leave_focus(window, cx) }
        if let Some(ed) = &self.editor && !ed.read(cx).focus_mode { ed.update(cx, |e, cx| { e.focus_mode = true; cx.notify(); }); }
        let alpha = if window.is_fullscreen() { 1. } else { self.focus_mode.opacity() };
        if (alpha - self.focus_mode.lights.get()).abs() > 0.004 {
            self.focus_mode.lights.set(alpha);
            lights(window, alpha);
        }
        if self.focus_mode.moving() { window.request_animation_frame(); }
    }

    /// The header in 专注模式: an empty strip that drags the window like a title bar and, while
    /// the pointer is on it, shows 退出专注模式 where the crumbs began.
    pub(crate) fn focus_bar(&self, cx: &mut Context<Self>) -> AnyElement {
        let o = self.focus_mode.opacity();
        div().id("focus-bar").h(px(HEADER_H)).flex_none().flex().items_center()
            .pl(px(if cfg!(target_os = "macos") { crate::theme::TRAFFIC_LIGHTS_W + 4. } else { 18. }))
            .title_bar()
            .on_hover(cx.listener(|w, on: &bool, _, cx| { w.focus_mode.set_reveal(*on); cx.notify(); }))
            .when(o > 0., |d| d.child(
                div().opacity(o).child(exit_button("focus-exit").no_drag()
                    .tip(crate::settings::tr("退出专注模式", "Exit Focus Mode"), Some(hint()))
                    .on_click(cx.listener(|w, _, window, cx| w.leave_focus(window, cx)))),
            ))
            .into_any_element()
    }
}
