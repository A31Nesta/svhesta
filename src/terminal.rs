use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use embedded_graphics::{
    Drawable,
    mono_font::{MonoTextStyle, ascii::FONT_6X10},
    pixelcolor::{Rgb565, Rgb888},
    prelude::{Point, Primitive, RgbColor, Size},
    primitives::{
        CornerRadii, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, RoundedRectangle, Styled,
    },
    text::Text,
};

use crate::cardputer::{AVDisplay, DISPLAY_SIZE_HEIGHT, DISPLAY_SIZE_WIDTH};

const TERMINAL_MAX_LINES: usize = 20;
const TERMINAL_SCREEN_SIZE: usize = ((DISPLAY_SIZE_HEIGHT - 20) / 10) as usize;
/// Maximum amout of characters per line considering font size and paddings
const TERMINAL_MAX_CHARS: usize = ((DISPLAY_SIZE_WIDTH - 20) / 6) as usize;

pub struct Terminal {
    panel: Styled<RoundedRectangle, PrimitiveStyle<Rgb565>>,
    buffer: Vec<String>,
}

impl Terminal {
    pub fn new() -> Self {
        let panel_style = PrimitiveStyleBuilder::new()
            .stroke_width(1)
            .stroke_color(Rgb888::new(132, 110, 63).into())
            .fill_color(Rgb565::new(1, 4, 1))
            .build();

        let padding: u32 = 5;
        let panel = RoundedRectangle::new(
            Rectangle::new(
                Point::new_equal(padding as i32),
                Size {
                    width: (DISPLAY_SIZE_WIDTH as u32) - 2 * padding,
                    height: (DISPLAY_SIZE_HEIGHT as u32) - 2 * padding,
                },
            ),
            CornerRadii::new(Size::new_equal(8)),
        )
        .into_styled(panel_style);
        Self {
            panel,
            buffer: Vec::with_capacity(TERMINAL_MAX_LINES),
        }
    }

    pub fn push(&mut self, string: &str) {
        // Split the string into lines
        let mut processed: Vec<String> = string
            .split('\n')
            .flat_map(|line| {
                if line.len() <= TERMINAL_MAX_CHARS {
                    alloc::vec![line.to_string()]
                } else {
                    let mut chars = line.chars();
                    (0..)
                        .map(|_| chars.by_ref().take(TERMINAL_MAX_CHARS).collect::<String>())
                        .take_while(|s| !s.is_empty())
                        .collect::<Vec<_>>()
                }
            })
            .collect();

        // Add new lines to buffer
        self.buffer.append(&mut processed);

        // Delete excess lines in buffer
        if self.buffer.len() > TERMINAL_MAX_LINES {
            let excess = self.buffer.len() - TERMINAL_MAX_LINES;
            self.buffer.drain(0..excess);
        }
    }

    pub fn draw(&mut self, display: &mut AVDisplay) {
        self.draw_panel(display);
        self.draw_text(display);
    }

    fn draw_panel(&mut self, display: &mut AVDisplay) {
        self.panel.draw(display).unwrap();
    }

    fn draw_text(&mut self, display: &mut AVDisplay) {
        // create String to draw
        let buf_len = self.buffer.len();
        let text_str = if buf_len > TERMINAL_SCREEN_SIZE {
            &self.buffer[buf_len - TERMINAL_SCREEN_SIZE..buf_len]
        } else {
            &self.buffer[..]
        }
        .join("\n");

        let text = Text::new(
            &text_str,
            Point::new(10, 16),
            MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE),
        );
        text.draw(display).unwrap();
    }
}
