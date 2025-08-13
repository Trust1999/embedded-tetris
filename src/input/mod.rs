use std::sync::{LazyLock, Mutex};
use std::time::{Duration, Instant};
use esp_idf_hal::gpio::{Gpio4, Gpio5, Gpio6, Gpio7, Input, PinDriver};
use heapless::spsc::Queue;
use crate::game::ButtonAction;

//queue to save button inputs
pub static ACTION_QUEUE: LazyLock<Mutex<Queue<ButtonAction, 32>>> = LazyLock::new(|| Mutex::new(Queue::new()));

//to save button
pub static BUTTON1: LazyLock<Mutex<Option<PinDriver<'static, Gpio4, Input>>>> = LazyLock::new(|| Mutex::new(None));
pub static BUTTON2: LazyLock<Mutex<Option<PinDriver<'static, Gpio5, Input>>>> = LazyLock::new(|| Mutex::new(None));
pub static BUTTON3: LazyLock<Mutex<Option<PinDriver<'static, Gpio6, Input>>>> = LazyLock::new(|| Mutex::new(None));
pub static BUTTON4: LazyLock<Mutex<Option<PinDriver<'static, Gpio7, Input>>>> = LazyLock::new(|| Mutex::new(None));

// Debounce-time, initial to 1 second into the past
pub static LAST_PRESS_1: LazyLock<Mutex<Instant>> = LazyLock::new(|| Mutex::new(Instant::now() - Duration::from_secs(1)));
pub static LAST_PRESS_2: LazyLock<Mutex<Instant>> = LazyLock::new(|| Mutex::new(Instant::now() - Duration::from_secs(1)));
pub static LAST_PRESS_3: LazyLock<Mutex<Instant>> = LazyLock::new(|| Mutex::new(Instant::now() - Duration::from_secs(1)));
pub static LAST_PRESS_4: LazyLock<Mutex<Instant>> = LazyLock::new(|| Mutex::new(Instant::now() - Duration::from_secs(1)));


// Debounce + Queue Push for Button 1 (MoveLeft)
pub fn gpio_04() {
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

// Debounce + Queue Push for Button 2 (MoveRight)
pub fn gpio_05() {
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

// Debounce + Queue Push for Button 3 (MoveDown)
pub fn gpio_06() {
    let now = Instant::now();
    let mut last_press = LAST_PRESS_3.lock().unwrap();

    if now.duration_since(*last_press) >= Duration::from_millis(200) {
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

// Debounce + Queue Push for Button 4 (Rotate)
pub fn gpio_07() {
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