/// A layer of the display canvas
/// `W` represents how many displays there are
#[derive(Clone, Copy)]
pub struct CanvasLayer<const W: usize> {
    pub(super) buffer: [[u8; 8]; W],
}

impl<const W: usize> Default for CanvasLayer<W> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const W: usize> CanvasLayer<W> {
    pub fn new() -> Self {
        CanvasLayer {
            buffer: [[0; 8]; W],
        }
    }

    pub fn clear(&mut self) {
        self.buffer = [[0; 8]; W];
    }

    #[allow(clippy::precedence)]
    pub fn set_pixel(&mut self, x: usize, y: usize, value: bool) {
        let display_index = x / 8;

        if x < W * 8 && y < 8 {
            let mask = 1 << x % 8;
            if value {
                self.buffer[display_index][y] |= mask;
            } else {
                self.buffer[display_index][y] &= !mask;
            }
        }
    }

    #[allow(clippy::precedence)]
    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        let display_index = x / 8;

        if x < W * 8 && y < 8 {
            let mask = 1 << x % 8;
            (self.buffer[display_index][y] & mask) != 0
        } else {
            false
        }
    }

    pub fn is_full(&self) -> bool {
        for display in self.buffer {
            for row in display {
                if row != 0xff {
                    return false;
                }
            }
        }

        true
    }

    pub fn is_empty(&self) -> bool {
        for display in self.buffer {
            for row in display {
                if row != 0 {
                    return false;
                }
            }
        }

        true
    }
}
