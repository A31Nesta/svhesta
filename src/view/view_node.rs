use alloc::{string::String, vec::Vec};
use embedded_graphics::Drawable;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::primitives::{CornerRadii, RoundedRectangle};
use embedded_graphics::text::Text;
use embedded_graphics::{
    mono_font::{MonoFont, iso_8859_16::FONT_6X10},
    pixelcolor::Rgb565,
    prelude::{Point, Primitive, RgbColor, Size},
    primitives::{PrimitiveStyleBuilder, Rectangle},
};
use typed_builder::TypedBuilder;

use crate::cardputer::{DISPLAY_SIZE_HEIGHT, DISPLAY_SIZE_WIDTH, SVDisplay};

/// Direction of the children elements
pub enum Direction {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}
pub enum JustifyContent {
    Start,
    End,
    Center,
    SpaceBetween,
    SpaceAround,
}
pub enum AlignItems {
    Start,
    End,
    Center,
    // No stretch
    // No baseline
}

#[derive(Clone, Debug)]
pub struct Area {
    pub top_left: Point,
    pub size: Size,
}
impl Default for Area {
    fn default() -> Self {
        Self {
            top_left: Point::zero(),
            size: Size::new(DISPLAY_SIZE_WIDTH as u32, DISPLAY_SIZE_HEIGHT as u32),
        }
    }
}

/// CSS/Web Flex-ish layout node. IT USES ALLOC!
#[derive(TypedBuilder)]
pub struct ViewNode {
    #[builder(default)]
    children: Vec<ViewNode>,

    #[builder(default)]
    default_area: Area,
    #[builder(default)]
    max_area: Area,

    // Layout Parameters
    #[builder(default = Direction::Row)]
    direction: Direction,
    #[builder(default = JustifyContent::Start)]
    justify_content: JustifyContent,
    #[builder(default = AlignItems::Start)]
    align_items: AlignItems,

    // Parameters of THIS node
    #[builder(default = None, setter(strip_option))]
    text: Option<String>,
    #[builder(default = &FONT_6X10)]
    font: &'static MonoFont<'static>,
    #[builder(default = None, setter(strip_option))]
    width: Option<u16>,
    #[builder(default = None, setter(strip_option))]
    height: Option<u16>,

    #[builder(default = 0)]
    margin: u16,
    #[builder(default = 0)]
    padding: u16,

    // Visual config
    #[builder(default = Rgb565::WHITE)]
    color: Rgb565,
    #[builder(default = None, setter(strip_option))]
    background_color: Option<Rgb565>,

    #[builder(default = Rgb565::WHITE)]
    border_color: Rgb565,
    #[builder(default = 0)]
    border_width: u8,
    #[builder(default = 0)]
    border_radius: u8,
}

impl ViewNode {
    /// Adds a child to this node
    pub fn child(mut self, child: Self) -> Self {
        self.children.push(child);
        self
    }

    /// Render
    pub fn render(&self, display: &mut SVDisplay) {
        // If we have a background
        if self.background_color.is_some() || self.border_width > 0 {
            let mut bg_style_builder = PrimitiveStyleBuilder::new();

            // Background Color
            if let Some(bg_color) = self.background_color {
                bg_style_builder = bg_style_builder.fill_color(bg_color);
            }
            // Border color and width
            if self.border_width > 0 {
                bg_style_builder = bg_style_builder
                    .stroke_width(self.border_width as u32)
                    .stroke_color(self.border_color);
            }

            let bg_style = bg_style_builder.build();

            // Create rectangle with size that respects margins
            let rectangle = Rectangle {
                top_left: Point {
                    x: self.default_area.top_left.x + self.margin as i32,
                    y: self.default_area.top_left.y + self.margin as i32,
                },
                size: Size {
                    width: self.default_area.size.width - (2 * self.margin as u32),
                    height: self.default_area.size.height - (2 * self.margin as u32),
                },
            };

            // Draw rectangle or rounded rectangle depending on border radius
            if self.border_radius > 0 {
                RoundedRectangle::new(
                    rectangle,
                    CornerRadii::new(Size {
                        width: self.border_radius as u32,
                        height: self.border_radius as u32,
                    }),
                )
                .into_styled(bg_style)
                .draw(display)
                .unwrap();
            } else {
                rectangle.into_styled(bg_style).draw(display).unwrap();
            }
        }

        // Then, we draw the text
        if let Some(text) = &self.text {
            let drawable_text = Text::new(
                text,
                // Point, we also have to add margin AND padding
                Point {
                    x: self.default_area.top_left.x + (self.margin + self.padding) as i32,
                    y: self.default_area.top_left.y
                        + self.font.character_size.width as i32
                        + (self.margin + self.padding) as i32,
                },
                MonoTextStyle::new(self.font, self.color.clone()),
            );
            drawable_text.draw(display).unwrap();
        }
    }
    /// Render entire Tree
    pub fn render_tree(&self, display: &mut SVDisplay) {
        // Draw this node
        self.render(display);
        // Draw all nodes
        for node in &self.children {
            node.render_tree(display);
        }
    }
}
