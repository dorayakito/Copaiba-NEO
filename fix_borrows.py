import os
import re

ui_dir = "src/app"

for filename in os.listdir(ui_dir):
    if not filename.endswith(".rs"): continue
    filepath = os.path.join(ui_dir, filename)
    with open(filepath, "r", encoding="utf-8") as f:
        content = f.read()

    original = content
    # Replace self.is_rtl() with is_rtl in the layout calls
    content = content.replace("crate::app::layout::horizontal(ui, self.is_rtl(),", "crate::app::layout::horizontal(ui, is_rtl,")
    content = content.replace("crate::app::layout::horizontal_wrapped(ui, self.is_rtl(),", "crate::app::layout::horizontal_wrapped(ui, is_rtl,")
    content = content.replace("crate::app::layout::horizontal_centered(ui, self.is_rtl(),", "crate::app::layout::horizontal_centered(ui, is_rtl,")

    # Now we need to insert `let is_rtl = self.is_rtl();` at the top of methods
    # We look for `pub fn show_` or other big functions taking `&mut self` or `&self`
    # and insert `let is_rtl = self.is_rtl();` right after the opening brace.
    def insert_rtl(match):
        return match.group(0) + "\n        let is_rtl = self.is_rtl();"

    content = re.sub(r'(pub fn .*?\(\s*&mut self.*?\)\s*\{)', insert_rtl, content)
    content = re.sub(r'(pub fn .*?\(\s*&self.*?\)\s*\{)', insert_rtl, content)

    if content != original:
        with open(filepath, "w", encoding="utf-8") as f:
            f.write(content)

print("Fixed borrow checker issues.")
