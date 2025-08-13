use crate::game::ButtonAction;
use esp_idf_hal::gpio::{Gpio4, Gpio5, Gpio6, Gpio7, Input, PinDriver};
use lockfree::queue::Queue;
use std::sync::{LazyLock, Mutex};
use std::time::{Duration, Instant};

//queue to save button inputs
pub static ACTION_QUEUE: LazyLock<Queue<ButtonAction>> = LazyLock::new(|| Queue::new());

// Debounce-time, initial to 1 second into the past
pub static LAST_PRESS_1: LazyLock<Mutex<Instant>> =
    LazyLock::new(|| Mutex::new(Instant::now() - Duration::from_secs(1)));
pub static LAST_PRESS_2: LazyLock<Mutex<Instant>> =
    LazyLock::new(|| Mutex::new(Instant::now() - Duration::from_secs(1)));
pub static LAST_PRESS_3: LazyLock<Mutex<Instant>> =
    LazyLock::new(|| Mutex::new(Instant::now() - Duration::from_secs(1)));
pub static LAST_PRESS_4: LazyLock<Mutex<Instant>> =
    LazyLock::new(|| Mutex::new(Instant::now() - Duration::from_secs(1)));

// Debounce + Queue Push for Button 1 (MoveLeft)
pub fn gpio_04() {
    let now = Instant::now();
    let mut last_press = LAST_PRESS_1.lock().unwrap();

    if now.duration_since(*last_press) >= Duration::from_millis(200) {
        ACTION_QUEUE.push(ButtonAction::MoveLeft);
        *last_press = now;
    }
}

// Debounce + Queue Push for Button 2 (MoveRight)
pub fn gpio_05() {
    let now = Instant::now();
    let mut last_press = LAST_PRESS_2.lock().unwrap();

    if now.duration_since(*last_press) >= Duration::from_millis(200) {
        ACTION_QUEUE.push(ButtonAction::MoveRight);
        *last_press = now;
    }
}

// Debounce + Queue Push for Button 3 (MoveDown)
pub fn gpio_06() {
    let now = Instant::now();
    let mut last_press = LAST_PRESS_3.lock().unwrap();

    if now.duration_since(*last_press) >= Duration::from_millis(200) {
        ACTION_QUEUE.push(ButtonAction::MoveDown);
        *last_press = now;
    }
}

// Debounce + Queue Push for Button 4 (Rotate)
pub fn gpio_07() {
    let now = Instant::now();
    let mut last_press = LAST_PRESS_4.lock().unwrap();

    if now.duration_since(*last_press) >= Duration::from_millis(200) {
        ACTION_QUEUE.push(ButtonAction::Rotate);
        *last_press = now;
    }
}
