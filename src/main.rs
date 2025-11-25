mod card;
mod deck;
mod pile;

use card::{Card, Rank, Suit};
use deck::Deck;
use pile::{Pile, PileType};

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();

    println!("Created and shuffled a deck of {} cards", deck.len());

    let mut tableau_pile = Pile::new(PileType::Tableau);
    let mut foundation_pile = Pile::new(PileType::Foundation);
    let mut stock_pile = Pile::new(PileType::Stock);
    let mut waste_pile = Pile::new(PileType::Waste);

    let drawn_cards = deck.draw_multiple(5);
    for card in drawn_cards {
        stock_pile.push(card);
    }

    println!("Stock pile has {} cards", stock_pile.len());
    println!("Backend card and pile logic initialized successfully!");
}
