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
#![no_std]
#![no_main]

use panic_halt as _;

use max7219::MAX7219;
use max7219_canvas::{ layer::CanvasLayer, DisplayCanvas };

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Replace with your pin setup

    let din = pins.d13.into_output();
    let cs = pins.d12.into_output();
    let clk = pins.d11.into_output();

    // Setup the display

    let mut display = MAX7219::from_pins(1 /* 1 display */, din, cs, clk).unwrap();
    display.power_on().unwrap();
    display.set_intensity(0, 0x0f).unwrap(); // Set intensity of the first display to maximum

    // Create display canvas

    let mut canvas: DisplayCanvas<2 /* 2 layers */, 1 /* 1 display */> = DisplayCanvas::new();

    // Create a static layer that never changes

    let mut static_layer = CanvasLayer::new();

    // Smiley face

    static_layer.set_pixel(1, 3, true);
    static_layer.set_pixel(1, 2, true);
    static_layer.set_pixel(2, 1, true);
    static_layer.set_pixel(3, 1, true);
    static_layer.set_pixel(4, 1, true);
    static_layer.set_pixel(5, 1, true);
    static_layer.set_pixel(6, 2, true);
    static_layer.set_pixel(6, 3, true);

    static_layer.set_pixel(1, 6, true);
    static_layer.set_pixel(2, 6, true);
    static_layer.set_pixel(1, 5, true);
    static_layer.set_pixel(2, 5, true);

    static_layer.set_pixel(6, 6, true);
    static_layer.set_pixel(5, 6, true);
    static_layer.set_pixel(6, 5, true);
    static_layer.set_pixel(5, 5, true);

    // Create an animated layer

    let mut animated_layer = CanvasLayer::new();
    let mut animation_position: usize = 0;

    loop {
        // Update the animated layer

        animated_layer.clear();
        animated_layer.set_pixel(animation_position, animation_position, true);

        animation_position = (animation_position + 1) % 8;

        // Update the display

        canvas.update_layer(0, static_layer);
        canvas.update_layer(1, animated_layer);

        canvas.write_to_display(0, &mut display); // Display the canvas to the first display

        arduino_hal::delay_ms(100);
    }
}

```