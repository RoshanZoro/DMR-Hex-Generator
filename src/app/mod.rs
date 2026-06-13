//! The egui application: state, layout, and event handling.

use std::time::{Duration, Instant};

use eframe::CreationContext;
use egui::{Align, Layout, RichText};
use zeroize::Zeroizing;

use crate::clipboard::ClipboardManager;
use crate::config::{Config, KeySize, MAX_KEYS, MIN_KEYS};
use crate::crypto;
use crate::security::SecretBytes;
use crate::ui::theme::{self, Palette};

/// One generated key plus its on-screen reveal state.
struct KeyEntry {
    secret: SecretBytes,
    revealed: bool,
}

impl KeyEntry {
    fn hex(&self, uppercase: bool) -> Zeroizing<String> {
        crypto::to_hex(self.secret.as_slice(), uppercase)
    }
}

/// Deferred UI actions, applied after the immediate-mode frame is built to
/// avoid borrowing `self` mutably while reading the key list.
enum Action {
    Generate,
    ClearAll,
    Copy(usize),
    ToggleReveal(usize),
    RevealAll(bool),
}

#[derive(PartialEq)]
enum StatusKind {
    Info,
    Good,
    Error,
}

pub struct KeygenApp {
    config: Config,
    keys: Vec<KeyEntry>,
    clipboard: ClipboardManager,

    palette: Palette,
    dark_mode: bool,
    /// Forces a theme regardless of the OS (`DMR_THEME=dark|light`); `None`
    /// means follow the system preference.
    theme_override: Option<bool>,

    status: String,
    status_kind: StatusKind,

    clipboard_wipe_deadline: Option<Instant>,
    keys_clear_deadline: Option<Instant>,
}

impl KeygenApp {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        // eframe sets the default style's dark_mode from the OS theme before
        // calling us, so this reflects the system preference at startup.
        let theme_override = std::env::var("DMR_THEME")
            .ok()
            .map(|t| !t.eq_ignore_ascii_case("light"));
        let dark = theme_override.unwrap_or(cc.egui_ctx.style().visuals.dark_mode);
        let palette = theme::for_system(dark);
        theme::apply(&cc.egui_ctx, palette);

        let clipboard = ClipboardManager::new();
        let mut status =
            "Ready — keys come from the OS CSPRNG and are held in RAM only.".to_string();
        let mut status_kind = StatusKind::Info;
        if let Some(err) = clipboard.init_error() {
            status = format!("Clipboard unavailable: {err}");
            status_kind = StatusKind::Error;
        }

        KeygenApp {
            config: Config::default(),
            keys: Vec::new(),
            clipboard,
            palette,
            dark_mode: dark,
            theme_override,
            status,
            status_kind,
            clipboard_wipe_deadline: None,
            keys_clear_deadline: None,
        }
    }

    /// Follow the OS light/dark preference and (re)apply our theme each frame.
    ///
    /// eframe re-applies the OS theme's default visuals every frame, so we must
    /// re-assert our styling here or our widget colors get clobbered.
    fn sync_theme(&mut self, ctx: &egui::Context) {
        let dark = self.theme_override.unwrap_or_else(|| {
            ctx.input(|i| i.raw.system_theme)
                .map(|t| t == egui::Theme::Dark)
                .unwrap_or(self.dark_mode)
        });
        self.dark_mode = dark;
        self.palette = theme::for_system(dark);
        theme::apply(ctx, self.palette);
    }

    fn set_status(&mut self, msg: impl Into<String>, kind: StatusKind) {
        self.status = msg.into();
        self.status_kind = kind;
    }

    // ---- Actions -----------------------------------------------------------

    fn generate(&mut self) {
        // Dropping old entries zeroizes their key material immediately.
        self.keys.clear();

        let n = self.config.num_keys.clamp(MIN_KEYS, MAX_KEYS);
        let byte_len = self.config.key_size.bytes();

        for _ in 0..n {
            match crypto::generate_key(byte_len) {
                Ok(secret) => self.keys.push(KeyEntry {
                    secret,
                    revealed: !self.config.start_hidden,
                }),
                Err(e) => {
                    self.keys.clear();
                    self.set_status(
                        format!("Generation aborted — {e}. No keys produced."),
                        StatusKind::Error,
                    );
                    self.keys_clear_deadline = None;
                    return;
                }
            }
        }

        let locked = self.keys.iter().all(|k| k.secret.is_locked());
        let lock_note = if locked {
            "memory-locked"
        } else {
            "in RAM (page-lock unavailable)"
        };
        let strength = match self.config.key_size {
            KeySize::Aes256 => "AES-256",
            KeySize::Aes128 => "AES-128",
        };
        self.set_status(
            format!("Generated {n} {strength} key(s) — {lock_note}."),
            StatusKind::Good,
        );

        self.keys_clear_deadline = if self.config.auto_clear_enabled {
            Some(Instant::now() + Duration::from_secs(self.config.auto_clear_secs))
        } else {
            None
        };
    }

    fn clear_all(&mut self) {
        let had = !self.keys.is_empty();
        self.keys.clear();
        self.keys_clear_deadline = None;
        if had {
            self.set_status("Keys cleared and wiped from memory.", StatusKind::Good);
        }
    }

    fn copy(&mut self, idx: usize) {
        let Some(entry) = self.keys.get(idx) else {
            return;
        };
        let hex = entry.hex(self.config.uppercase);
        match self.clipboard.set(hex) {
            Ok(()) => {
                if self.config.clipboard_wipe_enabled {
                    self.clipboard_wipe_deadline = Some(
                        Instant::now() + Duration::from_secs(self.config.clipboard_wipe_secs),
                    );
                    self.set_status(
                        format!(
                            "Copied key #{} — clipboard wipes in {}s.",
                            idx + 1,
                            self.config.clipboard_wipe_secs
                        ),
                        StatusKind::Good,
                    );
                } else {
                    self.clipboard_wipe_deadline = None;
                    self.set_status(
                        format!("Copied key #{} (clipboard auto-wipe is off).", idx + 1),
                        StatusKind::Info,
                    );
                }
            }
            Err(e) => self.set_status(format!("Copy failed: {e}"), StatusKind::Error),
        }
    }

    fn apply(&mut self, action: Action) {
        match action {
            Action::Generate => self.generate(),
            Action::ClearAll => self.clear_all(),
            Action::Copy(i) => self.copy(i),
            Action::ToggleReveal(i) => {
                if let Some(k) = self.keys.get_mut(i) {
                    k.revealed = !k.revealed;
                }
            }
            Action::RevealAll(v) => {
                for k in &mut self.keys {
                    k.revealed = v;
                }
            }
        }
    }

    // ---- Timers ------------------------------------------------------------

    fn process_timers(&mut self, ctx: &egui::Context) {
        let now = Instant::now();

        if let Some(deadline) = self.clipboard_wipe_deadline {
            if now >= deadline {
                let wiped = self.clipboard.wipe_if_ours();
                self.clipboard_wipe_deadline = None;
                self.set_status(
                    if wiped {
                        "Clipboard wiped.".to_string()
                    } else {
                        "Clipboard wipe skipped (contents changed).".to_string()
                    },
                    StatusKind::Info,
                );
            }
        }

        if let Some(deadline) = self.keys_clear_deadline {
            if now >= deadline {
                self.keys.clear();
                self.keys_clear_deadline = None;
                self.set_status("Keys auto-cleared and wiped from memory.", StatusKind::Info);
            }
        }

        // Keep repainting while a countdown is live so timers fire and the
        // displayed seconds tick down even without user input.
        if self.clipboard_wipe_deadline.is_some() || self.keys_clear_deadline.is_some() {
            ctx.request_repaint_after(Duration::from_millis(250));
        }
    }

    fn secs_left(deadline: Option<Instant>) -> Option<u64> {
        deadline.map(|d| d.saturating_duration_since(Instant::now()).as_secs())
    }

    // ---- Reusable UI bits --------------------------------------------------

    fn section_label(ui: &mut egui::Ui, p: &Palette, text: &str) {
        ui.add_space(2.0);
        ui.label(
            RichText::new(text)
                .size(11.5)
                .strong()
                .color(p.muted),
        );
        ui.add_space(4.0);
    }

    fn card<R>(ui: &mut egui::Ui, p: &Palette, add: impl FnOnce(&mut egui::Ui) -> R) -> R {
        egui::Frame::none()
            .fill(p.surface)
            .stroke(egui::Stroke::new(1.0, p.border))
            .rounding(9.0)
            .inner_margin(12.0)
            .show(ui, add)
            .inner
    }

    // ---- Panels ------------------------------------------------------------

    fn header(&self, ui: &mut egui::Ui) {
        let p = self.palette;
        ui.add_space(2.0);
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("DMR AES Key Generator")
                    .size(21.0)
                    .strong()
                    .color(p.text),
            );
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                Self::pill(ui, &p, "auto-wipe");
                Self::pill(ui, &p, "OS CSPRNG");
                Self::pill(ui, &p, "RAM-only");
            });
        });
        ui.label(
            RichText::new(
                "Cryptographically random hex keys for DMR AES — nothing is written to disk.",
            )
            .size(12.0)
            .color(p.muted),
        );
        ui.add_space(2.0);
    }

    fn pill(ui: &mut egui::Ui, p: &Palette, text: &str) {
        egui::Frame::none()
            .fill(p.surface_hi)
            .rounding(20.0)
            .inner_margin(egui::Margin::symmetric(9.0, 3.0))
            .show(ui, |ui| {
                ui.label(RichText::new(text).size(10.5).strong().color(p.accent));
            });
    }

    fn config_panel(&mut self, ui: &mut egui::Ui, actions: &mut Vec<Action>) {
        let p = self.palette;

        Self::section_label(ui, &p, "CONFIGURATION");
        Self::card(ui, &p, |ui| {
            ui.label(RichText::new("Key strength").color(p.muted).size(12.0));
            ui.add_space(2.0);
            for size in [KeySize::Aes256, KeySize::Aes128] {
                ui.radio_value(&mut self.config.key_size, size, size.label());
            }

            ui.add_space(10.0);
            ui.label(RichText::new("Number of keys").color(p.muted).size(12.0));
            ui.add(
                egui::Slider::new(&mut self.config.num_keys, MIN_KEYS..=MAX_KEYS)
                    .clamping(egui::SliderClamping::Always),
            );

            ui.add_space(10.0);
            ui.checkbox(&mut self.config.uppercase, "Uppercase hex");
            ui.checkbox(&mut self.config.start_hidden, "Hide keys on screen by default");
        });

        ui.add_space(12.0);
        Self::section_label(ui, &p, "CLIPBOARD");
        Self::card(ui, &p, |ui| {
            ui.checkbox(
                &mut self.config.clipboard_wipe_enabled,
                "Auto-wipe clipboard after copy",
            );
            ui.add_enabled_ui(self.config.clipboard_wipe_enabled, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("after").color(p.muted).size(12.0));
                    ui.add(
                        egui::DragValue::new(&mut self.config.clipboard_wipe_secs)
                            .range(1..=600)
                            .suffix(" s"),
                    );
                });
            });
        });

        ui.add_space(12.0);
        Self::section_label(ui, &p, "MEMORY");
        Self::card(ui, &p, |ui| {
            ui.checkbox(&mut self.config.auto_clear_enabled, "Auto-clear keys from RAM");
            ui.add_enabled_ui(self.config.auto_clear_enabled, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("after").color(p.muted).size(12.0));
                    ui.add(
                        egui::DragValue::new(&mut self.config.auto_clear_secs)
                            .range(5..=3600)
                            .suffix(" s"),
                    );
                });
            });
        });

        ui.add_space(16.0);
        let gen_btn = ui.add_sized(
            [ui.available_width(), 42.0],
            egui::Button::new(
                RichText::new("Generate keys")
                    .size(15.0)
                    .strong()
                    .color(p.on_accent),
            )
            .fill(p.accent)
            .rounding(8.0),
        );
        if gen_btn.clicked() {
            actions.push(Action::Generate);
        }
        if !self.keys.is_empty() {
            ui.add_space(7.0);
            if ui
                .add_sized(
                    [ui.available_width(), 32.0],
                    egui::Button::new("Clear & wipe now").rounding(8.0),
                )
                .clicked()
            {
                actions.push(Action::ClearAll);
            }
        }
    }

    fn keys_panel(&mut self, ui: &mut egui::Ui, actions: &mut Vec<Action>) {
        let p = self.palette;
        ui.horizontal(|ui| {
            Self::section_label(ui, &p, "KEYS");
            if !self.keys.is_empty() {
                ui.label(
                    RichText::new(format!("· {} in RAM", self.keys.len()))
                        .size(11.5)
                        .color(p.muted),
                );
            }
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if !self.keys.is_empty() {
                    if ui.button("Hide all").clicked() {
                        actions.push(Action::RevealAll(false));
                    }
                    if ui.button("Reveal all").clicked() {
                        actions.push(Action::RevealAll(true));
                    }
                }
            });
        });
        ui.add_space(4.0);

        if self.keys.is_empty() {
            ui.add_space(60.0);
            ui.vertical_centered(|ui| {
                ui.label(RichText::new("No keys in memory").size(16.0).color(p.muted));
                ui.add_space(2.0);
                ui.label(
                    RichText::new("Set your options, then click \"Generate keys\".")
                        .size(12.5)
                        .color(p.muted),
                );
            });
            return;
        }

        let hex_len = self.config.key_size.hex_len();
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                for idx in 0..self.keys.len() {
                    self.key_row(ui, idx, hex_len, actions);
                    ui.add_space(8.0);
                }
            });
    }

    fn key_row(&self, ui: &mut egui::Ui, idx: usize, hex_len: usize, actions: &mut Vec<Action>) {
        let p = self.palette;
        let entry = &self.keys[idx];
        egui::Frame::none()
            .fill(p.surface)
            .stroke(egui::Stroke::new(1.0, p.border))
            .rounding(9.0)
            .inner_margin(egui::Margin::symmetric(12.0, 9.0))
            .show(ui, |ui| {
                // Line 1: index badge + the key (wraps if the window is narrow).
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        RichText::new(format!("{:02}", idx + 1))
                            .monospace()
                            .color(p.accent)
                            .strong(),
                    );
                    ui.add_space(6.0);

                    if entry.revealed {
                        let hex = entry.hex(self.config.uppercase);
                        ui.add(
                            egui::Label::new(RichText::new(hex.as_str()).monospace().size(14.0))
                                .selectable(true),
                        );
                    } else {
                        // Latin-1 middots (not emoji glyphs) for a clean mask.
                        let mask: String = "\u{00B7}".repeat(hex_len.min(64));
                        ui.label(RichText::new(mask).monospace().size(14.0).color(p.muted));
                    }
                });

                ui.add_space(7.0);

                // Line 2: right-aligned actions, so they never collide with the
                // key. Wrapped in a horizontal so the row keeps a single-line
                // height instead of expanding to fill the scroll area.
                ui.horizontal(|ui| {
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui
                            .button(if entry.revealed { "Hide" } else { "Reveal" })
                            .clicked()
                        {
                            actions.push(Action::ToggleReveal(idx));
                        }
                        if ui
                            .add(
                                egui::Button::new(
                                    RichText::new("Copy").strong().color(p.on_accent),
                                )
                                .fill(p.accent),
                            )
                            .on_hover_text("Copy to clipboard")
                            .clicked()
                        {
                            actions.push(Action::Copy(idx));
                        }
                    });
                });
            });
    }

    fn status_bar(&self, ui: &mut egui::Ui) {
        let p = self.palette;
        ui.add_space(1.0);
        ui.horizontal(|ui| {
            let color = match self.status_kind {
                StatusKind::Good => p.ok,
                StatusKind::Error => p.danger,
                StatusKind::Info => p.accent,
            };
            // Painted status dot — font-independent, no glyph/emoji needed.
            let (rect, _) = ui.allocate_exact_size(egui::vec2(10.0, 10.0), egui::Sense::hover());
            ui.painter().circle_filled(rect.center(), 4.0, color);
            ui.add_space(2.0);
            ui.label(RichText::new(&self.status).size(12.0));

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let mut shown = false;
                if let Some(s) = Self::secs_left(self.clipboard_wipe_deadline) {
                    ui.label(
                        RichText::new(format!("clipboard wipe in {s}s"))
                            .size(12.0)
                            .color(p.muted),
                    );
                    shown = true;
                }
                if let Some(s) = Self::secs_left(self.keys_clear_deadline) {
                    let sep = if shown { "  \u{00B7}  " } else { "" };
                    ui.label(
                        RichText::new(format!("keys clear in {s}s{sep}"))
                            .size(12.0)
                            .color(p.muted),
                    );
                }
            });
        });
        ui.add_space(1.0);
    }
}

impl eframe::App for KeygenApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.sync_theme(ctx);
        let p = self.palette;
        let mut actions: Vec<Action> = Vec::new();

        let panel_frame =
            |fill| egui::Frame::none().fill(fill).inner_margin(egui::Margin::symmetric(16.0, 10.0));

        egui::TopBottomPanel::top("header")
            .frame(panel_frame(p.bg))
            .show(ctx, |ui| self.header(ui));

        egui::SidePanel::left("config")
            .resizable(false)
            .exact_width(278.0)
            .frame(egui::Frame::none().fill(p.bg).inner_margin(egui::Margin::symmetric(16.0, 8.0)))
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| self.config_panel(ui, &mut actions));
            });

        egui::TopBottomPanel::bottom("status")
            .frame(egui::Frame::none().fill(p.surface).inner_margin(egui::Margin::symmetric(16.0, 7.0)))
            .show(ctx, |ui| self.status_bar(ui));

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(p.bg).inner_margin(egui::Margin::symmetric(16.0, 8.0)))
            .show(ctx, |ui| self.keys_panel(ui, &mut actions));

        for action in actions {
            self.apply(action);
        }

        self.process_timers(ctx);
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // Best-effort: remove our secret from the clipboard. Key buffers are
        // zeroized as `self.keys` is dropped.
        self.clipboard.wipe_if_ours();
        self.keys.clear();
    }
}
