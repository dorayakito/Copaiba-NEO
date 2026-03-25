import os
import re

ui_dir = "src/app"
count = 0

patterns = [
    (re.compile(r'\bui\.horizontal\s*\(\s*\|ui\|\s*\{'), r'crate::app::layout::horizontal(ui, self.is_rtl(), |ui| {'),
    (re.compile(r'\bui\.horizontal_wrapped\s*\(\s*\|ui\|\s*\{'), r'crate::app::layout::horizontal_wrapped(ui, self.is_rtl(), |ui| {'),
    (re.compile(r'\bui\.horizontal_centered\s*\(\s*\|ui\|\s*\{'), r'crate::app::layout::horizontal_centered(ui, self.is_rtl(), |ui| {'),
]

for filename in os.listdir(ui_dir):
    if not filename.endswith(".rs"):
        continue
    filepath = os.path.join(ui_dir, filename)
    with open(filepath, "r", encoding="utf-8") as f:
        content = f.read()
    original = content
    for pattern, replacement in patterns:
        content, n = pattern.subn(replacement, content)
        count += n
    if content != original:
        with open(filepath, "w", encoding="utf-8") as f:
            f.write(content)

print(f"Replaced {count} occurrences of horizontal layouts.")
