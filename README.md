1. License
2. Test
3. readme fertigstellen
4. PowerPoint
5. Dokumentation

# Embedded Tetris 🎮

Ein Tetris-Klon für den **ESP32-S3**, programmiert in **Rust**.  
Das Spiel läuft auf einer **8x32 LED-Matrix (MAX7219)** und wird über **vier Taster** gesteuert.  
Highscores werden im Flash gespeichert und können über einen integrierten **Webserver** im WLAN abgerufen werden.

---

## Features

- Klassisches Tetris mit **Screen-Wrapping** (Blöcke verlassen das Spielfeld und erscheinen auf der anderen Seite).
- **Interrupt-basierte Eingabeverarbeitung** für schnelle und zuverlässige Steuerung.
- Anzeige auf **LED-Matrix (32x8 Pixel)** via SPI.
- **Persistente Highscore-Speicherung** im Non-Volatile Storage (NVS).
- Integrierter **WLAN-Access-Point & Webserver** zur Highscore-Anzeige.

---

## Hardware

- ESP32-S3-DevKitC-1
- MAX7219 LED-Matrix (4 × 8x8 Module → 32x8 Pixel)
- 4 Taster für Eingabe

---

## Setup

1. Repository klonen
2. rustup intsallieren
3. ESP-IDF-Toolchain einrichten (Linux/macOS empfohlen)
4. Projekt bauen und flashen
5. cargo run

## Nutzung

* ESP32 startet nach Flashen automatisch das Spiel.
* Mit WLAN ESP32-Tetris (Passwort: tetris123) verbinden.
* Im Browser http://192.168.4.1/ öffnen → Highscores ansehen.
* Steuerung: Links, Rechts, Runter, Drehen über angeschlossene Taster.
