use crate::game_objects::CardRow;
use crate::ui::DebugLog;

pub struct BalatermRenderingInstructions<'a> {
    pub title: &'a str,
    pub hands: &'a [CardRow],
    pub selected_hand: Option<usize>,
    pub selected_slot: Option<usize>,
    pub debug_log: &'a DebugLog,
}
