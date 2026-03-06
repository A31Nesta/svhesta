use alloc::string::ToString;
use embedded_graphics::{
    mono_font::jis_x0201::FONT_10X20,
    pixelcolor::Rgb565,
    prelude::{RgbColor, WebColors},
};

use crate::{cardputer::SVDisplay, view::view_node::ViewNode};

pub mod view_node;

pub struct View {
    root: ViewNode,
}

impl View {
    pub fn new() -> Self {
        let root = ViewNode::builder()
            .margin(8)
            .padding(8)
            .background_color(Rgb565::CSS_DARK_GOLDENROD)
            .border_color(Rgb565::CSS_ORANGE)
            .border_width(1)
            .border_radius(12)
            .text("Hello, World!".to_string())
            .font(&FONT_10X20)
            .build();

        View { root }
    }

    pub fn draw(&self, display: &mut SVDisplay) {
        self.root.render_tree(display);
    }
}
