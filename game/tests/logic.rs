use game::DISPLAY_HEIGHT;
use game::logic::piece::{Piece, PieceKind, Rotation};
use game::logic::{Blocks, ButtonAction, GameState, InStartState, StartMenuPhase};
use std::collections::HashSet;
use std::time::Instant;

#[test]
fn test_piece_rotation() {
    let mut piece = Piece::new(0, 0, PieceKind::T);
    assert_eq!(piece.rotation.to_u16(), 0);

    piece.rotate(Rotation::Deg90);
    assert_eq!(piece.rotation.to_u16(), 90);

    piece.rotate(Rotation::Deg270);
    assert_eq!(piece.rotation.to_u16(), 0);
}

#[test]
fn test_piece_movement() {
    let mut piece = Piece::new(3, 3, PieceKind::O);

    piece.move_by(1, -1);
    assert_eq!(piece.x, 4);
    assert_eq!(piece.y, 2);

    piece.move_to(0, 0);
    assert_eq!(piece.x, 0);
    assert_eq!(piece.y, 0);
}

#[test]
fn test_block_positions_for_t_shape() {
    let mut piece = Piece::new(3, 2, PieceKind::T);
    let expected_pos_deg0: HashSet<(i16, i16)> =
        [(3, 2), (3, 3), (4, 3), (3, 4)].iter().cloned().collect();

    let positions_deg0: HashSet<(i16, i16)> = piece.block_positions().collect();
    assert_eq!(positions_deg0, expected_pos_deg0);

    piece.rotate(Rotation::Deg90);
    let expected_pos_deg90: HashSet<(i16, i16)> =
        [(2, 3), (3, 3), (3, 2), (4, 3)].iter().cloned().collect();

    let positions_deg90: HashSet<(i16, i16)> = piece.block_positions().collect();
    assert_eq!(positions_deg90, expected_pos_deg90);
}

#[test]
fn test_blocks_set_and_get() {
    let mut blocks = Blocks {
        data: [0; DISPLAY_HEIGHT as usize],
    };

    blocks.set(2, 5);

    assert_eq!(blocks.get(2, 5), true);
    assert_eq!(blocks.get(0, 0), false);
}

#[test]
fn test_blocks_place_piece() {
    let mut blocks = Blocks {
        data: [0; DISPLAY_HEIGHT as usize],
    };
    let piece = Piece::new(0, 0, PieceKind::O);

    blocks.place_piece(&piece);

    assert!(blocks.get(0, 0));
    assert!(blocks.get(1, 0));
    assert!(blocks.get(0, 1));
    assert!(blocks.get(1, 1));
}

#[test]
fn test_remove_one_full_row() {
    let mut blocks = Blocks {
        data: [0; DISPLAY_HEIGHT as usize],
    };
    blocks.data[31] = 0xff;
    blocks.data[30] = 0b00011000;

    let removed_count = blocks.remove_full_rows();

    // Assert
    assert_eq!(removed_count, 1);
    assert_eq!(blocks.data[31], 0b00011000);
    assert_eq!(blocks.data[0], 0x00);
}
#[test]
fn test_gamestate_start_to_ingame() {
    let start_state = GameState::StartMenu(InStartState {
        phase: StartMenuPhase::Text,
        last_update: Instant::now(),
    });
    let new_state = start_state.update(Some(ButtonAction::Rotate), Instant::now(), |_| {});

    assert!(matches!(new_state, GameState::InGame(_)));
}
