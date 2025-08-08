use std::collections::VecDeque;
use std::time::Duration;

mod game;
use game::TetrisGame;

use esp_idf_hal::gpio::*;
use esp_idf_hal::gpio::{self, PinDriver, Pull};
use esp_idf_hal::peripherals::Peripherals;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy)]
enum ButtonAction {
    MoveLeft,
    MoveRight,
    MoveDown,
    Rotate,
}

fn setup_buttons(
    btn1: Gpio4,
    btn2: Gpio5,
    btn3: Gpio6,
    btn4: Gpio7,
    queue: Arc<Mutex<VecDeque<ButtonAction>>>,
) -> anyhow::Result<()> {
    // Button1
    let mut button1 = PinDriver::input(btn1)?;
    button1.set_pull(Pull::Up)?;
    button1.set_interrupt_type(gpio::InterruptType::NegEdge)?;
    let queue1 = queue.clone();
    let _sub1 = unsafe {
        button1.subscribe(move || {
            queue1.lock().unwrap().push_back(ButtonAction::MoveLeft);
        })
    }?;

    // Button2
    let mut button2 = PinDriver::input(btn2)?;
    button2.set_pull(Pull::Up)?;
    button2.set_interrupt_type(gpio::InterruptType::NegEdge)?;
    let queue2 = queue.clone();
    let _sub2 = unsafe {
        button2.subscribe(move || {
            queue2.lock().unwrap().push_back(ButtonAction::MoveRight);
        })
    }?;

    // Button3
    let mut button3 = PinDriver::input(btn3)?;
    button3.set_pull(Pull::Up)?;
    button3.set_interrupt_type(gpio::InterruptType::NegEdge)?;
    let queue3 = queue.clone();
    let _sub3 = unsafe {
        button3.subscribe(move || {
            queue3.lock().unwrap().push_back(ButtonAction::MoveDown);
        })
    }?;

    // Button4
    let mut button4 = PinDriver::input(btn4)?;
    button4.set_pull(Pull::Up)?;
    button4.set_interrupt_type(gpio::InterruptType::NegEdge)?;
    let queue4 = queue.clone();
    let _sub4 = unsafe {
        button4.subscribe(move || {
            queue4.lock().unwrap().push_back(ButtonAction::Rotate);
        })
    }?;

    Ok(())
}

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;
    let mut led = PinDriver::output(pins.gpio48).unwrap();
    led.set_low().unwrap();
    let action_queue = Arc::new(Mutex::new(VecDeque::new()));

    setup_buttons(
        pins.gpio4,
        pins.gpio5,
        pins.gpio6,
        pins.gpio7,
        action_queue.clone(),
    ).unwrap();

    let mut game = TetrisGame::new();

    loop {
        // Versuche, die Warteschlange zu sperren und eine Aktion zu entnehmen.
        let action = action_queue.lock().unwrap().pop_front();

        // Wenn eine Aktion vorhanden war, wende sie an.
        match action {
            Some(_) => {
                // LED kurz aufleuchten lassen als Feedback
                led.set_high().unwrap();

                std::thread::sleep(Duration::from_millis(500)); // Kurze Verzögerung, damit das Leuchten sichtbar ist
                led.set_low().unwrap();
            }
            None => {}
        }
    }

    for i in 0.. {
        let display = game.step(i);
        println!("{}", display);
        std::thread::sleep(Duration::from_millis(500));
    }
}
