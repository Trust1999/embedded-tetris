use std::time::Duration;

mod game;
use game::TetrisGame;

use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_sys as _; // Bindings für ESP-IDF
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn setup() {
    // Vier GPIO-Eingänge mit Pull-Up
    let mut button01 = PinDriver::input(peripherals.pins.gpio4)?;
    button01.set_pull(Pull::Up)?;
    let mut button02 = PinDriver::input(peripherals.pins.gpio5)?;
    button02.set_pull(Pull::Up)?;
    let mut button03 = PinDriver::input(peripherals.pins.gpio6)?;
    button03.set_pull(Pull::Up)?;
    let mut button04 = PinDriver::input(peripherals.pins.gpio7)?;
    button04.set_pull(Pull::Up)?;
}

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let mut game = TetrisGame::new();

    for i in 0.. {
        let display = game.step(i);
        println!("{}", display);
        std::thread::sleep(Duration::from_millis(500));
    }
}
