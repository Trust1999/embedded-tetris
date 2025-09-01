# Embedded Tetris

### Autoren

Nika Sommer

Anton Unger

### Datum

13.08.2025

## Einleitung

### Motivation und Problemstellung

Die zentrale Motivation dieses Projekts war die praktische Anwendung von im Informatikstudium erworbenem theoretischem
Wissen. Die Umsetzung eines bekannten Spielklassikers wie Tetris auf ressourcenbeschränkter Embedded-Hardware bot
hierfür den idealen Rahmen. Das Projekt stellte uns vor die Herausforderung, eine vollständige Applikation von der
hardwarenahen Ansteuerung bis zur Weboberfläche zu entwickeln. Eine besondere Anforderung war die Erweiterung des
klassischen Spielprinzips: Anstelle fester seitlicher Begrenzungen sollten die Spielblöcke das Spielfeld an einem Rand
verlassen und auf der gegenüberliegenden Seite wieder eintreten können (Screen-Wrapping).

Das Endprodukt ist ein auf einem ESP32-S3-Mikrocontroller in Rust implementiertes Tetris-Spiel. Die Darstellung des 8x32
Pixel großen Spielfelds erfolgt auf einem kaskadierten MAX7219 LED-Dot-Matrix-Display. Die Steuerung wird über vier
Taster realisiert, und die zehn besten Spielergebnisse werden persistent gespeichert und über eine auf dem ESP32
gehostete Weboberfläche zugänglich gemacht.

### Ziele des Projektes

Das Hauptziel war die Entwicklung eines vollständig funktions- und spielfähigen Tetris-Klons auf der ausgewählten
Embedded-Plattform. Dies umfasste die korrekte Implementierung aller wesentlichen Kernfunktionen:

* Spielmechanik: Blockbewegungen (links, rechts, unten), Rotation, automatisches Herabfallen der Blöcke, Erkennung und
  Auflösung vollständiger horizontaler Linien sowie eine funktionierende Punktevergabe.
* Anzeige und Steuerung: Eine flüssige Echtzeit-Darstellung des Spielgeschehens auf dem LED-Display sowie eine
  reaktionsschnelle Verarbeitung der Taster-Eingaben.
* Highscore-System: Persistente Speicherung der besten Punktzahlen, die auch nach einem Neustart des Geräts erhalten
  bleiben.
* Web-Interface: Bereitstellung einer einfachen Weboberfläche über einen WLAN-Access-Point, um die Highscore-Liste
  abzurufen.

Ein sekundäres, persönliches Ziel war die Vertiefung unserer Kenntnisse in der hardwarenahen Programmierung mit Rust,
der Arbeit mit Embedded-Systemen und deren Peripherie sowie der Integration von Netzwerkfunktionalitäten wie einem
Webserver auf einem Mikrocontroller.

### Relevanz und Anwendungsgebiet

Das Projekt verbindet die hardwarenahe Programmierung, Embedded Systems und Webentwicklung in einer
greifbaren Anwendung. Die Implementierung zeigt exemplarisch, wie moderne, sichere Programmiersprachen wie Rust für
komplexe Aufgaben auf Mikrocontrollern eingesetzt werden können.

Potenzielle Anwendungsgebiete und Relevanz des Projekts sind:

* Lehr- und Lernprojekte: Als anschauliches Beispiel im Bereich Embedded Systems, um die Realisierung von
  Echtzeitanwendungen auf limitierter Hardware zu demonstrieren.
* Demonstrationen und Prototyping: Der hohe Wiedererkennungswert von Tetris eignet sich hervorragend für Präsentationen
  auf Messen oder für das Prototyping von interaktiven Displays mit einfacher Benutzerinteraktion.
* Praxiserfahrung mit Rust: Das Projekt liefert wertvolle Einblicke in die Stärken und Herausforderungen von Rust im
  Embedded-Bereich, einem noch wachsenden Ökosystem.

## Technischer Hintergrund

### Relevante Technologien

Das Projekt stützt sich auf die Programmiersprache Rust, die aufgrund ihrer Speichersicherheit und
Performance-Eigenschaften eine exzellente Wahl für die Entwicklung robuster Embedded-Systeme ist.

Zentrale technische Aspekte der Umsetzung sind:

* Interrupt-gesteuerte Eingabeverarbeitung: Die vier Taster sind über GPIOs angebunden und lösen bei Betätigung
  Interrupts aus. Dies ermöglicht eine reaktionsschnelle und ressourcenschonende Erfassung von Benutzereingaben, ohne
  dass die CPU ständig die Pin-Zustände abfragen muss (Polling).
* LED-Matrix-Ansteuerung via SPI: Die Kommunikation mit dem MAX7219-Treiberchip des Displays erfolgt über das Serial
  Peripheral Interface (SPI). Dieses serielle Protokoll erlaubt eine effiziente und schnelle Übertragung der Bilddaten
  an die kaskadierten 8x8-Module.
* Persistente Datenspeicherung (NVS): Highscores werden im Non-Volatile Storage (NVS) des ESP32 abgelegt. Dieser
  Flash-Speicherbereich behält seine Daten auch ohne Stromversorgung, wodurch die Spielstände dauerhaft gesichert sind.
* Integrierter Webserver: Der ESP32 agiert als WLAN-Access-Point und hostet einen HTTP-Server. Dies ermöglicht es, die
  Highscore-Liste plattformunabhängig über einen Webbrowser auf jedem WLAN-fähigen Gerät abzurufen.

### Verwendete Frameworks, Hardware, Protokolle

Frameworks & Bibliotheken:

* esp-idf-hal: Eine Hardware Abstraction Layer (HAL) für Rust, die eine typsichere und high-level Ansteuerung der
  ESP32-Peripherie wie GPIOs und SPI ermöglicht.
* esp-idf-svc: Stellt System-Services wie Logging, Wi-Fi-Management und die NVS-Anbindung für das esp-idf-Ökosystem
  bereit.
* std::sync::atomic: Zur thread-sicheren Kommunikation zwischen den Interrupt-Routinen (welche die Taster-Eingaben
  registrieren) und dem Haupt-Thread der Spiellogik.

Hardware:

* ESP32-S3-DevKitC-1: Ein leistungsstarker Dual-Core-Mikrocontroller mit integriertem Wi-Fi, der umfassende
  Unterstützung für das Rust-Ökosystem bietet.
* BerryBase MAX7219 4-in-1 LED-Dot-Matrix-Display: Ein Anzeigemodul bestehend aus vier kaskadierten 8x8-LED-Matrizen,
  was eine Gesamtauflösung von 32x8 Pixeln ergibt.
* Vier Taster: Einfache mechanische Schalter für die Spielsteuerung (Links, Rechts, Runter, Drehen).

Protokolle & Schnittstellen:

* SPI (Serial Peripheral Interface): Wird für die High-Speed-Datenübertragung zwischen dem ESP32-S3 und der LED-Matrix
  genutzt.
* GPIO (General Purpose Input/Output): Dient zur Abfrage der Tasterzustände (Input) und zur Ansteuerung der
  Chip-Select-Leitung des Displays (Output).
* Wi-Fi (IEEE 802.11): Ermöglicht den Betrieb des ESP32 als Access Point für die Bereitstellung des Webservers.

## Projektidee und Anforderung

### Kurze Beschreibung des Konzepts

Das Projekt realisiert das Spiel Tetris auf einem autarken Embedded-System. Ein ESP32-S3 Mikrocontroller steuert die
gesamte Spiellogik, verarbeitet Benutzereingaben und stellt das Spielfeld auf einem 32x8 Pixel LED-Matrix-Display dar.
Spieler steuern die fallenden Blöcke mit vier Tastern. Eine Besonderheit ist das "Screen-Wrapping", bei dem Blöcke, die
seitlich aus dem Spielfeld bewegt werden, auf der gegenüberliegenden Seite wieder erscheinen. Die erreichten Punktzahlen
werden persistent gespeichert und können über eine vom ESP32 bereitgestellte Webseite eingesehen werden, indem man sich
mit dessen WLAN-Netzwerk verbindet.

### Zielgruppe/User

Die Zielgruppe des Projekts ist breit gefächert und umfasst:

* Hobby-Elektroniker und Maker: Personen mit Interesse an der Verknüpfung von Hardware und Software in praktischen
  Projekten.
* Software-Entwickler: Insbesondere jene, die ihre Kenntnisse in Rust und im Bereich der Embedded-Programmierung
  erweitern möchten.
* Studierende: Als praktisches Projekt zur Anwendung von Konzepten aus den Bereichen Rechnerarchitektur, Echtzeitsysteme
  und Softwareentwicklung.
* Retro-Gaming-Fans: Enthusiasten, die Freude daran haben, klassische Spiele auf unkonventioneller, selbstgebauter
  Hardware zu erleben.

### Funktionale und Nicht-Funktionale Anforderungen

Funktionale Anforderungen:

* Spielsteuerung: Das System muss die Steuerung der Blöcke (Bewegung nach links/rechts, Beschleunigung nach unten,
  Rotation) über vier dedizierte Taster ermöglichen.
* Spielmechanik: Das Spiel muss fallende Blöcke generieren, Kollisionen erkennen, volle Linien auflösen, den Punktestand
  zählen und ein "Game Over"-Szenario erkennen, wenn das Spielfeld voll ist.
* Anzeige: Das Spielfeld, die fallenden Blöcke und der aktuelle Zustand müssen in Echtzeit auf dem LED-Display
  visualisiert werden.
* Highscore: Das System muss eine Liste der Top-10-Punktestände führen, diese nach einem Spiel aktualisieren und
  persistent speichern.
* Web-Interface: Das Gerät muss einen WLAN-Access-Point aufspannen und eine Webseite mit der Highscore-Liste ausliefern
  können.

Nicht-Funktionale Anforderungen:

* Zuverlässigkeit: Die Anwendung muss stabil und ohne Abstürze laufen.
* Reaktionszeit: Taster-Eingaben müssen ohne spürbare Verzögerung (< 150 ms) verarbeitet und auf dem Display
  widergespiegelt werden.
* Benutzerfreundlichkeit: Die Steuerung muss einfach und intuitiv verständlich sein.
* Wartbarkeit: Der Rust-Code soll modular und klar strukturiert sein, um zukünftige Erweiterungen oder Fehlerbehebungen
  zu vereinfachen.

### Erste Skizzen und Diagramme

#### Schaltplan

![Schaltplan](../assets/Schaltplan.png)

#### Klassendiagramm

## Architektur und Umsetzung

### Übersicht der Systemarchitektur

Die Systemarchitektur ist in mehrere logische Schichten unterteilt, um eine klare Trennung der Verantwortlichkeiten (
Separation of Concerns) zu gewährleisten und die Modularität zu erhöhen. Dies erleichtert die Wartung und Testbarkeit
des Systems.

Die Architektur lässt sich wie folgt gliedern:

1. Hardware Abstraction Layer (HAL): Die unterste Ebene wird durch die esp-idf-hal-Bibliothek gebildet. Sie abstrahiert
   den direkten Zugriff auf die Peripherie des ESP32-S3 (GPIO, SPI) und stellt sichere Rust-Schnittstellen bereit.
2. Treiber-Schicht: Auf der HAL aufbauend befindet sich der hardwarespezifische Treiber für die LED-Matrix (display::
   Max72xx). Dieser Treiber ist für die direkte Kommunikation mit den MAX7219-Chips über das SPI-Protokoll
   verantwortlich.
3. Abstraktions-Schicht (Display Trait): Eine zentrale Komponente der Architektur ist der display::Display-Trait. Er
   definiert eine generische Schnittstelle für ein anzeigbares Medium mit den Methoden set_pixel und fill. Diese
   Abstraktion entkoppelt die Spiellogik vollständig von der konkreten Hardware. Für Test- und Debugging-Zwecke
   existiert mit TextDisplay eine zweite Implementierung dieses Traits, die das Spielfeld in der Konsole ausgibt.
4. Rendering-Schicht: Das render-Modul ist dafür zuständig, den aktuellen Spielzustand (GameState) in sichtbare Pixel zu
   übersetzen. Es nutzt den Display-Trait, um die entsprechenden Pixel zu setzen, ohne wissen zu müssen, ob die Ausgabe
   auf einer LED-Matrix oder in der Konsole erfolgt.
5. Logik-Schicht: Das logic-Modul enthält die komplette Spiellogik, inklusive der Zustandsautomaten (GameState), der
   Blockdefinitionen (Piece) und der Spielregeln. Diese Schicht ist vollständig unabhängig von jeglicher Ein- und
   Ausgabe.
6. Applikations-Schicht: Die main.rs-Datei bildet die oberste Schicht. Sie initialisiert alle Hardwarekomponenten und
   Bibliotheken, verarbeitet in einer Endlosschleife die Benutzereingaben, aktualisiert den Zustand der Spiellogik und
   stößt den Rendering-Vorgang an.

### Modulaufbau / Komponenten

Das Projekt ist in klar definierte Rust-Module unterteilt, die jeweils eine spezifische Aufgabe erfüllen:

* main: Der Einstiegspunkt der Anwendung. Verantwortlich für die Initialisierung des Systems (Peripherie, NVS, Wi-Fi)
  und die Ausführung der zentralen Spielschleife (main loop).
* input: Definiert die Logik zur Konfiguration der GPIO-Pins als Taster-Eingänge mit Interrupts. Stellt atomare Flags (
  AtomicBool) für die thread-sichere Kommunikation zwischen Interrupt-Service-Routinen und der Hauptschleife bereit.
* logic: Beinhaltet die gesamte Spielmechanik. Der GameState-Enum (StartMenu, InGame, GameOver) dient als
  Zustandsautomat, der den Spielfluss steuert. Hier sind auch die Definitionen der Tetris-Steine (Piece) und die
  Verwaltung des Spielfelds (Blocks) angesiedelt.
* display: Dieses Modul kapselt alles, was mit der Anzeige zu tun hat. Es definiert den zentralen Display-Trait, den
  Max72xx-Treiber für die LED-Matrix und den TextDisplay-Treiber für die Konsolenausgabe.
* render: Enthält die Funktionen, um die verschiedenen Spielzustände auf einem Display-Objekt darzustellen. Hier sind
  Bitmaps für Buchstaben und Zahlen als const fn hinterlegt, um sie effizient im Flash-Speicher zu lagern.
* highscore: Kümmert sich um das Serialisieren und Deserialisieren der Highscore-Daten sowie um deren persistente
  Speicherung im Non-Volatile Storage (NVS).
* website: Implementiert den Wi-Fi Access Point und den HTTP-Webserver zur Anzeige der Highscore-Liste.

### Wichtige Schnittstellen

* display::Display Trait: Die wichtigste interne Schnittstelle. Sie entkoppelt die Rendering-Logik von der
  Display-Hardware und ermöglicht einfaches Mocking und Testen.
* SPI (Serial Peripheral Interface): Die Hardware-Schnittstelle zwischen dem ESP32 und dem MAX7219-Displaytreiber zur
  Übertragung von Bilddaten und Konfigurationsbefehlen.
* GPIO-Interrupts: Die Schnittstelle zwischen den physischen Tastern und der Software. Ein Tastendruck löst einen
  Interrupt aus, der von der input-Logik verarbeitet wird.
* HTTP-Server: Die externe Schnittstelle, die es Benutzern ermöglicht, über einen Webbrowser auf die Highscore-Daten
  zuzugreifen.

### Begründung von Entscheidungen

Wahl von Rust: Rust wurde aufgrund seiner Garantien für Speichersicherheit und seiner Fähigkeit, performanten,
hardwarenahen Code ohne Garbage Collector zu erzeugen, gewählt. Das starke Typsystem und das Ökosystem (embedded-hal)
sind ideal für robuste Embedded-Anwendungen.

Abstraktion durch Display-Trait: Die Entscheidung, einen generischen Display-Trait zu schaffen, war architektonisch
zentral. Sie ermöglichte die Entwicklung und das Testen der gesamten Rendering- und Spiellogik auf einem PC (über
TextDisplay), bevor die Hardware-Implementierung vollständig abgeschlossen war.

Interrupt-basierte Eingabe: Anstelle von Polling wurde ein Interrupt-basierter Ansatz für die Taster gewählt. Dies ist
deutlich effizienter, da die CPU nicht ständig den Zustand der Pins abfragen muss und sich in der Zwischenzeit anderen
Aufgaben widmen kann.

## Implementierung

### Beschreibung zentraler Programmteil

### Eingesetzte Tools und Sprachen

Als Programmiersprache wurde Rust verwendet

## Tests und Ergebnisse

### Was getestet

### Ergebnisse

## Fazit und Ausblick

### Was lief gut, was war schwierig?

### Erfüllung der Ziele

### Lessons Learnd

### Ideen für Weiterentwicklung

## Resportery-Überblick

### Aufbau des Repo

### Setup Anleitung

### Beispiel zur Nutzung

## Lizens und Danksagungen
