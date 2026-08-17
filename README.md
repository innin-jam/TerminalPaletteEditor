# Terminal Palette Designer

This is a small TUI program for creating color palettes.

## Installation

Inside cloned repo:

```shell
cargo run
```

## Keybinds

Normal mode:

| Key | Function |
| --- | --- |
| hjkl / Arrow Keys | Move Cursor |
| y | Yank color to clipboard |
| p | Paste color from clipboard |
| <space>p | Paste color from system clipboard |
| i | Edit color string |
| c | Enter color mode |
| <ctrl>c | quit |

Color mode:

| Key | Function |
| --- | --- |
| a | decrease multiplier |
| x | decrease multiplier |
| l/L | increase/decrease lightness |
| h/H | increase/decraese hue |
| s/S | increase/decraese chroma |
