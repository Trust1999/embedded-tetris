use std::sync::{Arc, Mutex};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::prelude::Peripherals;
use esp_idf_svc::http::server::{Configuration, EspHttpServer};
use esp_idf_svc::nvs::EspNvsPartition;
use esp_idf_svc::wifi::{AuthMethod, BlockingWifi, EspWifi};
use anyhow::Result;
use esp_idf_hal::io::Write;
use esp_idf_hal::modem::Modem;
use crate::highscore::Highscores;

// Configuration for the WLAN access point
const SSID: &str = "ESP32-Tetris";
const PASSWORD: &str = "tetris123";

// A structure that holds all necessary network services together.
// As long as an instance of this structure exists, the Wi-Fi and server remain active.
pub struct WifiServer<'a> {
    _wifi: BlockingWifi<EspWifi<'a>>,
    _server: EspHttpServer<'a>,
}

impl<'a> WifiServer<'a> {
    /// Creates and starts a Wi-Fi access point with an HTTP server.
    ///
    /// # Parameters
    /// - `modem`: The ESP32's Wi-Fi modem (taken from `Peripherals`)
    /// - `nvs`: Non-Volatile Storage Defeault Partition for storing Wi-Fi information
    /// - `highscores`: Thread-safe access to the highscore list
    pub fn new(
        modem: Modem,
        nvs: EspNvsPartition<esp_idf_svc::nvs::NvsDefault>,
        highscores: Arc<Mutex<Highscores>>,
    ) -> Result<Self> {
        // Get system event loop (only possible once per program)
        let sys_loop = EspSystemEventLoop::take()?;

        // Initialize WLAN driver (with modem and NVS)
        let mut wifi = BlockingWifi::wrap(
            EspWifi::new(modem, sys_loop.clone(), Some(nvs))?,
            sys_loop,
        )?;

        // Configure WLAN as an access point
        wifi.set_configuration(&esp_idf_svc::wifi::Configuration::AccessPoint(
            esp_idf_svc::wifi::AccessPointConfiguration {
                ssid: SSID.parse().unwrap(),
                password: PASSWORD.parse().unwrap(),
                auth_method: AuthMethod::WPA2Personal,
                ..Default::default()
            },
        ))?;

        // start wifi
        wifi.start()?;

        // Get the IP address of the access point
        let ip_info = wifi.wifi().ap_netif().get_ip_info()?;

        // Wait until the network interface is fully ready
        wifi.wait_netif_up()?;
        log::info!("Access Point '{}' ist aktiv. Verbinden Sie sich und öffnen Sie http://{}/", SSID, ip_info.ip);

        // Start HTTP server (default configuration)
        let mut server = EspHttpServer::new(&Configuration::default())?;

        // Register a handler for the main page
        server.fn_handler("/", esp_idf_svc::http::Method::Get, move |request| -> Result<(), Box<dyn std::error::Error>> {
            // Access to the high scores (thread-safe)
            let highscores_lock = highscores.lock().unwrap();
            // Generate HTML page
            let html_response = generate_html(&highscores_lock);
            // Write reply
            let mut response = request.into_ok_response()?;
            response.write_all(html_response.as_bytes())?;
            Ok(())
        })?;

        // Return the structure containing both the wifi driver and the server.
        Ok(Self {
            _wifi: wifi,
            _server: server,
        })
    }
}


/// Generates the HTML code for the highscore page.
///
/// If no highscores exist, a corresponding message is displayed.
fn generate_html(highscores: &Highscores) -> String {
    let mut body = String::new();

    // Insert high scores dynamically
    if highscores.scores.is_empty() {
        body.push_str("<p>Bisher keine Highscores aufgezeichnet.</p>");
    } else {
        body.push_str("<ol>");
        for (index, score) in highscores.scores.iter().enumerate() {
            body.push_str(&format!("<li>Platz: {} Punkte</li>", score));
        }
        body.push_str("</ol>");
    }

    // HTML page structure
    format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="utf-8">
            <title>ESP32 Tetris Highscores</title>
            <style>
                body {{ font-family: Arial, sans-serif; background-color: #282c34; color: #ffffff; display: flex; justify-content: center; align-items: center; height: 100vh; margin: 0; }}
                .container {{ background-color: #20232a; padding: 2rem; border-radius: 8px; box-shadow: 0 4px 8px rgba(0,0,0,0.2); text-align: center; }}
                h1 {{ color: #61dafb; }}
                ol {{ list-style-position: inside; padding: 0; }}
                li {{ background-color: #3c4049; margin: 0.5rem 0; padding: 0.5rem; border-radius: 4px; }}
            </style>
        </head>
        <body>
            <div class="container">
                <h1>Tetris Highscores</h1>
                {}
            </div>
        </body>
        </html>
        "#,
        body
    )
}