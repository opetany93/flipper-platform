# flipper-platform

Safe Rust wrappers over the Flipper Zero firmware, for the parts the
[`flipperzero`](https://crates.io/crates/flipperzero) crate does not cover.

- **Borrowed callbacks.** View port and timer callbacks are plain closures
  borrowed for the wrapper's lifetime: no boxing, no `'static`, and the borrow
  checker guarantees they are detached before anything they capture goes away.
- **RAII everywhere.** Every wrapper releases what it acquired on drop.
- **No `unsafe` at the call site.**

## Modules

| Module      | What it wraps                                              |
| ----------- | ---------------------------------------------------------- |
| `view_port` | A fullscreen view port with draw and input callbacks       |
| `canvas`    | Text, lines and XBM bitmaps on the canvas a draw call gets |
| `input`     | Key events as exhaustively matchable enums                 |
| `timer`     | Periodic Furi timers                                       |
| `backlight` | Keeping the display backlight on while a guard is alive    |

## Example

```rust
use flipper_platform::backlight::BacklightLock;
use flipper_platform::canvas::{Canvas, Font};
use flipper_platform::input::InputEvent;
use flipper_platform::view_port::ViewPort;

let _backlight = BacklightLock::acquire();

let on_draw = |canvas: &mut Canvas<'_>| {
    canvas.clear();
    canvas.set_font(Font::Bold);
    canvas.draw_str(2, 12, c"Hello, Flipper");
};
let on_input = |_event: InputEvent| {};

let view_port = ViewPort::fullscreen(&on_draw, &on_input);
```

Dropping `view_port` removes it from the GUI, and dropping `_backlight` hands
the backlight back to the firmware.

## Status

Early: the API covers what its first application needed and grows from there.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.
