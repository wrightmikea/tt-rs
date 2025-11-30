//! Bundled puzzle loading.

use super::data::Workspace;

/// Bundled puzzle JSON strings (compiled into the binary).
const PUZZLE_FILL_BOX: &str = include_str!("../../assets/puzzles/puzzle-fill-box.json");
const PUZZLE_MAKE_FOUR: &str = include_str!("../../assets/puzzles/puzzle-make-four.json");
const PUZZLE_MAKE_NINE: &str = include_str!("../../assets/puzzles/puzzle-make-nine.json");

/// Load a bundled puzzle by ID.
pub fn load_bundled_puzzle(id: &str) -> Option<Workspace> {
    let json = match id {
        "puzzle-fill-box" => PUZZLE_FILL_BOX,
        "puzzle-make-four" => PUZZLE_MAKE_FOUR,
        "puzzle-make-nine" => PUZZLE_MAKE_NINE,
        _ => return None,
    };

    match serde_json::from_str(json) {
        Ok(workspace) => Some(workspace),
        Err(e) => {
            log::error!("Failed to parse puzzle {}: {}", id, e);
            None
        }
    }
}
