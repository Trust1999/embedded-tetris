use std::time::{Duration, Instant};

mod game;
use game::TetrisGame;

use esp_idf_hal::gpio::*;
use esp_idf_hal::gpio::{self, PinDriver, Pull};
use std::sync::Mutex;
use esp_idf_hal::peripherals::Peripherals;
use once_cell::sync::Lazy;
use heapless::spsc::Queue;

static ACTION_QUEUE: Lazy<Mutex<Queue<ButtonAction, 100>>> = Lazy::new(|| Mutex::new(Queue::new()));
static BUTTON1: Lazy<Mutex<Option<PinDriver<'static, Gpio4, Input>>>> = Lazy::new(|| Mutex::new(None));
static BUTTON2: Lazy<Mutex<Option<PinDriver<'static, Gpio5, Input>>>> = Lazy::new(|| Mutex::new(None));
static BUTTON3: Lazy<Mutex<Option<PinDriver<'static, Gpio6, Input>>>> = Lazy::new(|| Mutex::new(None));
static BUTTON4: Lazy<Mutex<Option<PinDriver<'static, Gpio7, Input>>>> = Lazy::new(|| Mutex::new(None));

// Debounce-Zeitpunkte, initial auf 1 Sekunde in der Vergangenheit gesetzt
static LAST_PRESS_1: Lazy<Mutex<Instant>> = Lazy::new(|| Mutex::new(Instant::now() - Duration::from_secs(1)));
static LAST_PRESS_2: Lazy<Mutex<Instant>> = Lazy::new(|| Mutex::new(Instant::now() - Duration::from_secs(1)));
static LAST_PRESS_3: Lazy<Mutex<Instant>> = Lazy::new(|| Mutex::new(Instant::now() - Duration::from_secs(1)));
static LAST_PRESS_4: Lazy<Mutex<Instant>> = Lazy::new(|| Mutex::new(Instant::now() - Duration::from_secs(1)));

#[derive(Debug, Clone, Copy)]
enum ButtonAction {
    MoveLeft,
    MoveRight,
    MoveDown,
    Rotate,
}

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    let mut button1 = PinDriver::input(pins.gpio4).unwrap();
    button1.set_pull(Pull::Up).unwrap();
    button1.set_interrupt_type(gpio::InterruptType::PosEdge).unwrap();
    unsafe { button1.subscribe(gipo_04).unwrap(); }
    button1.enable_interrupt().unwrap();
    *BUTTON1.lock().unwrap() = Some(button1);

    let mut button2 = PinDriver::input(pins.gpio5).unwrap();
    button2.set_pull(Pull::Up).unwrap();
    button2.set_interrupt_type(gpio::InterruptType::PosEdge).unwrap();
    unsafe { button2.subscribe(gipo_05).unwrap(); }
    button2.enable_interrupt().unwrap();
    *BUTTON2.lock().unwrap() = Some(button2);

    let mut button3 = PinDriver::input(pins.gpio6).unwrap();
    button3.set_pull(Pull::Up).unwrap();
    button3.set_interrupt_type(gpio::InterruptType::PosEdge).unwrap();
    unsafe { button3.subscribe(gipo_06).unwrap(); }
    button3.enable_interrupt().unwrap();
    *BUTTON3.lock().unwrap() = Some(button3);

    let mut button4 = PinDriver::input(pins.gpio7).unwrap();
    button4.set_pull(Pull::Up).unwrap();
    button4.set_interrupt_type(gpio::InterruptType::PosEdge).unwrap();
    unsafe { button4.subscribe(gipo_07).unwrap(); }
    button4.enable_interrupt().unwrap();
    *BUTTON4.lock().unwrap() = Some(button4);

    let mut game = TetrisGame::new();

    loop {
        let action_opt = {
            let mut queue = ACTION_QUEUE.lock().unwrap();
            queue.dequeue()
        };

        if let Some(action) = action_opt {
            println!("Button gedrückt: {:?}", action);
            std::thread::sleep(Duration::from_millis(500));
        }

        std::thread::sleep(Duration::from_millis(10));
    }

    // Alternativ: Game-Loop, der nicht erreicht wird, da oben eine Endlosschleife ist
    /*
    for i in 0.. {
        let display = game.step(i);
        println!("{}", display);
        std::thread::sleep(Duration::from_millis(500));
    }
    */
}

// Debounce + Queue Push für Button 1 (MoveLeft)
fn gipo_04() {
    let now = Instant::now();
    let mut last_press = LAST_PRESS_1.lock().unwrap();

    if now.duration_since(*last_press) >= Duration::from_millis(100) {
        if let Ok(mut queue) = ACTION_QUEUE.lock() {
            let _ = queue.enqueue(ButtonAction::MoveLeft);
        }
        *last_press = now;
    }

    if let Ok(mut maybe_button) = BUTTON1.lock() {
        if let Some(button) = maybe_button.as_mut() {
            let _ = button.enable_interrupt();
        }
    }
}

// Debounce + Queue Push für Button 2 (MoveRight)
fn gipo_05() {
    let now = Instant::now();
    let mut last_press = LAST_PRESS_2.lock().unwrap();

    if now.duration_since(*last_press) >= Duration::from_millis(100) {
        if let Ok(mut queue) = ACTION_QUEUE.lock() {
            let _ = queue.enqueue(ButtonAction::MoveRight);
        }
        *last_press = now;
        }
    if let Ok(mut maybe_button) = BUTTON2.lock() {
        if let Some(button) = maybe_button.as_mut() {
            let _ = button.enable_interrupt();
        }
    }
}

// Debounce + Queue Push für Button 3 (MoveDown)
fn gipo_06() {
    let now = Instant::now();
    let mut last_press = LAST_PRESS_3.lock().unwrap();

    if now.duration_since(*last_press) >= Duration::from_millis(100) {
        if let Ok(mut queue) = ACTION_QUEUE.lock() {
            let _ = queue.enqueue(ButtonAction::MoveDown);
        }
        *last_press = now;
        }
    if let Ok(mut maybe_button) = BUTTON3.lock() {
        if let Some(button) = maybe_button.as_mut() {
            let _ = button.enable_interrupt();
        }
    }
}

// Debounce + Queue Push für Button 4 (Rotate)
fn gipo_07() {
    let now = Instant::now();
    let mut last_press = LAST_PRESS_4.lock().unwrap();

    if now.duration_since(*last_press) >= Duration::from_millis(100) {
        if let Ok(mut queue) = ACTION_QUEUE.lock() {
            let _ = queue.enqueue(ButtonAction::Rotate);
        }
        *last_press = now;
        }
    if let Ok(mut maybe_button) = BUTTON4.lock() {
        if let Some(button) = maybe_button.as_mut() {
            let _ = button.enable_interrupt();
        }
    }
}
