#[cfg(test)]
mod tests {
    use super::super::{Card, Deck, Pile, PileType, Rank, Suit};
    use std::collections::HashSet;

    // ==================== Card Tests ====================

    #[test]
    fn test_card_new() {
        let card = Card::new(Suit::Hearts, Rank::Ace);
        assert_eq!(card.suit, Suit::Hearts);
        assert_eq!(card.rank, Rank::Ace);
        assert!(!card.face_up);
    }

    #[test]
    fn test_card_flip() {
        let mut card = Card::new(Suit::Spades, Rank::King);
        assert!(!card.face_up);

        card.flip();
        assert!(card.face_up);

        card.flip();
        assert!(!card.face_up);
    }

    #[test]
    fn test_card_is_opposite_color() {
        let red_card = Card::new(Suit::Hearts, Rank::Five);
        let black_card = Card::new(Suit::Spades, Rank::Four);
        let another_red = Card::new(Suit::Diamonds, Rank::Three);

        assert!(red_card.is_opposite_color(&black_card));
        assert!(black_card.is_opposite_color(&red_card));
        assert!(!red_card.is_opposite_color(&another_red));
    }

    #[test]
    fn test_card_is_one_rank_lower() {
        let ace = Card::new(Suit::Hearts, Rank::Ace);
        let two = Card::new(Suit::Spades, Rank::Two);
        let three = Card::new(Suit::Clubs, Rank::Three);
        let king = Card::new(Suit::Diamonds, Rank::King);

        assert!(ace.is_one_rank_lower(&two));
        assert!(two.is_one_rank_lower(&three));
        assert!(!three.is_one_rank_lower(&two));
        assert!(!king.is_one_rank_lower(&ace));
    }

    #[test]
    fn test_suit_is_red() {
        assert!(Suit::Hearts.is_red());
        assert!(Suit::Diamonds.is_red());
        assert!(!Suit::Clubs.is_red());
        assert!(!Suit::Spades.is_red());
    }

    #[test]
    fn test_suit_is_black() {
        assert!(Suit::Clubs.is_black());
        assert!(Suit::Spades.is_black());
        assert!(!Suit::Hearts.is_black());
        assert!(!Suit::Diamonds.is_black());
    }

    #[test]
    fn test_rank_value() {
        assert_eq!(Rank::Ace.value(), 1);
        assert_eq!(Rank::Two.value(), 2);
        assert_eq!(Rank::Five.value(), 5);
        assert_eq!(Rank::Ten.value(), 10);
        assert_eq!(Rank::Jack.value(), 11);
        assert_eq!(Rank::Queen.value(), 12);
        assert_eq!(Rank::King.value(), 13);
    }

    #[test]
    fn test_card_display_face_up() {
        let mut card = Card::new(Suit::Hearts, Rank::Ace);
        card.flip();
        assert_eq!(format!("{}", card), "A♥");
    }

    #[test]
    fn test_card_display_face_down() {
        let card = Card::new(Suit::Hearts, Rank::Ace);
        assert_eq!(format!("{}", card), "🂠");
    }

    // ==================== Deck Tests ====================

    #[test]
    fn test_deck_new_has_52_cards() {
        let deck = Deck::new();
        assert_eq!(deck.len(), 52);
        assert!(!deck.is_empty());
    }

    #[test]
    fn test_deck_all_cards_unique() {
        let mut deck = Deck::new();
        let mut seen_cards = HashSet::new();

        for _ in 0..52 {
            if let Some(card) = deck.draw() {
                let card_id = (card.suit as u8, card.rank as u8);
                assert!(
                    seen_cards.insert(card_id),
                    "Duplicate card found: {:?} {:?}",
                    card.suit,
                    card.rank
                );
            }
        }

        assert_eq!(seen_cards.len(), 52, "Should have 52 unique cards");
    }

    #[test]
    fn test_deck_has_all_suits_and_ranks() {
        let mut deck = Deck::new();
        let mut hearts = 0;
        let mut diamonds = 0;
        let mut clubs = 0;
        let mut spades = 0;

        for _ in 0..52 {
            if let Some(card) = deck.draw() {
                match card.suit {
                    Suit::Hearts => hearts += 1,
                    Suit::Diamonds => diamonds += 1,
                    Suit::Clubs => clubs += 1,
                    Suit::Spades => spades += 1,
                }
            }
        }

        assert_eq!(hearts, 13);
        assert_eq!(diamonds, 13);
        assert_eq!(clubs, 13);
        assert_eq!(spades, 13);
    }

    #[test]
    fn test_deck_draw() {
        let mut deck = Deck::new();
        let initial_len = deck.len();

        let card = deck.draw();
        assert!(card.is_some());
        assert_eq!(deck.len(), initial_len - 1);
    }

    #[test]
    fn test_deck_draw_until_empty() {
        let mut deck = Deck::new();

        for i in 0..52 {
            assert_eq!(deck.len(), 52 - i);
            assert!(deck.draw().is_some());
        }

        assert!(deck.is_empty());
        assert_eq!(deck.len(), 0);
        assert!(deck.draw().is_none());
    }

    #[test]
    fn test_deck_draw_multiple() {
        let mut deck = Deck::new();
        let cards = deck.draw_multiple(5);

        assert_eq!(cards.len(), 5);
        assert_eq!(deck.len(), 47);
    }

    #[test]
    fn test_deck_draw_multiple_more_than_available() {
        let mut deck = Deck::new();
        let cards = deck.draw_multiple(60);

        assert_eq!(cards.len(), 52);
        assert!(deck.is_empty());
    }

    #[test]
    fn test_deck_shuffle_changes_order() {
        let mut deck1 = Deck::new();
        let mut deck2 = Deck::new();

        deck2.shuffle();

        let mut cards1 = Vec::new();
        let mut cards2 = Vec::new();

        for _ in 0..52 {
            cards1.push(deck1.draw().unwrap());
            cards2.push(deck2.draw().unwrap());
        }

        let same_count = cards1
            .iter()
            .zip(cards2.iter())
            .filter(|(c1, c2)| c1 == c2)
            .count();

        assert!(
            same_count < 52,
            "Shuffled deck should have different order"
        );
    }

    #[test]
    fn test_deck_shuffle_preserves_all_cards() {
        let mut deck = Deck::new();
        deck.shuffle();

        let mut seen_cards = HashSet::new();
        for _ in 0..52 {
            if let Some(card) = deck.draw() {
                seen_cards.insert((card.suit as u8, card.rank as u8));
            }
        }

        assert_eq!(seen_cards.len(), 52, "Shuffle should preserve all cards");
    }

    #[test]
    fn test_deck_default() {
        let deck = Deck::default();
        assert_eq!(deck.len(), 52);
    }

    // ==================== Pile Tests ====================

    #[test]
    fn test_pile_new() {
        let pile = Pile::new(PileType::Tableau);
        assert_eq!(pile.pile_type, PileType::Tableau);
        assert!(pile.is_empty());
        assert_eq!(pile.len(), 0);
    }

    #[test]
    fn test_pile_push_and_pop() {
        let mut pile = Pile::new(PileType::Waste);
        let card = Card::new(Suit::Hearts, Rank::Ace);

        pile.push(card);
        assert_eq!(pile.len(), 1);
        assert!(!pile.is_empty());

        let popped = pile.pop();
        assert!(popped.is_some());
        assert_eq!(popped.unwrap(), card);
        assert!(pile.is_empty());
    }

    #[test]
    fn test_pile_peek() {
        let mut pile = Pile::new(PileType::Tableau);
        assert!(pile.peek().is_none());

        let card1 = Card::new(Suit::Hearts, Rank::Five);
        let card2 = Card::new(Suit::Spades, Rank::Four);

        pile.push(card1);
        assert_eq!(pile.peek(), Some(&card1));

        pile.push(card2);
        assert_eq!(pile.peek(), Some(&card2));
    }

    #[test]
    fn test_pile_tableau_can_place_king_on_empty() {
        let pile = Pile::new(PileType::Tableau);
        let king = Card::new(Suit::Hearts, Rank::King);
        let queen = Card::new(Suit::Spades, Rank::Queen);

        assert!(pile.can_place_card(&king));
        assert!(!pile.can_place_card(&queen));
    }

    #[test]
    fn test_pile_tableau_requires_opposite_color() {
        let mut pile = Pile::new(PileType::Tableau);
        let red_five = Card::new(Suit::Hearts, Rank::Five);
        let black_four = Card::new(Suit::Spades, Rank::Four);
        let red_four = Card::new(Suit::Diamonds, Rank::Four);

        pile.push(red_five);

        assert!(pile.can_place_card(&black_four));
        assert!(!pile.can_place_card(&red_four));
    }

    #[test]
    fn test_pile_tableau_requires_one_rank_lower() {
        let mut pile = Pile::new(PileType::Tableau);
        let red_five = Card::new(Suit::Hearts, Rank::Five);
        let black_four = Card::new(Suit::Spades, Rank::Four);
        let black_three = Card::new(Suit::Clubs, Rank::Three);

        pile.push(red_five);

        assert!(pile.can_place_card(&black_four));
        assert!(!pile.can_place_card(&black_three));
    }

    #[test]
    fn test_pile_foundation_requires_ace_on_empty() {
        let pile = Pile::new(PileType::Foundation);
        let ace = Card::new(Suit::Hearts, Rank::Ace);
        let two = Card::new(Suit::Hearts, Rank::Two);

        assert!(pile.can_place_card(&ace));
        assert!(!pile.can_place_card(&two));
    }

    #[test]
    fn test_pile_foundation_requires_same_suit() {
        let mut pile = Pile::new(PileType::Foundation);
        let hearts_ace = Card::new(Suit::Hearts, Rank::Ace);
        let hearts_two = Card::new(Suit::Hearts, Rank::Two);
        let spades_two = Card::new(Suit::Spades, Rank::Two);

        pile.push(hearts_ace);

        assert!(pile.can_place_card(&hearts_two));
        assert!(!pile.can_place_card(&spades_two));
    }

    #[test]
    fn test_pile_foundation_requires_ascending_rank() {
        let mut pile = Pile::new(PileType::Foundation);
        pile.push(Card::new(Suit::Hearts, Rank::Ace));
        pile.push(Card::new(Suit::Hearts, Rank::Two));

        let three = Card::new(Suit::Hearts, Rank::Three);
        let four = Card::new(Suit::Hearts, Rank::Four);

        assert!(pile.can_place_card(&three));
        assert!(!pile.can_place_card(&four));
    }

    #[test]
    fn test_pile_stock_and_waste_cannot_place() {
        let stock = Pile::new(PileType::Stock);
        let waste = Pile::new(PileType::Waste);
        let card = Card::new(Suit::Hearts, Rank::Ace);

        assert!(!stock.can_place_card(&card));
        assert!(!waste.can_place_card(&card));
    }

    #[test]
    fn test_pile_take_cards_from() {
        let mut pile = Pile::new(PileType::Tableau);
        pile.push(Card::new(Suit::Hearts, Rank::King));
        pile.push(Card::new(Suit::Spades, Rank::Queen));
        pile.push(Card::new(Suit::Hearts, Rank::Jack));
        pile.push(Card::new(Suit::Spades, Rank::Ten));

        let taken = pile.take_cards_from(2);

        assert_eq!(taken.len(), 2);
        assert_eq!(pile.len(), 2);
        assert_eq!(taken[0].rank, Rank::Jack);
        assert_eq!(taken[1].rank, Rank::Ten);
    }

    #[test]
    fn test_pile_take_cards_from_invalid_index() {
        let mut pile = Pile::new(PileType::Tableau);
        pile.push(Card::new(Suit::Hearts, Rank::King));

        let taken = pile.take_cards_from(10);
        assert_eq!(taken.len(), 0);
        assert_eq!(pile.len(), 1);
    }

    #[test]
    fn test_pile_add_cards() {
        let mut pile = Pile::new(PileType::Tableau);
        pile.push(Card::new(Suit::Hearts, Rank::King));

        let cards = vec![
            Card::new(Suit::Spades, Rank::Queen),
            Card::new(Suit::Hearts, Rank::Jack),
        ];

        pile.add_cards(cards);
        assert_eq!(pile.len(), 3);
    }

    #[test]
    fn test_pile_flip_top_card() {
        let mut pile = Pile::new(PileType::Tableau);
        let card = Card::new(Suit::Hearts, Rank::King);
        assert!(!card.face_up);

        pile.push(card);
        pile.flip_top_card();

        let top = pile.peek().unwrap();
        assert!(top.face_up);
    }

    #[test]
    fn test_pile_flip_top_card_on_empty() {
        let mut pile = Pile::new(PileType::Tableau);
        pile.flip_top_card();
        assert!(pile.is_empty());
    }

    #[test]
    fn test_pile_types() {
        let tableau = Pile::new(PileType::Tableau);
        let foundation = Pile::new(PileType::Foundation);
        let stock = Pile::new(PileType::Stock);
        let waste = Pile::new(PileType::Waste);

        assert_eq!(tableau.pile_type, PileType::Tableau);
        assert_eq!(foundation.pile_type, PileType::Foundation);
        assert_eq!(stock.pile_type, PileType::Stock);
        assert_eq!(waste.pile_type, PileType::Waste);
    }
}
