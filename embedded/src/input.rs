use esp_idf_hal::gpio;
use esp_idf_hal::gpio::{Input, InputPin, OutputPin, Pin, PinDriver, Pull};
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_sys::EspError;
use std::sync::atomic::{AtomicBool, Ordering};

pub static BUTTON_LEFT: AtomicBool = AtomicBool::new(false);
pub static BUTTON_RIGHT: AtomicBool = AtomicBool::new(false);
pub static BUTTON_DOWN: AtomicBool = AtomicBool::new(false);
pub static BUTTON_ROTATE: AtomicBool = AtomicBool::new(false);

pub fn setup_button<'d>(
    pin: impl Peripheral<P = impl InputPin + OutputPin> + 'd,
    callback: impl FnMut() + Send + 'static,
) -> Result<PinDriver<'d, impl Pin, Input>, EspError> {
    // Create a new PinDriver for GPIO4 configured as an input pin
    let mut driver = PinDriver::input(pin)?;
    // Enable an internal pull-up resistor on GPIO4
    driver.set_pull(Pull::Up)?;
    // Set the interrupt to trigger on a positive edge (low → high transition)
    driver.set_interrupt_type(gpio::InterruptType::NegEdge)?;
    // Subscribe the GPIO4 interrupt to call the function `gipo_04` when triggered
    // `unsafe` is needed because we are passing a raw function pointer
    unsafe { driver.subscribe(callback)? };
    // Enable interrupts for this pin
    driver.enable_interrupt()?;

    Ok(driver)
}

/// Queue Push for Button 1 (MoveLeft)
pub fn gpio_04() {
    BUTTON_LEFT.store(true, Ordering::SeqCst);
}

/// Queue Push for Button 2 (MoveRight)
pub fn gpio_05() {
    BUTTON_RIGHT.store(true, Ordering::SeqCst);
}

/// Queue Push for Button 3 (MoveDown)
pub fn gpio_06() {
    BUTTON_DOWN.store(true, Ordering::SeqCst);
}

/// Queue Push for Button 4 (Rotate)
pub fn gpio_07() {
    BUTTON_ROTATE.store(true, Ordering::SeqCst);
}
