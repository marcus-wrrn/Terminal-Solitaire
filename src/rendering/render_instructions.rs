use crate::game_logic::HoverState;
use crate::game_objects::{Board, Selection};
use crate::ui::DebugLog;



/// Contains information on how to best render the board
pub struct RenderingInstructions<'a> {
    pub board_rendering_instr: BoardRenderingIntr<'a>,
    pub debug_log: &'a DebugLog,
}

impl<'a> RenderingInstructions<'a> {
    pub fn new(
        board: &'a Board, 
        selection: &'a Selection, 
        selection_visible: bool, 
        hover_state: &'a HoverState, 
        debug_log: &'a DebugLog
    ) -> Self {
        Self {
            board_rendering_instr: BoardRenderingIntr { 
                board, 
                selection, 
                selection_visible, 
                hover_state 
            },
            debug_log: debug_log
        }
    }
}

pub struct BoardRenderingIntr<'a> {
    pub board: &'a Board,
    pub selection: &'a Selection,
    pub selection_visible: bool,
    pub hover_state: &'a HoverState
}

