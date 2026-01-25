# Svhesta

A lightweight collection of utilities for Cardputer ADV users who are not
interested in hacking. Svhesta aims to make the Cardputer ADV into a
useful tool for end-users.

> [!WARNING]  
> This firmware is currently a WORK IN PROGRESS.  
> Since I'm using Rust, I can't use M5Unified like you would with C++
> and Arduino or PlatformIO (or ESP-IDF); I have to make or configure
> drivers for every peripheral (like the Display or Keyboard).
> 
> Since I don't have a Cardputer (original or 1.1), **the keyboard** driver
> in this firmware **only works on the Cardputer ADV**.

## The goal of Svhesta

The aim of this firmware is **to be as comfortable as possible**.
This is why I focus on these points:

- **Small size**: Fast installation via Firmware
- **Agilily**: Shortcuts, simple UI and search features

## Svhesta Remote

The main feature of Svhesta. It's like
[Ultimate Remote by geo-tp](https://github.com/geo-tp/Ultimate-Remote)
but with a relational database on the SD card instead of a bundled
infrared database.

The SQLite database can be downloaded or built using
[IRDB2SQL](https://github.com/A31Nesta/irdb2sql) on the
Flipper Zero IRDB.

> [!NOTE]  
> IRDB2SQL is currently private. In the releases of the repository
> I'll put prebuilt SQLite DBs for Svhesta.
>
> The repository will become public once Svhesta Remote functionality
> is fully planned and once SD Card drivers are implemented.

## Notes and the technical side

As you already know from the little "Languages" bar on the side, this
firmware is made with Rust instead of C++. To work with the ESP32 i'm
using `esp-hal` instead of `esp-idf-hal`, meaning that this project uses
`no_std` Rust.

There are 2 Cardputer crates that already configure drivers for some
peripherals but they're both for `esp-idf-hal` and `std`, so I used
them as reference and made my own `esp-hal` implementation. You
can check them out in the `cardputer` module of this project.

For now I'm mostly just implementing drivers for the peripherals I need.
This is the roadmap for implementations and drivers:

- [x] **G0**: First test with input in Embassy
- [x] **Display**
- [x] **Cardputer ADV Keyboard**
    - [ ] **Cardputer** (not ADV) **Keyboard**: Can't test
- [ ] **SD Card**: Works with SPI like the display, investigate how SDs work in embedded
- [ ] **IR Emitter**: Just a GPIO Output... right??? (yes but complicated)

This means that I probably won't be implementing (for now at least):

- Audio In/Out
- Expansions like LoRa Cap
- Gyro

---

<sub>If you're curious about the name I just made it the fuck up</sub>
