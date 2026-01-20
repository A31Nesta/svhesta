use esp_hal::{
    gpio::{Level, Output, OutputConfig},
    peripherals::Peripherals,
};

mod display;

use crate::cardputer::display::{DisplayPeripherals, get_display};
pub use display::{AVDisplay, DISPLAY_SIZE_HEIGHT, DISPLAY_SIZE_WIDTH};

/// The Cardputer's features in one package, similar to M5Unified
pub struct Cardputer {
    pub display: AVDisplay,
    pub backlight: Output<'static>,
}

impl Cardputer {
    pub fn new(peripherals: Peripherals) -> Self {
        // Initialize Display
        let display_peripherals = DisplayPeripherals {
            spi: peripherals.SPI2,
            sck: peripherals.GPIO36,
            mosi: peripherals.GPIO35,
            cs: peripherals.GPIO37,
            dc: peripherals.GPIO34,
            rst: peripherals.GPIO33,
        };
        let display = get_display(display_peripherals);

        // Initialize Backlight
        let backlight = Output::new(peripherals.GPIO38, Level::Low, OutputConfig::default());

        // Build and return
        return Cardputer { display, backlight };
    }
}
