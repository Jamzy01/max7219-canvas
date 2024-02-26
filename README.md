# Max 7219 Canvas

A library built for rust that makes setting and getting pixels from co-ordinates aswell as handling displaying different layers on top of each other. Works with the [max7219](https://crates.io/crates/max7219) crate.

[![Crates.io][crates-badge]][crates-url]
[![Docs.rs][docs-badge]][docs-url]

[crates-badge]: https://img.shields.io/crates/v/max7219-canvas.svg
[crates-url]: https://crates.io/crates/max7219-canvas
[docs-badge]: https://docs.rs/max7219-canvas/badge.svg
[docs-url]: https://docs.rs/max7219-canvas

> [!CAUTION]
> Whilst this crate was built to support multiple displays, it has never been tested using multiple displays, use at your own discretion

## Example

```rust
use max7219::MAX7219;
```