#![no_std]

use max7219::{ connectors::Connector, MAX7219 };

use self::layer::CanvasLayer;

pub mod layer;

/// A display canvas for the Max 7219
///
/// `W` represents how many displays there are
/// `L` represents how many layers there are
///
/// Layers are simply stacked on top of each other where if a pixel is set in any of the layers then the pixel illuminates, therefore no layer has "priority" over the other and the order of layers is irrelevant
#[derive(Clone, Copy)]
pub struct DisplayCanvas<const L: usize, const W: usize> {
    buffer: [CanvasLayer<W>; L],
}

impl<const L: usize, const W: usize> Default for DisplayCanvas<L, W> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const L: usize, const W: usize> DisplayCanvas<L, W> {
    pub fn new() -> Self {
        DisplayCanvas {
            buffer: [CanvasLayer::default(); L],
        }
    }

    pub fn update_layer(&mut self, layer_index: usize, layer: CanvasLayer<W>) {
        self.buffer[layer_index] = layer;
    }

    pub fn as_raw_data(&self, display_index: usize) -> [u8; 8] {
        let mut buffer = [0; 8];

        for layer in 0..L {
            for (y, data) in buffer.iter_mut().enumerate() {
                *data |= self.buffer[layer].buffer[display_index][y];
            }
        }

        buffer
    }

    pub fn write_to_display<C: Connector>(&self, display_index: usize, display: &mut MAX7219<C>) {
        display
            .write_raw(display_index, &self.as_raw_data(display_index))
            .expect("Failed to write to display");
    }
}
