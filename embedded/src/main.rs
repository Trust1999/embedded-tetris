use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::spi::{SpiDeviceDriver, SpiDriver};
use esp_idf_svc::nvs::{EspNvs, EspNvsPartition, NvsDefault};
use game::display::Max72xx;
use game::logic::{GameState, InGameState};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub mod highscore;
use highscore::{NVS_NAMESPACE, load_highscores, save_highscores};

mod website;
use website::WifiServer;

mod input;
use input::{ACTION_QUEUE, gpio_04, gpio_05, gpio_06, gpio_07, setup_button};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    // NVS partition for WLAN configuration
    let partition = EspNvsPartition::<NvsDefault>::take().unwrap();
    let mut nvs = EspNvs::new(partition.clone(), NVS_NAMESPACE, true).unwrap();

    // Webserver initialization with score from memory
    let highscores = Arc::new(Mutex::new(load_highscores(&mut nvs)?));
    let _wifi_server = WifiServer::new(
        peripherals.modem,
        partition.clone(),
        Arc::clone(&highscores),
    )?;

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

    let mut button1 = setup_button(peripherals.pins.gpio4, gpio_04);
    let mut button2 = setup_button(peripherals.pins.gpio5, gpio_05);
    let mut button3 = setup_button(peripherals.pins.gpio6, gpio_06);
    let mut button4 = setup_button(peripherals.pins.gpio7, gpio_07);

    let mut game_state = GameState::InGame(InGameState::new());

    println!("{:?}", highscores);

    while highscores.try_lock().is_err() {}

    loop {
        button1.enable_interrupt()?;
        button2.enable_interrupt()?;
        button3.enable_interrupt()?;
        button4.enable_interrupt()?;

        let button_actions = ACTION_QUEUE.pop_iter().collect::<Vec<_>>();

        {
            let mut highscores = highscores.lock().unwrap();
            game_state = game_state.update(&button_actions, Instant::now(), |score| {
                highscores.add_score(score)
            });
            save_highscores(&mut nvs, &highscores)?;
        }

        game::logic::render(&game_state, &mut display);

        display.transfer_bitmap()?;
    }
}
