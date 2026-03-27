use egui::{Color32, Rect, Stroke, Vec2};
use std::time::Instant;

#[derive(Clone, Copy, PartialEq)]
pub enum ToastKind {
    Info,
    Success,
    Warning,
    Error,
}

impl ToastKind {
    pub fn color(&self) -> Color32 {
        match self {
            ToastKind::Info => Color32::from_rgb(50, 150, 250),
            ToastKind::Success => Color32::from_rgb(50, 220, 100),
            ToastKind::Warning => Color32::from_rgb(250, 200, 50),
            ToastKind::Error => Color32::from_rgb(250, 80, 80),
        }
    }
    
    pub fn icon(&self) -> &'static str {
        match self {
            ToastKind::Info => "ℹ",
            ToastKind::Success => "✔",
            ToastKind::Warning => "⚠",
            ToastKind::Error => "✖",
        }
    }
}

pub struct Toast {
    text: String,
    kind: ToastKind,
    created_at: Instant,
    duration: f32, // In seconds
}

impl Toast {
    pub fn new(text: impl Into<String>, kind: ToastKind, duration_sec: f32) -> Self {
        Self {
            text: text.into(),
            kind,
            created_at: Instant::now(),
            duration: duration_sec,
        }
    }
}

#[derive(Default)]
pub struct ToastManager {
    toasts: Vec<Toast>,
}

impl ToastManager {
    pub fn info(&mut self, text: impl Into<String>) {
        self.toasts.push(Toast::new(text, ToastKind::Info, 3.0));
    }
    pub fn success(&mut self, text: impl Into<String>) {
        self.toasts.push(Toast::new(text, ToastKind::Success, 3.0));
    }
    pub fn warning(&mut self, text: impl Into<String>) {
        self.toasts.push(Toast::new(text, ToastKind::Warning, 5.0));
    }
    pub fn error(&mut self, text: impl Into<String>) {
        self.toasts.push(Toast::new(text, ToastKind::Error, 5.0));
    }

    pub fn draw(&mut self, ctx: &egui::Context) {
        if self.toasts.is_empty() { return; }
        
        // Remove expired toasts
        let now = Instant::now();
        self.toasts.retain(|t| now.duration_since(t.created_at).as_secs_f32() < t.duration + 0.5);

        let mut y_offset = 20.0;
        let screen_rect = ctx.screen_rect();
        
        for toast in self.toasts.iter().rev() {
            let elapsed = now.duration_since(toast.created_at).as_secs_f32();
            
            // Fade in (0.2s) and fade out (last 0.5s)
            let mut alpha = 1.0_f32;
            if elapsed < 0.2 {
                alpha = elapsed / 0.2;
            } else if elapsed > toast.duration {
                alpha = 1.0 - ((elapsed - toast.duration) / 0.5).clamp(0.0, 1.0);
            }
            
            if alpha <= 0.0 { continue; } // Invisible

            let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Tooltip, egui::Id::new("toasts")));

            let text_color = Color32::WHITE.linear_multiply(alpha);
            let icon_color = toast.kind.color().linear_multiply(alpha);
            let bg_color = Color32::from_rgba_premultiplied(30, 30, 40, (230.0 * alpha) as u8);
            let stroke_color = Color32::from_rgba_premultiplied(60, 60, 80, (200.0 * alpha) as u8);

            // Calculate text rect bounds directly instead of using egui Layout to decouple from central constraints
            let galley = ctx.fonts(|f| f.layout_no_wrap(toast.text.clone(), egui::FontId::proportional(14.0), text_color));
            let icon_galley = ctx.fonts(|f| f.layout_no_wrap(toast.kind.icon().to_string(), egui::FontId::proportional(16.0), icon_color));
            
            let padding = Vec2::new(12.0, 10.0);
            let spacing = 8.0;
            let total_width = icon_galley.rect.width() + spacing + galley.rect.width() + padding.x * 2.0;
            let total_height = galley.rect.height().max(icon_galley.rect.height()) + padding.y * 2.0;
            
            // Slide up animation (10px) on intro
            let anim_offset = if elapsed < 0.2 { 10.0 * (1.0 - alpha) } else { 0.0 };

            let rect = Rect::from_min_size(
                egui::pos2(screen_rect.right() - total_width - 20.0, screen_rect.bottom() - y_offset - total_height + anim_offset),
                Vec2::new(total_width, total_height)
            );

            painter.rect(
                rect,
                8.0,
                bg_color,
                Stroke::new(1.0, stroke_color),
                egui::StrokeKind::Inside,
            );

            painter.galley(rect.min + padding, icon_galley.clone(), icon_color);
            painter.galley(rect.min + egui::vec2(padding.x + icon_galley.rect.width() + spacing, padding.y), galley, text_color);

            y_offset += total_height + 10.0;
        }
        
        if !self.toasts.is_empty() {
            ctx.request_repaint_after(std::time::Duration::from_millis(16));
        }
    }
}
