use crate::{cardputer::SVDisplay, event::TERMINAL};

pub async fn draw_terminal(display: &mut SVDisplay) {
    let mut lock = TERMINAL.lock().await;
    let terminal = lock.as_mut().unwrap();
    terminal.draw(display);
}
