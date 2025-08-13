use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::spi::{SpiDeviceDriver, SpiDriver};
use esp_idf_svc::nvs::{EspNvs, EspNvsPartition, NvsDefault};
use std::sync::{Arc, Mutex};

mod time;
use time::Time;

mod game;
use game::{GameState, InGameState, render};

mod display;
use display::Max72xx;

mod highscore;
use highscore::{Highscores, NVS_NAMESPACE, load_highscores, save_highscores};

mod website;
use website::WifiServer;

mod input;
use crate::input::{ACTION_QUEUE, gpio_04, gpio_05, gpio_06, gpio_07, setup_button};

const DISPLAY_WIDTH: u8 = 8;
const DISPLAY_HEIGHT: u8 = 8 * 4;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    // NVS partition for WLAN configuration
    let partition = EspNvsPartition::<NvsDefault>::take().unwrap();
    let mut nvs = EspNvs::new(partition.clone(), NVS_NAMESPACE, true).unwrap();

    //Webserver initialization with score from memory
    let highscores = Arc::new(Mutex::new(load_highscores(&mut nvs).unwrap()));
    let server_highscores = Arc::clone(&highscores);
    let _wifi_server =
        WifiServer::new(peripherals.modem, partition.clone(), server_highscores).unwrap();

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

    let mut time = Time::setup(peripherals.timer00)?;
    time.start()?;

    let mut game_state = GameState::InGame(InGameState::new());

    println!("{:?}", highscores);

    loop {
        button1.enable_interrupt()?;
        button2.enable_interrupt()?;
        button3.enable_interrupt()?;
        button4.enable_interrupt()?;

        time.update()?;

        let button_actions = ACTION_QUEUE.pop_iter().collect::<Vec<_>>();

        /*
        to save a highscore
        let mut highscores_lock = highscores.lock().unwrap();
        highscores_lock.add_score(score);
        save_highscores(&mut nvs, &highscores_lock)?;
        */

        game_state = game_state.update(&button_actions, &time);

        render(&game_state, &mut display);

        display.transfer_bitmap()?;
    }
}
