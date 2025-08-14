use crate::display::Display;
use crate::logic::piece::Piece;
use crate::logic::{GameState, InGameState, InStartState};
use crate::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

pub fn render(game_state: &mut GameState, display: &mut impl Display) {
    match game_state {
        GameState::StartMenu(state) => render_start(state, display),
        GameState::InGame(state) => render_in_game(state, display),
        GameState::GameOver(score) => render_score(*score, display),
    }
}

fn render_start(state: &mut InStartState, display: &mut impl Display) {
    match state {
        InStartState::Text => {
            for i in 0..4 {
                let offset = 8 * i as u8;
                render_letter(i, offset, display);
            }
            *state = InStartState::ButtonStart;
        }
        InStartState::ButtonStart => {
            for i in 0..4 {
                let offset = 8 * i as u8;
                render_button(false, offset, display);
            }
            *state = InStartState::ButtonPressed;
        }
        InStartState::ButtonPressed => {
            for i in 0..4 {
                let offset = 8 * i as u8;
                render_button(true, offset, display);
            }
            *state = InStartState::ButtonReleased;
        }
        InStartState::ButtonReleased => {
            for i in 0..4 {
                let offset = 8 * i as u8;
                render_button(false, offset, display);
            }
            *state = InStartState::Text;
        }
    }
}

fn render_letter(row: i32, offset: u8, display: &mut impl Display) {
    let bitmap = letter_bitmap(row);

    for (y, row) in bitmap.iter().enumerate() {
        for x in 0..8 {
            let mask = 1 << (7 - x); // leftmost pixel is the highest bit
            let pixel_on = (row & mask) != 0;
            if pixel_on {
                display.set_pixel(x, offset + y as u8, true);
            }
        }
    }
}

fn render_button(button: bool, offset: u8, display: &mut impl Display) {
    let bitmap = button_bitmap(button);

    for (y, row) in bitmap.iter().enumerate() {
        for x in 0..8 {
            let mask = 1 << (7 - x); // leftmost pixel is the highest bit
            let pixel_on = (row & mask) != 0;
            if pixel_on {
                display.set_pixel(x, offset + y as u8, true);
            }
        }
    }
}

const fn letter_bitmap(c: i32) -> [u8; 8] {
    match c {
        0 => [
            0b01111110, 0b00011000, 0b00011000, 0b00011000, 0b00000000, 0b01111110, 0b01100000,
            0b01111100,
        ],
        1 => [
            0b01100000, 0b01111110, 0b00000000, 0b01111110, 0b00011000, 0b00011000, 0b00011000,
            0b01111100,
        ],
        2 => [
            0b01101100, 0b01111110, 0b01101100, 0b01100110, 0b00000000, 0b00011000, 0b00011000,
            0b00011000,
        ],
        3 => [
            0b00011000, 0b00000000, 0b01111110, 0b01100000, 0b01111110, 0b00000110, 0b01111110,
            0b00000000,
        ],
        _ => [0; 8],
    }
}

const fn button_bitmap(b: bool) -> [u8; 8] {
    match b {
        true => [
            0b00000000, 0b00000000, 0b00011000, 0b00011000, 0b01111110, 0b01111110, 0b00000000,
            0b00000000,
        ],
        false => [
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b01111110, 0b01111110, 0b00000000,
            0b00000000,
        ],
    }
}

fn render_in_game(state: &InGameState, display: &mut impl Display) {
    for x in 0..DISPLAY_WIDTH {
        for y in 0..DISPLAY_HEIGHT {
            display.set_pixel(x, y, state.blocks.get(x as i16, y as i16));
        }
    }

    render_piece(&state.current_piece, display);

    // Divider for next piece
    for i in 0..DISPLAY_WIDTH {
        display.set_pixel(i, 7, true);
    }

    if let Some(next_piece) = &state.next_piece {
        render_piece(next_piece, display);
    }
}

fn render_piece(piece: &Piece, display: &mut impl Display) {
    for (x, y) in piece.block_positions() {
        display.set_pixel(wrap_x(x), y as u8, true);
    }
}

pub fn wrap_x(x: i16) -> u8 {
    (if x < 0 { 8 - x.abs() % 8 } else { x % 8 }) as u8
}

fn render_score(score: u32, display: &mut impl Display) {
    display.fill(false);

    for (i, digit) in [
        score / 1000 % 10,
        score / 100 % 10,
        score / 10 % 10,
        score % 10,
    ]
    .iter()
    .enumerate()
    {
        let offset = 8 * i as u8;
        render_digit(*digit, offset, display);
    }
}

fn render_digit(digit: u32, offset: u8, display: &mut impl Display) {
    let bitmap = digit_bitmap(digit);

    for (y, row) in bitmap.iter().enumerate() {
        for x in 0..8 {
            let mask = 1 << (7 - x); // leftmost pixel is the highest bit
            let pixel_on = (row & mask) != 0;
            if pixel_on {
                display.set_pixel(x, offset + y as u8, true);
            }
        }
    }
}

const fn digit_bitmap(digit: u32) -> [u8; 8] {
    match digit {
        0 => [
            0b00111100, 0b01100110, 0b01101110, 0b01110110, 0b01100110, 0b01100110, 0b00111100,
            0b00000000,
        ],
        1 => [
            0b00011000, 0b00111000, 0b00011000, 0b00011000, 0b00011000, 0b00011000, 0b00111100,
            0b00000000,
        ],
        2 => [
            0b00111100, 0b01100110, 0b00000110, 0b00001100, 0b00011000, 0b01100000, 0b01111110,
            0b00000000,
        ],
        3 => [
            0b00111100, 0b01100110, 0b00000110, 0b00011100, 0b00000110, 0b01100110, 0b00111100,
            0b00000000,
        ],
        4 => [
            0b00001100, 0b00011100, 0b00101100, 0b01001100, 0b01111110, 0b00001100, 0b00001100,
            0b00000000,
        ],
        5 => [
            0b01111110, 0b01100000, 0b01111100, 0b00000110, 0b00000110, 0b01100110, 0b00111100,
            0b00000000,
        ],
        6 => [
            0b00111100, 0b01100110, 0b01100000, 0b01111100, 0b01100110, 0b01100110, 0b00111100,
            0b00000000,
        ],
        7 => [
            0b01111110, 0b01100110, 0b00000110, 0b00001100, 0b00011000, 0b00011000, 0b00011000,
            0b00000000,
        ],
        8 => [
            0b00111100, 0b01100110, 0b01100110, 0b00111100, 0b01100110, 0b01100110, 0b00111100,
            0b00000000,
        ],
        9 => [
            0b00111100, 0b01100110, 0b01100110, 0b00111110, 0b00000110, 0b01100110, 0b00111100,
            0b00000000,
        ],
        _ => unreachable!(),
    }
}
