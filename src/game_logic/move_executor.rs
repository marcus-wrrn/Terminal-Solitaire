use crate::game_objects::Board;

pub struct MoveExecutor;

impl MoveExecutor {
    pub fn draw_from_stock(board: &mut Board) -> bool {
        if let Some(mut card) = board.stock.pop() {
            card.face_up = true;
            board.waste.add_card(card);
            true
        } else {
            false
        }
    }

    pub fn reset_stock(board: &mut Board) {
        while let Some(mut card) = board.waste.pop() {
            card.face_up = false;
            board.stock.add_card(card);
        }
    }

    pub fn move_card_to_tableau(board: &mut Board, from_pile: usize, to_pile: usize, card_index: usize) -> Result<(), &'static str> {
        if from_pile >= 7 || to_pile >= 7 {
            return Err("Invalid pile index");
        }

        let cards_to_move = if let Some(pile) = board.tableau.get(from_pile) {
            if card_index >= pile.len() {
                return Err("Invalid card index");
            }
            pile.cards[card_index..].to_vec()
        } else {
            return Err("Invalid from pile");
        };

        if let Some(first_card) = cards_to_move.first() {
            if !board.tableau[to_pile].can_place_card(first_card) {
                return Err("Cannot place card on target pile");
            }
        }

        let cards = board.tableau[from_pile].take_cards_from(card_index);
        board.tableau[to_pile].add_cards(cards);

        board.tableau[from_pile].flip_top_card();

        Ok(())
    }

    pub fn move_card_to_foundation(board: &mut Board, from_tableau: usize, foundation_index: usize) -> Result<(), &'static str> {
        if from_tableau >= 7 || foundation_index >= 4 {
            return Err("Invalid pile index");
        }

        let card = board.tableau[from_tableau].peek()
            .ok_or("No card to move")?
            .clone();

        if !board.foundation[foundation_index].can_place_card(&card) {
            return Err("Cannot place card on foundation");
        }

        if let Some(card) = board.tableau[from_tableau].pop() {
            board.foundation[foundation_index].add_card(card);
            board.tableau[from_tableau].flip_top_card();
            Ok(())
        } else {
            Err("Failed to move card")
        }
    }

    pub fn move_waste_to_tableau(board: &mut Board, tableau_index: usize) -> Result<(), &'static str> {
        if tableau_index >= 7 {
            return Err("Invalid tableau index");
        }

        let card = board.waste.peek()
            .ok_or("No card in waste")?
            .clone();

        if !board.tableau[tableau_index].can_place_card(&card) {
            return Err("Cannot place card on tableau");
        }

        if let Some(card) = board.waste.pop() {
            board.tableau[tableau_index].add_card(card);
            Ok(())
        } else {
            Err("Failed to move card")
        }
    }

    pub fn move_waste_to_foundation(board: &mut Board, foundation_index: usize) -> Result<(), &'static str> {
        if foundation_index >= 4 {
            return Err("Invalid foundation index");
        }

        let card = board.waste.peek()
            .ok_or("No card in waste")?
            .clone();

        if !board.foundation[foundation_index].can_place_card(&card) {
            return Err("Cannot place card on foundation");
        }

        if let Some(card) = board.waste.pop() {
            board.foundation[foundation_index].add_card(card);
            Ok(())
        } else {
            Err("Failed to move card")
        }
    }

    pub fn move_stock_to_foundation(board: &mut Board, foundation_index: usize) -> Result<(), &'static str> {
        if foundation_index >= 4 {
            return Err("Invalid foundation index");
        }

        for i in 0..board.stock.cards.len() {
            let mut card = board.stock.cards[i].clone();
            card.face_up = true;

            if board.foundation[foundation_index].can_place_card(&card) {
                board.stock.cards.remove(i);
                board.foundation[foundation_index].add_card(card);
                return Ok(());
            }
        }

        Err("No valid card in stock for this foundation")
    }

    /// Used during the win state to move any card from the waste pile to the foundation
    /// Should not be called during normal gameplay
    pub fn move_waste_card_to_foundation(board: &mut Board, foundation_index: usize) -> Result<(), &'static str> {
        if foundation_index >= 4 {
            return Err("Invalid foundation index");
        }

        for i in 0..board.waste.cards.len() {
            let card = &board.waste.cards[i];

            if board.foundation[foundation_index].can_place_card(card) {
                let card = board.waste.cards.remove(i);
                board.foundation[foundation_index].add_card(card);
                return Ok(());
            }
        }

        Err("No valid card in waste for this foundation")
    }

    pub fn move_foundation_to_tableau(board: &mut Board, foundation_index: usize, tableau_index: usize) -> Result<(), &'static str> {
        if foundation_index >= 4 || tableau_index >= 7 {
            return Err("Invalid pile index");
        }

        let card = board.foundation[foundation_index].peek()
            .ok_or("No card in foundation")?
            .clone();

        if !board.tableau[tableau_index].can_place_card(&card) {
            return Err("Cannot place card on tableau");
        }

        if let Some(card) = board.foundation[foundation_index].pop() {
            board.tableau[tableau_index].add_card(card);
            Ok(())
        } else {
            Err("Failed to move card")
        }
    }
}
