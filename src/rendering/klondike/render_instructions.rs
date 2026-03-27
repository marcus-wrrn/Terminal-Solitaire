use crate::game_objects::{Board, Selection, HoverState};
use crate::ui::DebugLog;

/// Contains information on how to best render the board
pub struct RenderingInstructions<'a> {
    pub title: &'a str,
    pub board_rendering_instr: BoardRenderingIntr<'a>,
    pub debug_log: &'a DebugLog,
}

impl<'a> RenderingInstructions<'a> {
    pub fn new(
        title: &'a str,
        board: &'a Board,
        selection: Option<&'a Selection>,
        picked_up: Option<&'a Selection>,
        hover_state: &'a HoverState,
        debug_log: &'a DebugLog
    ) -> Self {
        Self {
            title,
            board_rendering_instr: BoardRenderingIntr {
                board,
                selection,
                picked_up,
                hover_state
            },
            debug_log
        }
    }
}

pub struct BoardRenderingIntr<'a> {
    pub board: &'a Board,
    pub selection: Option<&'a Selection>,
    pub picked_up: Option<&'a Selection>,
    pub hover_state: &'a HoverState
}

