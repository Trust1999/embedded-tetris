use esp_idf_hal::gpio;
use esp_idf_hal::gpio::{Input, InputPin, OutputPin, Pin, PinDriver, Pull};
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::sys::esp_timer_get_time;
use game::logic::ButtonAction;
use lockfree::queue::Queue;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::LazyLock;

//queue to save button inputs
pub static ACTION_QUEUE: LazyLock<Queue<ButtonAction>> = LazyLock::new(|| Queue::new());
const DEBOUNCE_US: u32 = 100_000; //100 ms
static LAST_ACTION_TIME: AtomicU32 = AtomicU32::new(0);

pub fn setup_button<'d>(
    pin: impl Peripheral<P = impl InputPin + OutputPin> + 'd,
    callback: impl FnMut() -> () + Send + 'static,
) -> PinDriver<'d, impl Pin, Input> {
    // Create a new PinDriver for GPIO4 configured as an input pin
    let mut driver = PinDriver::input(pin).unwrap();
    // Enable an internal pull-up resistor on GPIO4
    driver.set_pull(Pull::Up).unwrap();
    // Set the interrupt to trigger on a positive edge (low → high transition)
    driver
        .set_interrupt_type(gpio::InterruptType::NegEdge)
        .unwrap();
    // Subscribe the GPIO4 interrupt to call the function `gipo_04` when triggered
    // `unsafe` is needed because we are passing a raw function pointer
    unsafe { driver.subscribe(callback).unwrap() };
    // Enable interrupts for this pin
    driver.enable_interrupt().unwrap();

    driver
}

fn debounce_and_queue(action: ButtonAction) {
    // `esp_timer_get_time()` returns the time since boot in microseconds.
    // This is very fast and safe to use in an interrupt.
    let current_time = unsafe { esp_timer_get_time() as u32 };
    // Loads the last time point atomically (without lock).
    let last_time = LAST_ACTION_TIME.load(Ordering::Relaxed);

    // Checks if enough time has passed.
    if current_time - last_time > DEBOUNCE_US {
        // Attempts to update the `LAST_ACTION_TIME` with the current time.
        // `compare_exchange` is an atomic operation that ensures that the value
        // is only updated if it hasn't changed since the last time it was read (`last_time`).
        // This prevents race conditions if two keys are pressed almost simultaneously.
        if LAST_ACTION_TIME
            .compare_exchange(
                last_time,
                current_time,
                Ordering::Relaxed,
                Ordering::Relaxed,
            )
            .is_ok()
        {
            // Only if the time has been updated successfully will the action be queued.
            ACTION_QUEUE.push(action);
        }
    }
}

// Debounce + Queue Push for Button 1 (MoveLeft)
pub fn gpio_04() {
    debounce_and_queue(ButtonAction::MoveLeft);
}

// Debounce + Queue Push for Button 2 (MoveRight)
pub fn gpio_05() {
    debounce_and_queue(ButtonAction::MoveRight);
}

// Debounce + Queue Push for Button 3 (MoveDown)
pub fn gpio_06() {
    debounce_and_queue(ButtonAction::MoveDown);
}

// Debounce + Queue Push for Button 4 (Rotate)
pub fn gpio_07() {
    debounce_and_queue(ButtonAction::Rotate);
}
