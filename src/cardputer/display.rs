use embedded_hal_bus::spi::{ExclusiveDevice, NoDelay};
use esp_hal::{
    Blocking,
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
    peripherals::{GPIO33, GPIO34, GPIO35, GPIO36, GPIO37, SPI2},
    spi::{
        Mode,
        master::{Config, Spi},
    },
    time::Rate,
};
use mipidsi::{
    Builder, Display,
    interface::SpiInterface,
    models::ST7789,
    options::{ColorInversion, ColorOrder::Rgb, Orientation, Rotation},
};
use static_cell::StaticCell;

/// Display width
pub const DISPLAY_SIZE_WIDTH: u16 = 240;
/// Display height
pub const DISPLAY_SIZE_HEIGHT: u16 = 135;

/// BUFFER SIZE
const DISPLAY_BUFFER_SIZE: usize = 100;
/// Display Buffer
static DISPLAY_BUFFER: StaticCell<[u8; DISPLAY_BUFFER_SIZE]> = StaticCell::new();

pub type AVDisplay = Display<
    SpiInterface<
        'static,
        ExclusiveDevice<Spi<'static, Blocking>, Output<'static>, NoDelay>,
        Output<'static>,
    >,
    ST7789,
    Output<'static>,
>;

pub struct DisplayPeripherals {
    pub spi: SPI2<'static>,
    pub sck: GPIO36<'static>,
    pub mosi: GPIO35<'static>,
    pub cs: GPIO37<'static>,
    pub dc: GPIO34<'static>,
    pub rst: GPIO33<'static>,
}

/// Obtain the Cardputer Display from the Peripherals
pub fn get_display(peripherals: DisplayPeripherals) -> AVDisplay {
    // To set up a TFT display we use:
    // - DATA: MOSI, output data
    // - SCK: Serial Clock
    // - RS / DC / A0: Register Select / Data and Command, to differentiate data and command signals
    // - RST: Reset Pin
    // - CS: Chip Select, to activate the device

    let spi_config = Spi::new(
        peripherals.spi,
        Config::default()
            .with_frequency(Rate::from_mhz(80))
            .with_mode(Mode::_0),
    )
    .unwrap()
    .with_sck(peripherals.sck) // Serial Clock
    .with_mosi(peripherals.mosi); // MOSI (Serial Out)

    let cs = Output::new(peripherals.cs, Level::Low, OutputConfig::default());
    let dc = Output::new(peripherals.dc, Level::Low, OutputConfig::default());
    let rst = Output::new(peripherals.rst, Level::High, OutputConfig::default());

    // let interface = SPIInterface::new(spi_config, dc);
    let device = ExclusiveDevice::new_no_delay(spi_config, cs).unwrap();
    let interface = SpiInterface::new(device, dc, DISPLAY_BUFFER.init([0; DISPLAY_BUFFER_SIZE]));

    let display = Builder::new(ST7789, interface)
        .invert_colors(ColorInversion::Inverted)
        .display_size(DISPLAY_SIZE_HEIGHT, DISPLAY_SIZE_WIDTH)
        .color_order(Rgb)
        .display_offset(52, 40)
        .orientation(Orientation {
            rotation: Rotation::Deg90,
            mirrored: false,
        })
        .reset_pin(rst)
        .init(&mut Delay::default())
        .unwrap();

    display
}
