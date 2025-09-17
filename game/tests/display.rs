use game::display::render::wrap_x;
use game::display::{Display, TextDisplay};
use game::logic::piece::{Piece, PieceKind};
use game::logic::{Blocks, InGameState};

#[cfg(test)]
mod text_display_tests {
    use super::*;
    use game::DISPLAY_HEIGHT;

    #[test]
    fn test_new_is_empty() {
        let display = TextDisplay::new();
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..8 {
                assert!(!display.get_pixel(x, y));
            }
        }
    }

    #[test]
    fn test_set_and_get_pixel() {
        let mut display = TextDisplay::new();

        display.set_pixel(3, 5, true);
        assert!(display.get_pixel(3, 5));

        display.set_pixel(3, 5, false);
        assert!(!display.get_pixel(3, 5));
    }

    #[test]
    fn test_fill() {
        let mut display = TextDisplay::new();

        display.fill(true);
        assert!(display.get_pixel(0, 0));
        assert!(display.get_pixel(7, 31));

        display.fill(false);
        assert!(!display.get_pixel(0, 0));
        assert!(!display.get_pixel(7, 31));
    }
}

#[cfg(test)]
mod render_logic_tests {
    use super::*;

    #[test]
    fn test_wrap_x_logic() {
        assert_eq!(wrap_x(0), 0);
        assert_eq!(wrap_x(7), 7);
        assert_eq!(wrap_x(8), 0, "Sollte bei 8 auf 0 umbrechen");
        assert_eq!(wrap_x(-1), 7, "Sollte bei -1 auf 7 umbrechen");
        assert_eq!(wrap_x(-9), 7, "Sollte bei -9 auf 7 umbrechen");
    }

    #[test]
    fn test_render_score() {
        let mut display = TextDisplay::new();
        let score = 1234;

        let mut game_over_state = game::logic::GameState::GameOver(score);
        game::display::render::render(&mut game_over_state, &mut display);

        assert!(display.get_pixel(3, 0));
        assert!(display.get_pixel(2, 8));
        assert!(display.get_pixel(2, 16));
        assert!(display.get_pixel(4, 24));
    }

    #[test]
    fn test_render_in_game_state() {
        let mut display = TextDisplay::new();
        let blocks = Blocks {
            data: [0; game::DISPLAY_HEIGHT as usize],
        };

        let current_piece = Piece::new(3, 10, PieceKind::T);
        let next_piece = Some(Piece::new(2, 2, PieceKind::O));

        let game_state_data = InGameState {
            blocks,
            score: 0,
            current_piece,
            next_piece,
            time_last_move: std::time::Instant::now(),
        };
        let mut game_state = game::logic::GameState::InGame(game_state_data);

        game::display::render::render(&mut game_state, &mut display);

        assert!(display.get_pixel(0, 31));
        assert!(display.get_pixel(0, 7));
        assert!(display.get_pixel(7, 7));
        assert!(display.get_pixel(3, 10));
        assert!(display.get_pixel(2, 2));
    }
}
