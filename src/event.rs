#[cfg(debug_assertions)]
use defmt::info;

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel, mutex::Mutex};

use crate::{event::keys::SVKey, terminal::Terminal};

pub mod keys;

static EVENT_BUS: Channel<CriticalSectionRawMutex, SVEvent, 16> = Channel::new();
static VIEW_EVENT_BUS: Channel<CriticalSectionRawMutex, SViewEvent, 16> = Channel::new();

/// The global Terminal. All logs sent via the Log function end up here. Must be drawn
/// manually
pub static TERMINAL: Mutex<CriticalSectionRawMutex, Option<Terminal>> = Mutex::new(None);

#[derive(Debug, Clone, Copy)]
pub enum SVEvent {
    G0Up,
    G0Down,
    KeyUp(SVKey),
    KeyDown(SVKey),
}

#[derive(Debug, Clone, Copy)]
pub enum SViewEvent {
    RedrawTerminal,
}

/// Logs a message to the terminal and sends a Redraw event
pub async fn log(message: &str) {
    {
        let mut lock = TERMINAL.lock().await;
        let terminal = lock.as_mut().unwrap();
        terminal.push(message);
    }
    #[cfg(debug_assertions)]
    info!("{}", message);

    view_event_send(SViewEvent::RedrawTerminal).await;
}

/// Sends an event
pub async fn event_send(evt: SVEvent) {
    EVENT_BUS.send(evt).await;
}
/// Waits until the next sent event
pub async fn event_receive() -> SVEvent {
    EVENT_BUS.receive().await
}
/// Receives the current event or None if there aren't any.
pub fn event_receive_sync() -> Option<SVEvent> {
    EVENT_BUS.try_receive().ok()
}

/// Sends an event
pub async fn view_event_send(evt: SViewEvent) {
    VIEW_EVENT_BUS.send(evt).await;
}
/// Waits until the next sent event
pub async fn view_event_receive() -> SViewEvent {
    VIEW_EVENT_BUS.receive().await
}
/// Receives the current event or None if there aren't any.
pub fn view_event_receive_sync() -> Option<SViewEvent> {
    VIEW_EVENT_BUS.try_receive().ok()
}
