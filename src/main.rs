use embedded_hal::spi::SpiDevice;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::spi;
use std::time::Duration;

mod game;
use game::TetrisGame;

mod display;
use display::Max72xx;

fn setup() -> anyhow::Result<(Max72xx<impl SpiDevice<Error = spi::SpiError>>, ())> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    // Initialize SPI2
    let spi_driver = spi::SpiDriver::new(
        peripherals.spi2,
        peripherals.pins.gpio12,       // SCLK (FSPICLK)
        peripherals.pins.gpio11,       // MOSI (FSPID)
        Some(peripherals.pins.gpio13), // MISO (FSPIQ), not used
        &Default::default(),
    )?;

    // Chip Select pin for the cascaded MAX72xx devices
    let cs_pin = peripherals.pins.gpio10;
    let spi = spi::SpiDeviceDriver::new(spi_driver, Some(cs_pin), &Default::default())?;

    let mut max = Max72xx::new(spi, 4);
    max.reset()?;

    std::thread::sleep(Duration::from_millis(100));

    Ok((max, ()))
}

fn main() -> anyhow::Result<()> {
    let (mut display, _) = setup()?;
    let mut game = TetrisGame::new();

    for i in 0.. {
        game.step(i, &mut display);
        display.transfer_bitmap()?;
    }

    Ok(())
}
