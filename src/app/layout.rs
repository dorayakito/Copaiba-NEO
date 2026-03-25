use egui::{Layout, Align, Ui, InnerResponse, vec2};

/// RTL layout helpers.
/// NOTE: egui does not have a BiDi text engine, so flipping to right_to_left
/// also reverses glyph order within labels, making Arabic unreadable.
/// For now, we always use LTR widget placement and rely on the Arabic font
/// to render individual characters correctly. The `is_rtl` parameter is
/// retained so we can enable true RTL when egui adds BiDi support.

pub fn horizontal<R>(ui: &mut Ui, _is_rtl: bool, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
    ui.horizontal(add_contents)
}

pub fn horizontal_wrapped<R>(ui: &mut Ui, _is_rtl: bool, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
    ui.horizontal_wrapped(add_contents)
}

pub fn horizontal_centered<R>(ui: &mut Ui, _is_rtl: bool, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
    let w = ui.available_width();
    ui.allocate_ui_with_layout(vec2(w, 0.0), Layout::left_to_right(Align::Center).with_main_justify(true), add_contents)
}
