# Terminal Palette Designer

## Last Edit

inside ./src/app/color.rs, switch to Oklab

## Todo

- [x] center align grid
- [x] black bg
- [x] display cursor
- [x] show hex code
- [x] input as hex code (see json)
  - [x] `i`: enter insert mode
    - [x] `enter`: exit insert mode and save
    - [x] `enter`: exit insert mode, ignore changes
    - [x] `<C-w>`: delete text
    - [ ] movable cursor with arrow keys during insert mode
- [x] render inputted text
- [x] `<S-a>`: add color to the right, enter insert mode
- [x] `<S-a>`: add color to end, enter insert mode
- [?] builtin clipboard
- [x] `y`: yank to clipboard
- [x] `d`: yank to clipboard, delete color
- [x] `p`: paste clipboard after cursor
- [x] `<S-p>`: paste clipboard before cursor
- [x] leader key functionality
- [x] `<S-r>`: replace color with clipboard at cursor
- [?] system-clipboard compatibility
- [?] `<space>y`: yank to system-clipboard
- [x] `<space>p`: insert system-clipboard at cursor
- [x] `<space><S-r>`: replace color with system-clipboard at cursor
- [x] `h` `j` `k` `l`
- [ ] color leader_mode
  - [ ] multiplier indicator
  - [ ] h/H for hue shift on selected color
  - [ ] s/S for saturation shift on selected color
  - [ ] v/V for value shift on selected color
  - [ ] more keybinds like above for othor colorspaces

- [ ] multiselection
- [ ] color fg text according to swatch color, so it's always readable
- [ ] toggle cursor, text
- [ ] set bg color
- [ ] undo/redo

## Color Mode

Z -> enter one time color mode

## References

- [for text input](https://ratatui.rs/tutorials/json-editor/)
- [color palette (not generator) in terminal](https://crates.io/crates/material)
