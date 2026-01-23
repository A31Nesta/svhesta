#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use defmt::info;
use embassy_executor::Spawner;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::RgbColor;
use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;
use svhesta::cardputer::Cardputer;
use svhesta::event::TERMINAL;
use svhesta::terminal::Terminal;
use {esp_backtrace as _, esp_println as _};

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 73744);

    // This is safe!! I never use TIMG0 anywhere else. I need to clone it
    // because I can't partially move peripherals before building the
    // Cardputer object.
    let timg0 = TimerGroup::new(unsafe { peripherals.TIMG0.clone_unchecked() });
    esp_rtos::start(timg0.timer0);

    info!("Embassy initialized!");

    // Init cardputer
    let mut cardputer = Cardputer::new(peripherals);
    info!("Cardputer initialized!");

    cardputer.display.clear(Rgb565::BLACK).unwrap();
    cardputer.backlight.set_high();

    // Init Terminal
    {
        *(TERMINAL.lock().await) = Some(Terminal::new());

        let mut lock = TERMINAL.lock().await;
        let terminal = lock.as_mut().unwrap();

        terminal.push("Embassy and Cardputer initialized.");
        terminal.push("Svhesta up and running!");
        // terminal.draw(&mut cardputer.display);
    }

    // TODO: Spawn tasks for Keyboard, Battery...
    spawner.spawn(svhesta::tasks::input_g0(cardputer.g0)).ok();
    spawner
        .spawn(svhesta::tasks::output_display(cardputer.display))
        .ok();

    // Yield (run other tasks)
    core::future::pending::<()>().await;
    loop {}
    // loop {
    //     // info!("Hello world!");
    //     Timer::after(Duration::from_secs(1)).await;
    // }
}
