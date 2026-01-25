use alloc::string::String;
use esp_backtrace as _;
use esp_hal::gpio::Input;

mod view;

use crate::{
    cardputer::{SVDisplay, keyboard::AdvKeyboard},
    event::{SVEvent, SViewEvent, event_receive, event_send, keys::SVKey, log, view_event_receive},
    tasks::view::draw_terminal,
};

#[embassy_executor::task]
pub async fn input_g0(mut g0: Input<'static>) {
    // log("Input task running").await;
    loop {
        g0.wait_for_falling_edge().await;
        event_send(SVEvent::G0Down).await;

        g0.wait_for_rising_edge().await;
        event_send(SVEvent::G0Up).await;
    }
}

/// Keyboard Task
#[embassy_executor::task]
pub async fn input_keyboard(mut keyboard: AdvKeyboard, mut keyboard_interrupt: Input<'static>) {
    // log("Keyboard task running").await;
    // Update once to flush fifo
    keyboard.update().await;

    loop {
        keyboard_interrupt.wait_for_low().await;
        keyboard.update().await;
    }
}

#[embassy_executor::task]
pub async fn output_display(mut display: SVDisplay) {
    // log("Display task running").await;
    draw_terminal(&mut display).await;

    loop {
        match view_event_receive().await {
            SViewEvent::RedrawTerminal => {
                draw_terminal(&mut display).await;
            }
        }
    }
}

#[embassy_executor::task]
pub async fn controller_main() {
    // log("Main Controller task running").await;

    let mut is_shift = false;
    let mut message = String::new();

    loop {
        match event_receive().await {
            SVEvent::KeyDown(key) => match key {
                SVKey::Shift => is_shift = true,
                SVKey::Fn | SVKey::Ctrl | SVKey::Opt | SVKey::Alt => (),
                SVKey::Backspace => {
                    message.pop();
                }
                SVKey::Enter => {
                    log(&message).await;
                    message.clear();
                }
                _ => {
                    message.push(key.char(is_shift).unwrap());
                }
            },
            SVEvent::KeyUp(key) => match key {
                SVKey::Shift => is_shift = false,
                _ => (),
            },
            _ => (),
        };
    }
}
