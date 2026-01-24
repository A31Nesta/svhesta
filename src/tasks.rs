use alloc::format;
use esp_backtrace as _;
use esp_hal::gpio::Input;

use crate::{
    cardputer::SVDisplay,
    event::{SVEvent, TERMINAL, event_receive, event_send, keys::SVKey, log},
};

#[embassy_executor::task]
pub async fn input_g0(mut g0: Input<'static>) {
    log("Input task running").await;
    loop {
        g0.wait_for_falling_edge().await;
        event_send(SVEvent::KeyDown(SVKey::G0)).await;

        g0.wait_for_rising_edge().await;
        event_send(SVEvent::KeyUp(SVKey::G0)).await;
    }
}

#[embassy_executor::task]
pub async fn output_display(mut display: SVDisplay) {
    log("Display task running").await;
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
            SVEvent::KeyDown(key) => {
                log(&format!("Pressed key: {:?}", key)).await;
            }
            SVEvent::KeyUp(key) => {
                log(&format!("Released key: {:?}", key)).await;
            }
        }
    }
}
