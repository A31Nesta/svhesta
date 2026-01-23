use defmt::info;
use esp_backtrace as _;
use esp_hal::gpio::Input;

use crate::{
    cardputer::SVDisplay,
    event::{SVEvent, TERMINAL, event_receive, event_send, keys::SVKey, log},
};

#[embassy_executor::task]
pub async fn input_g0(mut g0: Input<'static>) {
    info!("Input task running");
    loop {
        g0.wait_for_falling_edge().await;
        event_send(SVEvent::KeyDown(SVKey::G0)).await;
        log("Pressed G0!").await;

        g0.wait_for_rising_edge().await;
        event_send(SVEvent::KeyUp(SVKey::G0)).await;
        log("Released G0!").await;
    }
}

#[embassy_executor::task]
pub async fn output_display(mut display: SVDisplay) {
    info!("Display task running");
    // Draw once
    {
        let mut lock = TERMINAL.lock().await;
        let terminal = lock.as_mut().unwrap();
        terminal.draw(&mut display);
    }

    loop {
        match event_receive().await {
            SVEvent::RedrawTerminal => {
                let mut lock = TERMINAL.lock().await;
                let terminal = lock.as_mut().unwrap();
                terminal.draw(&mut display);
            }
            _ => (),
        }
    }
}
