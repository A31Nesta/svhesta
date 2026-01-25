//! Keyboard constants for the Cardputer ADV Keyboard driver
//!
//! Constants are taken from the files `keyboard.rs` and
//! `adv/keyboard.rs` in the Cardputer crate for ESP-IDF-HAL:
//!
//! - https://github.com/syurazo/cardputer/blob/3c8f7d8fb2cfd352bcb92c0c96d305e080438772/src/keyboard.rs
//! - https://github.com/syurazo/cardputer/blob/3c8f7d8fb2cfd352bcb92c0c96d305e080438772/src/adv/keyboard.rs

use crate::event::keys::SVKey;

/// I2C timeout in milliseconds
// pub const I2C_TIMEOUT_MS: u64 = 500u64;

/// 7bit I2C address
pub const I2C_ADDRESS: u8 = 0x34;

/// Configuration Register
///
/// Addr | bit7 | bit6 | bit5 | bit4 | bit3 | bit2 | bit1 | bit0
/// :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :---
/// 0x01 | `--` | `--` | `--` | `--` | `--` | `--` | `--` | `KE_IEN`
///
/// ## `KEIEN`: Key Event Interrupt Enable
///
/// * 0: disabled
/// * 1: enabled
///
pub const ADDR_CFG: u8 = 0x01;
/// Interrupt Status Register
///
/// Addr | bit7 | bit6 | bit5 | bit4 | bit3 | bit2 | bit1 | bit0
/// :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :------
/// 0x02 | `--` | `--` | `--` | `--` | `--` | `--` | `--` | `K_INT`
///
/// ## `K_INT`: Key Event Interrupt Status
///
/// * 0: not detected
/// * 1: detected
///
pub const INT_STATUS: u8 = 0x02;
/// Key Lock / Event Counter Register
///
/// Addr | bit7 | bit6 | bit5 | bit4 | bit3 | bit2 | bit1 | bit0
/// :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :---
/// 0x03 | `--` | `--` | `--` | `--` | KEC3 | KEC2 | KEC1 | KEC0
///
pub const REG_KEY_LCK_EC: u8 = 0x03;
/// Key Event Register
///
/// Addr | bit7 | bit6 | bit5 | bit4 | bit3 | bit2 | bit1 | bit0
/// :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :---
/// 0x04 | KEA7 | KEA6 | KEA5 | KEA4 | KEA3 | KEA2 | KEA1 | KEA0
///
pub const REG_KEY_EVENT_A: u8 = 0x04;
/// Keypad or GPIO Selection Register (1)
///
/// Addr | bit7 | bit6 | bit5 | bit4 | bit3 | bit2 | bit1 | bit0
/// :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :---
/// 0x1D | ROW7 | ROW6 | ROW5 | ROW4 | ROW3 | ROW2 | ROW1 | ROW0
///
pub const ADDR_KP_GPIO1: u8 = 0x1D;
/// Keypad or GPIO Selection Register (2)
///
/// Addr | bit7 | bit6 | bit5 | bit4 | bit3 | bit2 | bit1 | bit0
/// :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :---
/// 0x1E | COL7 | COL6 | COL5 | COL4 | COL3 | COL2 | COL1 | COL0
///
pub const ADDR_KP_GPIO2: u8 = 0x1E;
/// Keypad or GPIO Selection Register (3)
///
/// Addr | bit7 | bit6 | bit5 | bit4 | bit3 | bit2 | bit1 | bit0
/// :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :---
/// 0x1F | N/A  | N/A  | N/A  | N/A  | N/A  | N/A  | COL9 | COL8
///
pub const ADDR_KP_GPIO3: u8 = 0x1F;

/// Key conversion table indexed from bit 7 to bit 0 of `REG_KEY_EVENT_A`
///
///  H/L | 1       | 2    | 3    | 4    | 5    | 6    | 7    | 8    | 9    | 10
///  --: | :---    | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :---
///   00 | `` ` `` | TAB  | FN   | CTRL | `1`  | `Q`  | SHFT | OPT  | n/a  | n/a
///   10 | `2`     | `W`  | `A`  | ALT  | `3`  | `E`  | `S`  | `Z`  | n/a  | n/a
///   20 | `4`     | `R`  | `D`  | `X`  | `5`  | `T`  | `F`  | `C`  | n/a  | n/a
///   30 | `6`     | `Y`  | `G`  | `V`  | `7`  | `U`  | `H`  | `B`  | n/a  | n/a
///   40 | `8`     | `I`  | `J`  | `N`  | `9`  | `O`  | `K`  | `M`  | n/a  | n/a
///   50 | `0`     | `P`  | `L`  | `,`  | `-`  | `[`  | `;`  | `.`  | n/a  | n/a
///   60 | `=`     | `]`  | `'`  | `/`  | BS   | `\`  | ENTR | SPC  | n/a  | n/a
///
pub const KEY_MATRIX: [SVKey; 56] = [
    SVKey::Backquote,
    SVKey::Tab,
    SVKey::Fn,
    SVKey::Ctrl,
    SVKey::One,
    SVKey::Q,
    SVKey::Shift,
    SVKey::Opt,
    SVKey::Two,
    SVKey::W,
    SVKey::A,
    SVKey::Alt,
    SVKey::Three,
    SVKey::E,
    SVKey::S,
    SVKey::Z,
    SVKey::Four,
    SVKey::R,
    SVKey::D,
    SVKey::X,
    SVKey::Five,
    SVKey::T,
    SVKey::F,
    SVKey::C,
    SVKey::Six,
    SVKey::Y,
    SVKey::G,
    SVKey::V,
    SVKey::Seven,
    SVKey::U,
    SVKey::H,
    SVKey::B,
    SVKey::Eight,
    SVKey::I,
    SVKey::J,
    SVKey::N,
    SVKey::Nine,
    SVKey::O,
    SVKey::K,
    SVKey::M,
    SVKey::Zero,
    SVKey::P,
    SVKey::L,
    SVKey::Comma,
    SVKey::Minus,
    SVKey::OpenBracket,
    SVKey::SemiColon,
    SVKey::Period,
    SVKey::Equal,
    SVKey::CloseBracket,
    SVKey::Quote,
    SVKey::Slash,
    SVKey::Backspace,
    SVKey::Backslash,
    SVKey::Enter,
    SVKey::Space,
];
