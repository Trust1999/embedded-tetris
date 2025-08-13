use esp_idf_hal::gpio::*;
use esp_idf_hal::gpio::{self, PinDriver, Pull};
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
use crate::input::{
    ACTION_QUEUE, BUTTON1, BUTTON2, BUTTON3, BUTTON4, gpio_04, gpio_05, gpio_06, gpio_07,
};

const DISPLAY_WIDTH: u8 = 8;
const DISPLAY_HEIGHT: u8 = 8 * 4;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    // NVS-Partition für die WLAN-Konfiguration
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

    // Create a new PinDriver for GPIO4 configured as an input pin
    let mut button1 = PinDriver::input(peripherals.pins.gpio4).unwrap();
    // Enable an internal pull-up resistor on GPIO4
    button1.set_pull(Pull::Up).unwrap();
    // Set the interrupt to trigger on a positive edge (low → high transition)
    button1
        .set_interrupt_type(gpio::InterruptType::PosEdge)
        .unwrap();
    // Subscribe the GPIO4 interrupt to call the function `gipo_04` when triggered
    // `unsafe` is needed because we are passing a raw function pointer
    unsafe {
        button1.subscribe(gpio_04).unwrap();
    }
    // Enable interrupts for this pin
    button1.enable_interrupt().unwrap();
    *BUTTON1.lock().unwrap() = Some(button1);

    // Create a new PinDriver for GPIO5 configured as an input pin
    let mut button2 = PinDriver::input(peripherals.pins.gpio5).unwrap();
    // Enable an internal pull-up resistor on GPIO5
    button2.set_pull(Pull::Up).unwrap();
    // Set the interrupt to trigger on a positive edge (low → high transition)
    button2
        .set_interrupt_type(gpio::InterruptType::PosEdge)
        .unwrap();
    // Subscribe the GPIO4 interrupt to call the function `gipo_05` when triggered
    // `unsafe` is needed because we are passing a raw function pointer
    unsafe {
        button2.subscribe(gpio_05).unwrap();
    }
    // Enable interrupts for this pin
    button2.enable_interrupt().unwrap();
    *BUTTON2.lock().unwrap() = Some(button2);

    // Create a new PinDriver for GPIO6 configured as an input pin
    let mut button3 = PinDriver::input(peripherals.pins.gpio6).unwrap();
    // Enable an internal pull-up resistor on GPIO6
    button3.set_pull(Pull::Up).unwrap();
    // Set the interrupt to trigger on a positive edge (low → high transition)
    button3
        .set_interrupt_type(gpio::InterruptType::PosEdge)
        .unwrap();
    // Subscribe the GPIO4 interrupt to call the function `gipo_06` when triggered
    // `unsafe` is needed because we are passing a raw function pointer
    unsafe {
        button3.subscribe(gpio_06).unwrap();
    }
    // Enable interrupts for this pin
    button3.enable_interrupt().unwrap();
    *BUTTON3.lock().unwrap() = Some(button3);

    // Create a new PinDriver for GPIO7 configured as an input pin
    let mut button4 = PinDriver::input(peripherals.pins.gpio7).unwrap();
    // Enable an internal pull-up resistor on GPIO7
    button4.set_pull(Pull::Up).unwrap();
    // Set the interrupt to trigger on a positive edge (low → high transition)
    button4
        .set_interrupt_type(gpio::InterruptType::PosEdge)
        .unwrap();
    // Subscribe the GPIO4 interrupt to call the function `gipo_07` when triggered
    // `unsafe` is needed because we are passing a raw function pointer
    unsafe {
        button4.subscribe(gpio_07).unwrap();
    }
    // Enable interrupts for this pin
    button4.enable_interrupt().unwrap();
    *BUTTON4.lock().unwrap() = Some(button4);

    let mut time = Time::setup(peripherals.timer00)?;
    time.start()?;

    let mut game_state = GameState::InGame(InGameState::new());

    println!("{:?}", highscores);

    loop {
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
