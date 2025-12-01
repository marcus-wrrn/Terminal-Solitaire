use crate::game_objects::Board;
use std::time::{Duration, Instant};

pub struct AnimationManager {
    animation_active: bool,
    last_move_time: Option<Instant>,
    animation_interval: Duration,
}

impl AnimationManager {
    pub fn new() -> Self {
        Self {
            animation_active: false,
            last_move_time: None,
            animation_interval: Duration::from_millis(100),
        }
    }

    pub fn is_active(&self) -> bool {
        self.animation_active
    }

    pub fn start_animation(&mut self) {
        self.animation_active = true;
        self.last_move_time = Some(Instant::now());
    }

    pub fn stop_animation(&mut self) {
        self.animation_active = false;
        self.last_move_time = None;
    }

    pub fn win_animation(&mut self, board: &mut Board) -> Result<(), &'static str> {
        if !self.animation_active {
            return Err("Animation is not active");
        }

        if let Some(last_time) = self.last_move_time {
            if last_time.elapsed() < self.animation_interval {
                return Err("Animation interval not elapsed");
            }
        }

        match self.try_move_next_card(board) {
            Ok(()) => {
                self.last_move_time = Some(Instant::now());
                Ok(())
            }
            Err(e) => {
                self.stop_animation();
                Err(e)
            }
        }
    }

    fn try_move_next_card(&self, board: &mut Board) -> Result<(), &'static str> {
        // Try moving from waste to any foundation
        if board.waste.peek().is_some() {
            for foundation_index in 0..4 {
                if let Ok(()) = board.move_waste_to_foundation(foundation_index) {
                    return Ok(());
                }
            }
        }

        // Try moving from each tableau pile to any foundation
        for tableau_index in 0..7 {
            if board.tableau[tableau_index].peek().is_some() {
                for foundation_index in 0..4 {
                    if let Ok(()) = board.move_card_to_foundation(tableau_index, foundation_index) {
                        return Ok(());
                    }
                }
            }
        }

        // Try moving from stock to any foundation
        if !board.stock.is_empty() {
            for foundation_index in 0..4 {
                if let Ok(()) = board.move_stock_to_foundation(foundation_index) {
                    return Ok(());
                }
            }
        }

        Err("No valid moves available")
    }
}

impl Default for AnimationManager {
    fn default() -> Self {
        Self::new()
    }
}
