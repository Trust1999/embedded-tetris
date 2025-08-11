use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::spi::{SpiDeviceDriver, SpiDriver};
use std::time::Duration;

mod time;
use time::Time;

mod game;
use game::TetrisGame;

mod display;
use display::Max72xx;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let mut display = {
        // Initialize SPI2
        let spi_driver = SpiDriver::new(
            peripherals.spi2,
            peripherals.pins.gpio12,       // SCLK (FSPICLK)
            peripherals.pins.gpio11,       // MOSI (FSPID)
            Some(peripherals.pins.gpio13), // MISO (FSPIQ), not used
            &Default::default(),
        )?;

        // Chip Select pin for the cascaded MAX72xx devices
        let cs_pin = peripherals.pins.gpio10;
        let spi = SpiDeviceDriver::new(spi_driver, Some(cs_pin), &Default::default())?;

        Max72xx::new(spi, 4)
    };
    display.reset()?;

    let mut time = Time::setup(peripherals.timer00)?;
    time.start()?;

    let mut game = TetrisGame::new();
    for i in 0.. {
        time.update()?;

        game.step(i, &mut display);

        display.transfer_bitmap()?;
    }

    Ok(())
}
