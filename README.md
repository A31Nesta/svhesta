# Svhesta

A simple template with basic drivers to simplify the development of
`no_std` Rust-based firmware for the M5Stack Cardputer ADV.

> [!WARNING]  
> Since I don't have a Cardputer (original or 1.1), **the keyboard** driver
> in this firmware **only works on the Cardputer ADV**.

## Features

I'm using `esp-hal` instead of `esp-idf-hal`, meaning that this project
uses `no_std` Rust and I can't use any of the C-based IDF Components.

There are 2 Cardputer crates that already configure drivers for some
peripherals but they're both for `esp-idf-hal` and `std`, so I used
them as reference and made my own `esp-hal` implementation. You
can check them out in the `cardputer` module of this project.

These are the implemented drivers:

- [x] **G0**
- [x] **Display**
- [x] **Cardputer ADV Keyboard**
    - [ ] **Cardputer** (not ADV) **Keyboard**: Can't test
- [ ] **SD Card**
- [ ] **IR Emitter**
- [ ] **Audio In/Out**
- [ ] **Expansions like LoRa Cap**
- [ ] **IMU** (Gyro)

---

<sub>If you're curious about the name I just made it the fuck up</sub>
