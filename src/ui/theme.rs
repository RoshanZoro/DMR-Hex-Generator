//! Clean, professional theming with matching light and dark palettes.
//!
//! The active palette is chosen from the OS theme by the app and threaded
//! through the UI, so colors stay consistent and switch live with the system.

use egui::{Color32, Rounding, Stroke, Style, Visuals};

/// A full set of semantic colors for one theme variant.
#[derive(Clone, Copy)]
pub struct Palette {
    pub dark: bool,
    pub bg: Color32,
    pub surface: Color32,
    pub surface_hi: Color32,
    pub border: Color32,
    pub accent: Color32,
    pub accent_hover: Color32,
    pub on_accent: Color32,
    pub ok: Color32,
    pub danger: Color32,
    pub muted: Color32,
    pub text: Color32,
}

const fn rgb(r: u8, g: u8, b: u8) -> Color32 {
    Color32::from_rgb(r, g, b)
}

/// Dark theme — deep slate with a teal accent.
pub fn dark() -> Palette {
    Palette {
        dark: true,
        bg: rgb(0x0d, 0x11, 0x17),
        surface: rgb(0x16, 0x1c, 0x24),
        surface_hi: rgb(0x23, 0x2c, 0x38),
        border: rgb(0x2c, 0x37, 0x44),
        accent: rgb(0x2d, 0xd4, 0xbf),
        accent_hover: rgb(0x5e, 0xea, 0xd4),
        on_accent: rgb(0x04, 0x1a, 0x17),
        ok: rgb(0x4a, 0xde, 0x80),
        danger: rgb(0xf8, 0x71, 0x71),
        muted: rgb(0x8b, 0x97, 0xa7),
        text: rgb(0xe6, 0xed, 0xf3),
    }
}

/// Light theme — soft paper with a deeper teal accent for contrast.
pub fn light() -> Palette {
    Palette {
        dark: false,
        bg: rgb(0xf2, 0xf4, 0xf7),
        surface: rgb(0xff, 0xff, 0xff),
        surface_hi: rgb(0xe9, 0xed, 0xf2),
        border: rgb(0xd4, 0xda, 0xe2),
        accent: rgb(0x0d, 0x94, 0x88),
        accent_hover: rgb(0x0f, 0x76, 0x6e),
        on_accent: rgb(0xff, 0xff, 0xff),
        ok: rgb(0x15, 0x9a, 0x52),
        danger: rgb(0xd9, 0x34, 0x34),
        muted: rgb(0x5b, 0x65, 0x73),
        text: rgb(0x1c, 0x24, 0x30),
    }
}

/// Build the palette matching the OS preference.
pub fn for_system(is_dark: bool) -> Palette {
    if is_dark { dark() } else { light() }
}

/// Apply a palette to the egui context, styling every widget state so buttons,
/// inputs, sliders and frames all read consistently.
pub fn apply(ctx: &egui::Context, p: Palette) {
    let mut style = Style::default();
    let mut v = if p.dark { Visuals::dark() } else { Visuals::light() };

    v.dark_mode = p.dark;
    v.override_text_color = Some(p.text);
    v.panel_fill = p.bg;
    v.window_fill = p.surface;
    v.window_stroke = Stroke::new(1.0, p.border);
    v.window_rounding = Rounding::same(10.0);
    v.extreme_bg_color = if p.dark {
        rgb(0x0a, 0x0e, 0x13)
    } else {
        rgb(0xff, 0xff, 0xff)
    };
    v.faint_bg_color = p.surface_hi;
    v.hyperlink_color = p.accent;
    v.selection.bg_fill = p.accent.linear_multiply(0.35);
    v.selection.stroke = Stroke::new(1.0, p.accent);
    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, p.border);

    let rounding = Rounding::same(7.0);

    // Non-interactive (labels, separators, frame outlines)
    let w = &mut v.widgets;
    w.noninteractive.bg_fill = p.surface;
    w.noninteractive.weak_bg_fill = p.surface;
    w.noninteractive.bg_stroke = Stroke::new(1.0, p.border);
    w.noninteractive.fg_stroke = Stroke::new(1.0, p.text);
    w.noninteractive.rounding = rounding;

    // Resting buttons / inputs
    w.inactive.bg_fill = p.surface_hi;
    w.inactive.weak_bg_fill = p.surface_hi;
    w.inactive.bg_stroke = Stroke::new(1.0, p.border);
    w.inactive.fg_stroke = Stroke::new(1.0, p.text);
    w.inactive.rounding = rounding;
    w.inactive.expansion = 0.0;

    // Hover
    w.hovered.bg_fill = if p.dark { p.surface_hi } else { rgb(0xdf, 0xe5, 0xec) };
    w.hovered.weak_bg_fill = w.hovered.bg_fill;
    w.hovered.bg_stroke = Stroke::new(1.2, p.accent_hover);
    w.hovered.fg_stroke = Stroke::new(1.0, p.text);
    w.hovered.rounding = rounding;
    w.hovered.expansion = 1.0;

    // Pressed / on
    w.active.bg_fill = p.accent;
    w.active.weak_bg_fill = p.accent;
    w.active.bg_stroke = Stroke::new(1.0, p.accent);
    w.active.fg_stroke = Stroke::new(1.0, p.on_accent);
    w.active.rounding = rounding;
    w.active.expansion = 0.0;

    // Open (combo boxes etc.)
    w.open.bg_fill = p.surface_hi;
    w.open.weak_bg_fill = p.surface_hi;
    w.open.bg_stroke = Stroke::new(1.0, p.border);
    w.open.fg_stroke = Stroke::new(1.0, p.text);
    w.open.rounding = rounding;

    style.visuals = v;

    style.spacing.item_spacing = egui::vec2(10.0, 10.0);
    style.spacing.button_padding = egui::vec2(12.0, 7.0);
    style.spacing.window_margin = egui::Margin::same(14.0);
    style.spacing.menu_margin = egui::Margin::same(10.0);
    style.spacing.interact_size.y = 28.0;
    style.spacing.slider_width = 168.0;
    style.spacing.scroll = egui::style::ScrollStyle::solid();

    ctx.set_style(style);
}
