use embedded_graphics::{
    Drawable,
    mono_font::{MonoTextStyle, iso_8859_7::FONT_8X13_BOLD},
    pixelcolor::{Rgb565, Rgb888},
    prelude::{Point, Primitive, Size, WebColors},
    primitives::{Circle, CornerRadii, PrimitiveStyleBuilder, Rectangle, RoundedRectangle},
    text::Text,
};

use crate::{
    cardputer::{DISPLAY_SIZE_HEIGHT, DISPLAY_SIZE_WIDTH, SVDisplay},
    event::TERMINAL,
};

pub async fn draw_terminal(display: &mut SVDisplay) {
    let mut lock = TERMINAL.lock().await;
    let terminal = lock.as_mut().unwrap();
    terminal.draw(display);
}

pub async fn draw_tiny_widgets_demo(display: &mut SVDisplay) {
    let bg_style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb888::new(222, 76, 128).into())
        .build();

    let widget_style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb888::new(255, 200, 200).into())
        .build();

    let icon_style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::CSS_NAVY)
        .build();

    // BG
    Rectangle::new(
        Point::zero(),
        Size::new(DISPLAY_SIZE_WIDTH.into(), DISPLAY_SIZE_HEIGHT.into()),
    )
    .into_styled(bg_style)
    .draw(display)
    .unwrap();

    // Header
    Rectangle::new(Point::zero(), Size::new(DISPLAY_SIZE_WIDTH.into(), 15))
        .into_styled(widget_style)
        .draw(display)
        .unwrap();
    // Header text
    Text::new(
        "Tiny Widgets",
        Point { x: 4, y: 11 },
        MonoTextStyle::new(&FONT_8X13_BOLD, Rgb565::CSS_NAVY),
    )
    .draw(display)
    .unwrap();
    // Draw icons in bar
    for i in 0..3 {
        Circle::new(
            Point {
                x: ((DISPLAY_SIZE_WIDTH - 17) - 12 * i) as i32,
                y: 2,
            },
            10,
        )
        .into_styled(icon_style)
        .draw(display)
        .unwrap();
    }

    // Widgets
    let mut small_first = true;

    let lft_mrgn = 40;
    let top_mrgn = 24;
    let wdgt_sx = 50;
    let wdgt_sy = 40;
    let pad = 8;

    for i in 0..3 {
        RoundedRectangle::new(
            Rectangle {
                top_left: Point {
                    x: lft_mrgn,
                    y: top_mrgn + (i * (wdgt_sy + pad)) as i32,
                },
                size: Size {
                    width: if small_first { wdgt_sx } else { 2 * wdgt_sx },
                    height: wdgt_sy,
                },
            },
            CornerRadii::new(Size::new_equal(6)),
        )
        .into_styled(widget_style)
        .draw(display)
        .unwrap();

        RoundedRectangle::new(
            Rectangle {
                top_left: Point {
                    x: if small_first {
                        lft_mrgn + (wdgt_sx + pad) as i32
                    } else {
                        lft_mrgn + (2 * wdgt_sx + pad) as i32
                    },
                    y: top_mrgn + (i * (wdgt_sy + pad)) as i32,
                },
                size: Size {
                    width: if !small_first { wdgt_sx } else { 2 * wdgt_sx },
                    height: wdgt_sy,
                },
            },
            CornerRadii::new(Size::new_equal(6)),
        )
        .into_styled(widget_style)
        .draw(display)
        .unwrap();

        small_first = !small_first;
    }
}
