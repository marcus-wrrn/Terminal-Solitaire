use crate::game_objects::Selection;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HoverState {
    Valid(Selection),
    Invalid(Selection),
    None,
}
