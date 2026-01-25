//! Keyboard I2C Driver for the Cardputer ADV.
//!
//! I2C specifics adapted from the Cardputer crate for
//! ESP-IDF-HAL and from the official Cardputer ADV
//! schematics:
//!
//! - https://github.com/syurazo/cardputer/blob/3c8f7d8fb2cfd352bcb92c0c96d305e080438772
//! - https://m5stack-doc.oss-cn-shenzhen.aliyuncs.com/1178/Sch_M5CardputerAdv_v1.0_2025_06_20_17_19_58_page_02.png

#[cfg(debug_assertions)]
use defmt::info;
use esp_hal::{
    Async,
    i2c::master::{Config, I2c},
    peripherals::{GPIO8, GPIO9, I2C0},
    time::Rate,
};

use crate::{
    cardputer::keyboard::adv_constants::{
        ADDR_CFG, ADDR_KP_GPIO1, ADDR_KP_GPIO2, ADDR_KP_GPIO3, I2C_ADDRESS, INT_STATUS, KEY_MATRIX,
        REG_KEY_EVENT_A, REG_KEY_LCK_EC,
    },
    event::{SVEvent, event_send},
};

mod adv_constants;

pub struct AdvKeyboardPeripherals {
    pub i2c: I2C0<'static>,
    pub sda: GPIO8<'static>,
    pub scl: GPIO9<'static>,
}

pub struct AdvKeyboard {
    i2c: I2c<'static, Async>,
}

impl AdvKeyboard {
    pub async fn new(peripherals: AdvKeyboardPeripherals) -> Self {
        let mut i2c = I2c::new(
            peripherals.i2c,
            Config::default().with_frequency(Rate::from_hz(400_000)),
        )
        .unwrap()
        .with_scl(peripherals.scl)
        .with_sda(peripherals.sda)
        .into_async();

        // CONFIGURE I2C:
        // --------------
        // The Cardputer ADV schematic for the wiring of the TCA8418RTWR indicates
        // that it uses the ROWS 0-6 (not 7) and COLS 0-7 (not 8 or 9). We configure
        // that like so:

        // ADDR_KP_GPIO1 is for ROWS: We'll use 7 out of 8 possible rows:
        i2c.write_async(I2C_ADDRESS, &[ADDR_KP_GPIO1, 0b0111_1111])
            .await
            .unwrap();
        // ADDR_KP_GPIO2 is for the first 8 COLS (0-7): We'll use all of them since we need exactly 8:
        i2c.write_async(I2C_ADDRESS, &[ADDR_KP_GPIO2, 0b1111_1111])
            .await
            .unwrap();
        // ADDR_KP_GPIO3 is for COLS 8 and 9. We won't be using them:
        i2c.write_async(I2C_ADDRESS, &[ADDR_KP_GPIO3, 0b0000_0000])
            .await
            .unwrap();

        // SET FIFO:
        // ---------
        // - Enable Key Event Interrupt (interrupt when FIFO is not empty)
        // - Clear Key Event Interrupt Status
        i2c.write_async(I2C_ADDRESS, &[ADDR_CFG, 0x01])
            .await
            .unwrap();
        i2c.write_async(I2C_ADDRESS, &[INT_STATUS, 0x00])
            .await
            .unwrap();

        Self { i2c }
    }

    /// Read all the events in the I2C FIFO queue
    pub async fn update(&mut self) {
        #[cfg(debug_assertions)]
        let mut counter = 0;
        loop {
            #[cfg(debug_assertions)]
            {
                info!("Keyboard Update Iteration: {}", counter);
                counter += 1;
            }

            let event_count = self.n_events_in_buffer().await;
            if event_count == 0 {
                break;
            }

            self.process_all_events(event_count).await;
        }

        // Clear FIFO
        self.i2c
            .write_async(I2C_ADDRESS, &[INT_STATUS, 0x01])
            .await
            .unwrap();
    }

    async fn n_events_in_buffer(&mut self) -> u8 {
        let mut i2c_buffer: [u8; 1] = [0xff];
        self.i2c
            .write_read_async(I2C_ADDRESS, &[REG_KEY_LCK_EC], &mut i2c_buffer)
            .await
            .unwrap();
        i2c_buffer[0] & 0x0f
    }

    async fn process_all_events(&mut self, event_count: u8) {
        #[cfg(debug_assertions)]
        info!("Updating Keyboard! Event count is {}", event_count);

        for _ in 0..event_count {
            // Read event, get key and send as SVEvent
            let mut i2c_buffer: [u8; 1] = [0xff];
            self.i2c
                .write_read_async(I2C_ADDRESS, &[REG_KEY_EVENT_A], &mut i2c_buffer)
                .await
                .unwrap();
            if i2c_buffer[0] == 0xff {
                break;
            }

            let pressed = i2c_buffer[0] & 0x80 == 0x80; // First bit is 1 or 0 (pressed or not)
            let key = i2c_buffer[0] & 0x7f; // Key is the remaining 7 bits
            let sv_key = KEY_MATRIX[(key - (key / 10) * 2 - 1) as usize]; // Black Magic

            if pressed {
                event_send(SVEvent::KeyDown(sv_key)).await;
            } else {
                event_send(SVEvent::KeyUp(sv_key)).await;
            }
        }
    }
}
